from collections import defaultdict
from dataclasses import dataclass
import logging
from pathlib import PurePath
from graphlib import TopologicalSorter
import re
from typing import Iterable, Optional

from semver import Version

from release_tulpje.cargo_semver import cargo_semver_checks
from release_tulpje.constants import (
    RELEASE_FILENAME_MATCHLIST,
    RELEASE_FILENAME_MATCHLIST_WORKSPACE,
)
from release_tulpje.git import (
    CommitInfo,
    filter_commits_by_path,
    get_commits_since_ref,
    get_latest_tag,
)
from release_tulpje.crates import CrateInfo
from release_tulpje.formatter import colored_bool
from release_tulpje.util import filename_match, find_file_upwards, process_run
from release_tulpje.version import prefixed_version, version_bump

log = logging.getLogger(__name__)


@dataclass
class ReleaseInfo:
    crates: tuple[CrateInfo, ...]

    prev_version: Version
    curr_version: Version

    commits: tuple[CommitInfo, ...]
    changelog: str
    should_release: bool

    has_feature_commit: bool
    has_breaking_commit: bool
    is_breaking_semver_checks: bool

    def breaking_with_reason(self) -> str:
        if not self.is_breaking:
            return colored_bool(False)
        elif self.has_breaking_commit:
            return f"{colored_bool(True)} (Breaking Commit)"
        elif self.is_breaking_semver_checks:
            return f"{colored_bool(True)} (`cargo semver-checks` failed)"

        raise Exception(
            "did we add a new conditional for breaking changes, shouldn't reach this"
        )

    @property
    def changed(self) -> bool:
        return self.prev_version != self.curr_version

    @property
    def single_crate(self) -> bool:
        return len(self.crates) == 1

    @property
    def prefix(self) -> str:
        if self.single_crate:
            return f"{self.crates[0].name.removeprefix('tulpje-')}-"
        else:
            return ""

    @property
    def is_breaking(self) -> bool:
        return self.has_breaking_commit or self.is_breaking_semver_checks

    @property
    def dir(self) -> PurePath:
        if self.single_crate:
            return self.crates[0].path

        workspace_manifest = find_file_upwards(self.crates[0].path.parent, "Cargo.toml")
        if workspace_manifest is None:
            raise Exception(
                f"Couldn't find workspace root for path {self.crates[0].path}"
            )

        return workspace_manifest

    @property
    def prev_tag(self) -> str:
        return f"{self.prefix}v{self.prev_version}"

    @property
    def curr_tag(self) -> str:
        return f"{self.prefix}v{self.curr_version}"

    @property
    def commit_count(self) -> int:
        return len(self.commits)

    @property
    def changelog_path(self) -> str:
        return f"{self.dir}/CHANGELOG.md" if self.single_crate else "CHANGELOG.md"

    @property
    def tag_changelog(self) -> str:
        return re.sub(
            r"\[.*?\]\((.*?)\)",
            r"\1",
            "\n".join(
                self.changelog.replace(
                    "\n<details><summary>view details</summary>\n", ""
                )
                .replace("</details>", "")
                .splitlines()[2:]
            ).strip(),
        )


def process_dependencies(releases_by_deps: list[ReleaseInfo]) -> list[ReleaseInfo]:
    releases_copy = [ReleaseInfo(**release.__dict__) for release in releases_by_deps]

    # crate.name -> ReleaseInfo
    releases_by_crate: dict[str, ReleaseInfo] = {}
    for release in releases_copy:
        for crate in release.crates:
            releases_by_crate[crate.name] = release

    # reverse dependency lookup
    depended_on_by: dict[str, set[CrateInfo]] = defaultdict(set)
    for release in releases_by_deps:
        for crate in release.crates:
            for dependency in crate.workspace_dependencies:
                depended_on_by[dependency].add(crate)

    # iterate through dependencies and update versions where required
    for release in releases_copy:
        if release.should_release:
            for crate in release.crates:
                for depended_crate in depended_on_by[crate.name]:
                    crate_release = releases_by_crate[depended_crate.name]
                    crate_release.should_release = True
                    if not crate_release.changed:
                        crate_release.curr_version = (
                            crate_release.curr_version.next_version("patch")
                        )
    return [release for release in releases_copy if release.should_release]


