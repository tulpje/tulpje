{
  inputs,
  lib,
  craneLib,
  clang,
  wild,
}:
let
  crateName = "tulpje-utils";
  unfilteredRoot = ../..;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (unfilteredRoot + "/Cargo.toml")
      (unfilteredRoot + "/Cargo.lock")

      (craneLib.fileset.commonCargoSources (unfilteredRoot + "/crates/${crateName}"))
    ];
  };
  commonArgs = {
    inherit src;
    strictDeps = true;
    cargoExtraArgs = "-p ${crateName}";
    nativeBuildInputs = [
      clang
      wild
    ];
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs // {
    pname = "${crateName}-deps";
  };
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    pname = crateName;

    env = {
      TULPJE_VERSION_EXTRA = inputs.self.shortRev or inputs.self.dirtyShortRev or "";
      TULPJE_SKIP_VERGEN = true;
    };
  }
)
