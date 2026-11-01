{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "gateway-queue";
    rev = "c0433e5c8d91998f44524bab2265369b43a1ce1e";
    hash = "sha256-oyaH/Gz0wtFS0qrYka0f4D4Lyxjv8I0+9Kd8CDth3O8=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-gateway-queue";
    version = "unstable-2026-04-27";
    strictDeps = true;
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    meta.mainProgram = "twilight-gateway-queue";
  }
)
