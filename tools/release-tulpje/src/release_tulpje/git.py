from pathlib import PurePath
import re
from typing import Iterable, NamedTuple, Optional

from release_tulpje.util import process_run
from release_tulpje.version import latest_version, parse_semver_from_tag


class CommitInfo(NamedTuple):
    # type(scope)!: subject
    SUBJECT_REGEX = re.compile(r"^(.*?)(\(.*\))?(!)?: (.*)$")

    sha: str

    subject: str
    typ: str
    scope: str
    breaking: bool

    raw_subject: str
    files: list[PurePath]

    @classmethod
    def create(cls, sha: str, raw_subject: str, files: list[PurePath]):
        match = cls.SUBJECT_REGEX.match(raw_subject)
        assert match is not None

        typ, scope, breaking, subject = match.groups()
        return cls(sha, subject, typ, scope, breaking == "!", raw_subject, files)


def get_commits_since_ref(ref: str) -> list[CommitInfo]:
    def create_commit_info(sha, subject) -> CommitInfo:
        return CommitInfo.create(sha, subject, get_commit_files(sha))

    return [
        create_commit_info(*commit.split(" ", 1))
        for commit in process_run(
            ["git", "log", "--format=%H %s", f"{ref}..HEAD"], encoding="utf-8"
        ).splitlines()
    ]


def get_commit_files(sha: str) -> list[PurePath]:
    return [
        PurePath(p)
        for p in process_run(
            ["git", "diff-tree", "--no-commit-id", "--name-only", "-r", sha],
            encoding="utf-8",
        ).splitlines()
    ]


def git_tags_with_prefix(prefix: str = "") -> list[str]:
    return [
        tag
        for tag in process_run(["git", "tag", "--list"], encoding="utf-8").splitlines()
        if tag.startswith(f"{prefix}v")
    ]


def get_latest_tag(prefix: str = "", include_prerelease: bool = False) -> Optional[str]:
    tags = git_tags_with_prefix(prefix)
    versions = [
        v
        for v in (parse_semver_from_tag(t, prefix) for t in tags)
        if v is not None and (v.prerelease is None or include_prerelease)
    ]
    latest = latest_version(versions)
    if latest is None:
        return None

    return f"{prefix}v{latest}"


def filter_commits_by_path(
    commits: list[CommitInfo], paths: Iterable[PurePath], invert=False
) -> list[CommitInfo]:
    def check(file: PurePath, paths: Iterable[PurePath], invert: bool) -> bool:
        if invert:
            return any(path not in file.parents for path in paths)
        else:
            return any(path in file.parents for path in paths)

    return [
        commit
        for commit in commits
        if any(check(file, paths, invert) for file in commit.files)
    ]
