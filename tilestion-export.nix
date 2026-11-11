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
stdenv.mkDerivation {
  inherit pname version;

  src = lib.cleanSource ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = [
    rust-toolchain
    rustPlatform.cargoSetupHook
    chell-tilestion-transcoder
  ];

  buildPhase = ''
    runHook preBuild

    TARGET_ARCH="x86_64-unknown-linux-gnu";
    
    cargo rustdoc --target $TARGET_ARCH -- -Z unstable-options --output-format json
    
    INPUT="target/$TARGET_ARCH/doc/south_common.json"
    META="tilestion-meta.ron"

    chell-tilestion-transcoder $INPUT $META \
      definitions::command_msgs \
      definitions::timesync_msgs \
      definitions::telemetry \
      definitions::groundstation > telemetry.json

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    cp telemetry.json $out

    runHook postInstall
  '';
}
