from pathlib import PurePath
import re
from typing import NamedTuple


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
