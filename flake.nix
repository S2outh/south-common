{
  description = "embassy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default 
          ];
        };
        profile = pkgs.fenix.complete;
        rust-analyzer = pkgs.fenix.rust-analyzer;
        std-lib = pkgs.fenix.targets.thumbv7em-none-eabihf.latest;
        rust-toolchain = pkgs.fenix.combine [
          profile.rustc-unwrapped
          profile.rust-src
          profile.cargo
          profile.rustfmt
          profile.clippy
          std-lib.rust-std
        ];
      in
      {
        devShells.default =
        pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            rust-analyzer

            # extra cargo tools
            cargo-edit
            cargo-expand
          ];

          # set the rust src for rust_analyzer
          RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
