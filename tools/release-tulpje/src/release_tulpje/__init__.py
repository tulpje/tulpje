from collections import defaultdict
from pathlib import PurePath
import subprocess
import argparse
import sys
import logging

from termcolor import colored, cprint

from release_tulpje.crates import (
    CrateInfo,
    gather_crates,
    manifest_bump_version,
    workspace_bump_version,
    workspace_update_dependency,
)
from release_tulpje.formatter import RustishFormatter, colored_bool
from release_tulpje.releases import (
    ReleaseInfo,
    gather_release,
    process_dependencies,
    sort_releases_by_deps,
)
from release_tulpje.util import check_output_dry, dry_print

log = logging.getLogger(__name__)


def argparser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--skip-slow",
        action="store_true",
        help="Skip `cargo semver-checks` and `git-cliff` to speed up runs",
    )
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--prerelease")

    return parser


# TODO: Enforce independent crates not depending on workspace crates
def do_releases(
    releases_by_deps: list[ReleaseInfo], current_branch: str, execute=False
):
    if len(releases_by_deps) == 0:
        print(" [*] Nothing to release")
        return

    for release in releases_by_deps:
        cprint(
            f"Crate: {release.crates[0].name if release.single_crate else 'root'}",
            attrs=["bold"],
        )
        print(f" [*] Should release: {colored_bool(release.should_release)}")
        print(
            f" [*] Version: {colored(release.prev_version, 'red')} -> {colored(release.curr_version, 'green')}"
        )
        print(
            f" [*] Tag: {colored(release.prev_tag, 'red')} -> {colored(release.curr_tag, 'green')}"
        )
        print(f" [*] Commits since: {colored(release.commit_count, 'light_grey')}")
        print(f" [*] Feature: {colored_bool(release.has_feature_commit)}")
        print(f" [*] Breaking: {release.breaking_with_reason()}")
        print(f" [*] Commits ({release.commit_count}):")
        for commit in release.commits:
            cprint(f"    - {commit.raw_subject} ({commit.sha[:8]})", "grey")

    # reverse dependency lookup
    depended_on_by: dict[str, set[CrateInfo]] = defaultdict(set)
    for release in releases_by_deps:
        for crate in release.crates:
            for dependency in crate.workspace_dependencies:
                depended_on_by[dependency].add(crate)

    filtered_releases = [
        release for release in releases_by_deps if release.should_release
    ]

    dry_print(execute, " [-] Bumping versions...")
    for release in filtered_releases:
        name = release.crates[0].name if release.single_crate else "tulpje"
        print(
            f"     - {name}: {colored(release.prev_version, 'red')} -> {colored(release.curr_version, 'green')}"
        )

        if execute:
            if release.single_crate:
                manifest_bump_version(release.crates[0], release.curr_version)
            else:
                workspace_bump_version(
                    PurePath("./Cargo.toml"), release.crates, release.curr_version
                )

            for crate in release.crates:
                for depends_on in depended_on_by[crate.name]:
                    workspace_update_dependency(depends_on, crate, release.curr_version)

    dry_print(execute, " [-] Writing changelogs...")
    if execute:
        for release in filtered_releases:
            with open(release.changelog_path) as changelog_file:
                new_changelog = changelog_file.read().replace(
                    "##", f"{release.changelog}\n\n##", 1
                )

            with open(release.changelog_path, "w") as new_changelog_file:
                new_changelog_file.write(new_changelog)

    check_output_dry(
        " [-] Committing changes...",
        execute,
        [
            "git",
            "add",
            "CHANGELOG.md",
            "*/CHANGELOG.md",
            "Cargo.lock",
            "Cargo.toml",
            "*/Cargo.toml",
        ],
    )

    commit_message = "release: " + (
        ", ".join(
            f"{release.crates[0].name if release.single_crate else 'tulpje'} v{release.curr_version}"
            for release in reversed(filtered_releases)
        )
    )
    check_output_dry(
        None,
        execute,
        ["git", "commit", "--cleanup=verbatim", "--message", commit_message],
    )

    print(" [-] Tagging release...")
    for release in filtered_releases:
        check_output_dry(
            None,
            execute,
            ["git", "tag", "--cleanup=verbatim", "--file=-", release.curr_tag],
            input=release.tag_changelog.encode("utf-8"),
        )

    check_output_dry(
        " [-] Pushing release...",
        execute,
        ["git", "push", "origin", current_branch]
        + [release.curr_tag for release in filtered_releases],
    )

    print(" [-] Creating GitHub releases...")
    for release in filtered_releases:
        check_output_dry(
            f"     - {release.crates[0].name if release.single_crate else 'tulpje'}",
            execute,
            [
                "gh",
                "release",
                "create",
                release.curr_tag,
                "--notes-file=-",
                "--title",
                release.curr_tag,
            ]
            + (["--prerelease"] if release.curr_version.prerelease is not None else []),
            input=release.changelog.encode("utf-8"),
        )


def main(args: argparse.Namespace) -> int:
    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(RustishFormatter())
    log.addHandler(handler)
    log.setLevel(logging.DEBUG)

    if args.skip_slow and args.execute:
        print(" [!] combining --skip-slow with --execute is disallowed")
        return 1

    current_branch = (
        subprocess.check_output(["git", "branch", "--show-current"]).decode().strip()
    )
    if args.execute and args.prerelease is None and current_branch != "main":
        print(
            f" [!] regular releases are only allowed on 'main' branch, currently on {current_branch} did you intend to use `--prelease`?"
        )
        return 1

    # exclude the workspace hack from processing
    crates = [c for c in gather_crates() if c.name != "workspace-hack"]
    independent_crates = [crate for crate in crates if crate.independent]
    grouped_crates = [crate for crate in crates if not crate.independent]

    releases = [
        gather_release([crate], independent_crates, args.prerelease, args.skip_slow)
        for crate in independent_crates
    ] + [
        gather_release(
            grouped_crates, independent_crates, args.prerelease, args.skip_slow
        )
    ]
    for release in releases:
        log.debug(
            f"crate {release.crates[0].name}, should release: {release.should_release}"
        )
    releases_by_deps = sort_releases_by_deps(releases)
    releasable = process_dependencies(releases_by_deps)
    do_releases(releasable, current_branch, args.execute)

    return 0


def entrypoint():
    sys.exit(main(argparser().parse_args(sys.argv[1:])))
