import logging
from typing import Optional
from semver import Version

log = logging.getLogger(__name__)


def version_bump(
    sem_ver: Version, feature: bool, breaking: bool, prerelease: Optional[str]
) -> Version:
    if sem_ver.major == 0:
        if breaking:
            sem_ver = sem_ver.bump_minor()
        elif prerelease is None or (
            sem_ver.prerelease is None and prerelease is not None
        ):
            sem_ver = sem_ver.next_version("patch")
    else:
        if breaking:
            sem_ver = sem_ver.bump_major()
        elif feature:
            sem_ver = sem_ver.bump_minor()
        elif prerelease is None:
            sem_ver = sem_ver.next_version("patch")

    if prerelease is None:
        return sem_ver
    else:
        # if there's a different prerelease token strip the existing one
        if (
            sem_ver.prerelease is not None
            and sem_ver.prerelease.rsplit(".", 1)[0] != prerelease
        ):
            sem_ver = sem_ver.replace(prerelease=None)
        return sem_ver.bump_prerelease(token=prerelease)


def latest_version(versions: list[Version]) -> Optional[Version]:
    return max(versions) if len(versions) > 0 else None


def prefixed_version(prefix: str, version: Version) -> str:
    prefix.removesuffix("-")
    return f"v{version}" if len(prefix) == 0 else f"{prefix}-v{version}"


def parse_semver_from_tag(tag: str, prefix: str = "") -> Optional[Version]:
    normalized = tag.removeprefix(f"{prefix}v")
    try:
        return Version.parse(tag.removeprefix(f"{prefix}v"))
    except ValueError as error:
        log.warning(
            f"couldn't parse valid semver from `{tag}`, tried parsing `{normalized}`: {str(error)}"
        )