def gather_release(
    crates: list[CrateInfo],
    independent_crates: list[CrateInfo],
    prerelease: Optional[str],
    skip_slow: bool,
) -> ReleaseInfo:
    independent_crate = len(crates) == 1
    tag_prefix = "" if not independent_crate else crates[0].name.removeprefix("tulpje-")

    if independent_crate:
        log.info("gathering release for {} ...".format(crates[0].name))
        file_whitelist = RELEASE_FILENAME_MATCHLIST
    else:
        log.info("gathering release for main crate ...")
        file_whitelist = RELEASE_FILENAME_MATCHLIST.union(
            RELEASE_FILENAME_MATCHLIST_WORKSPACE
        ).union({f"!{crate.path}/**/*" for crate in independent_crates})

    latest_tag = get_latest_tag(f"{tag_prefix}-" if len(tag_prefix) > 0 else "", True)
    has_independent_tag = independent_crate and latest_tag is not None

    # fall back to main tag if there's none for the prefix yet
    if independent_crate and latest_tag is None:
        latest_tag = get_latest_tag()

    if latest_tag is None:
        raise Exception("ERR: couldn't find the previous tag")

    log.debug(f"latest tag: {latest_tag}, has_independent_tag: {has_independent_tag}")

    if independent_crate:
        # gather commits belonging to the independent crate
        commits = filter_commits_by_path(
            get_commits_since_ref(latest_tag), [crates[0].path]
        )
    else:
        # if this is part of the main release, filter any commits that belong
        # to independent crates
        commits = filter_commits_by_path(
            get_commits_since_ref(latest_tag),
            [crate.path for crate in independent_crates],
            True,
        )

    has_feature_commit = any(commit.typ == "feat" for commit in commits)
    has_breaking_change_commit = any(commit.breaking for commit in commits)
    has_breaking_change_semver_checks = (
        cargo_semver_checks(latest_tag, crates[0].name if independent_crate else None)[
            0
        ]
        if not skip_slow
        else False
    )

    if independent_crate:
        old_version = crates[0].version
    else:
        old_version = Version.parse(
            latest_tag.removeprefix(tag_prefix).removeprefix("v")
        )

    # release if there's changes or we're going from prerelease -> regular release
    should_release = (
        should_create_release(
            commits, file_whitelist, crates[0].path if independent_crate else None
        )
        or old_version.prerelease is not None
        and prerelease is None
    )

    if independent_crate and not has_independent_tag:
        new_version = old_version
    else:
        if prerelease is not None and old_version.prerelease is not None:
            new_version = old_version.bump_prerelease(prerelease)
        else:
            new_version = version_bump(
                old_version,
                has_feature_commit,
                has_breaking_change_commit or has_breaking_change_semver_checks,
                prerelease,
            )

    new_changelog = (
        create_changelog_update(
            tag_prefix,
            old_version,
            new_version,
            crates[0] if independent_crate else None,
            independent_crates,
        )
        if not skip_slow
        else ""
    )

    return ReleaseInfo(
        crates=tuple(crates),
        commits=tuple(commits),
        prev_version=old_version,
        curr_version=new_version,
        changelog=new_changelog,
        should_release=should_release,
        has_feature_commit=has_feature_commit,
        has_breaking_commit=has_breaking_change_commit,
        is_breaking_semver_checks=has_breaking_change_semver_checks,
    )


def should_create_release(
    commits: list[CommitInfo], matchlist: set[str], path: Optional[PurePath]
) -> bool:
    changed_files = {
        file
        for commit in commits
        for file in commit.files
        if path is None or path in file.parents
    }
    return any(filename_match(file, matchlist) for file in changed_files)


def create_changelog_update(
    tag_prefix: str,
    old_version: Version,
    new_version: Version,
    independent_crate: Optional[CrateInfo],
    independent_crates: Iterable[CrateInfo],
) -> str:
    log.info(
        "creating changelog update"
        + (f" for {tag_prefix}" if len(tag_prefix) > 0 else "")
    )
    if independent_crate:
        extra_args = ["--include-path", f"{independent_crate.path}/**/*"]
    else:
        extra_args = sum(
            [["--exclude-path", f"{crate.path}/**/*"] for crate in independent_crates],
            [],
        )

    command = (
        ["git-cliff"]
        + extra_args
        + [
            "--strip",
            "all",
            "--tag",
            str(new_version),
            f"{prefixed_version(tag_prefix, old_version)}..HEAD",
        ]
    )
    return process_run(command, encoding="utf-8").strip()


def sort_releases_by_deps(releases: list[ReleaseInfo]) -> list[ReleaseInfo]:
    # crate.name -> ReleaseInfo
    releases_by_crate: dict[str, ReleaseInfo] = {}
    for release in releases:
        for crate in release.crates:
            releases_by_crate[crate.name] = release

    # create an order of which dependencies to evaluate first
    dependency_order = [
        *TopologicalSorter(
            {
                crate.name: crate.workspace_dependencies
                for release in releases
                for crate in release.crates
            }
        ).static_order()
    ]
    releases_by_deps: list[ReleaseInfo] = []
    for crate in dependency_order:
        release = releases_by_crate[crate]
        if release not in releases_by_deps:
            releases_by_deps.append(releases_by_crate[crate])

    return releases_by_deps
