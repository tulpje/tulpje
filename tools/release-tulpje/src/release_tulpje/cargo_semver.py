import logging
import subprocess
from typing import NamedTuple, Optional


log = logging.getLogger(__name__)


class SemverCheckResult(NamedTuple):
    breaking: bool
    output: str


def cargo_semver_checks(
    baseline_rev: str, crate: Optional[str] = None
) -> SemverCheckResult:
    log.info(
        f"running `cargo semver-checks` with crate={crate} baseline_rev={baseline_rev} ... "
    )
    result = subprocess.run(
        ["cargo", "semver-checks", "--baseline-rev", baseline_rev]
        + ([] if crate is None else ["--package", crate]),
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        encoding="utf-8",
    )
    return SemverCheckResult(result.returncode != 0, result.stdout)
