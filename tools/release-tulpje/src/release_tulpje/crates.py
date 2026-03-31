from glob import glob
import json
import logging
from pathlib import PurePath
import tomllib
import tomlkit
from typing import Iterable, NamedTuple, Self
from semver import Version
from tomlkit.items import AoT

from release_tulpje.util import find_file_upwards, process_run

log = logging.getLogger(__name__)


class CrateInfo(NamedTuple):
    name: str
    version: Version
    path: PurePath
    independent: bool
    workspace_dependencies: frozenset[str]
    lock_file: PurePath

    @property
    def manifest(self) -> PurePath:
        return self.path.joinpath("Cargo.toml")

    @classmethod
    def from_manifest(cls, path: PurePath) -> Self:
        with open(path, "rb") as manifest_file:
            manifest = tomllib.load(manifest_file)

        workspace_manifest_path = find_file_upwards(path.parent.parent, "Cargo.toml")
        if workspace_manifest_path is None:
            raise Exception("Couldn't find workspace manifest")

        with open(workspace_manifest_path, "rb") as workspace_manifest_file:
            workspace_manifest = tomllib.load(workspace_manifest_file)

        try:
            independent = manifest["package"]["version"]["workspace"] is not True
        except IndexError, TypeError:
            independent = True

        version = Version.parse(
            manifest["package"]["version"]
            if independent
            else workspace_manifest["workspace"]["package"]["version"]
        )

        manifest_json = process_run(
            ["cargo", "read-manifest", "--frozen", "--manifest-path", path]
        )
        parsed_manifest = json.loads(manifest_json)
        workspace_dependencies = {
            d["name"]
            for d in parsed_manifest["dependencies"]
            if "path" in d and d["name"] != "workspace-hack"
        }

        lock_file = find_file_upwards(path, "Cargo.lock")
        if lock_file is None:
            raise Exception("Couldn't find Cargo.lock")

        return cls(
            name=parsed_manifest["name"],
            version=version,
            path=path.parent,
            independent=independent,
            workspace_dependencies=frozenset(workspace_dependencies),
            lock_file=lock_file,
        )


def gather_crates() -> list[CrateInfo]:
    log.info("gathering crates ... ")
    try:
        member_paths = tomllib.load(open("Cargo.toml", "rb"))["workspace"]["members"]
    except IndexError:
        log.debug("no workspace members ... ")
        member_paths = []

    cargo_toml_paths = [
        PurePath(c) for p in member_paths for c in glob(f"{p}/Cargo.toml")
    ]
    return [
        CrateInfo.from_manifest(cargo_toml_path) for cargo_toml_path in cargo_toml_paths
    ]


def lock_bump_version(crate: CrateInfo, new_version: Version):
    with open(crate.lock_file) as manifest_file:
        manifest = tomlkit.load(manifest_file)

    packages = manifest["package"]
    assert isinstance(packages, AoT)

    package = next(filter(lambda pkg: pkg["name"] == crate.name, packages))
    if package is None:
        raise Exception("Couldn't find package `{crate.name}` in `{lock_file}`")

    package["version"] = str(new_version)

    with open(crate.lock_file, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)


def workspace_bump_version(
    manifest_path: PurePath, crates: Iterable[CrateInfo], version: Version
):
    with open(manifest_path) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        manifest["workspace"]["package"][  # pyright: ignore[reportIndexIssue]
            "version"
        ] = str(version)

    with open(manifest_path, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)

    for crate in crates:
        lock_bump_version(crate, version)


def manifest_bump_version(crate: CrateInfo, version: Version):
    with open(crate.manifest) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        if not isinstance(
            manifest["package"]["version"],  # pyright: ignore[reportIndexIssue]
            str,
        ):
            raise Exception(
                f"Crate {manifest['name']} doesn't have a string version, likely workspace crate"
            )

        manifest["package"]["version"] = str(  # pyright: ignore[reportIndexIssue]
            version
        )
    with open(crate.manifest, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)

    lock_bump_version(crate, version)


def workspace_update_dependency(
    crate: CrateInfo, dependency: CrateInfo, new_version: Version
):
    """Update workspace `crate`'s dependency on `dependency` to `new_version`"""
    with open(crate.manifest) as manifest_file:
        manifest = tomlkit.load(manifest_file)
        manifest["dependencies"][dependency.name][  # pyright: ignore[reportIndexIssue]
            "version"
        ] = str(new_version)
    with open(crate.manifest, "w") as manifest_file:
        tomlkit.dump(manifest, manifest_file)
