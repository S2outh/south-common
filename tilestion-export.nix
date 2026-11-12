{
  runCommand,
  rustPlatform,
  rust-toolchain,
  chell-tilestion-transcoder,
  stdenv,
  lib,
  pname? "telemetry",
  version? "latest",
  ...
}:
let
  # set src explicitely to git tracked files
  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.gitTracked ./.;
  };

  # get short hash from nix store path
  shortHash = builtins.substring 0 7 (baseNameOf src);
in
stdenv.mkDerivation {
  inherit pname version src;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = [
    rust-toolchain
    rustPlatform.cargoSetupHook
    chell-tilestion-transcoder
  ];

  env.HASH = shortHash;

  buildPhase = ''
    runHook preBuild

    TARGET_ARCH="x86_64-unknown-linux-gnu";
    
    cargo rustdoc --target $TARGET_ARCH -- -Z unstable-options --output-format json
    
    INPUT="target/$TARGET_ARCH/doc/south_common.json"
    META="tilestion-meta.ron"
    VERSION="south-tmtc-$(date -u +%Y%m%d)-$HASH"

    chell-tilestion-transcoder \
      -v $VERSION $INPUT $META \
      definitions::command_msgs \
      definitions::timesync_msgs \
      definitions::telemetry \
      definitions::groundstation > export.json

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    cp export.json $out

    runHook postInstall
  '';
}
