{
  description = "A library to add quick tui setup wizards to rust applications";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell rec {
          buildInputs = [];
          nativeBuildInputs = [
            pkgs.bash
            pkgs.git

            pkgs.pkg-config
            pkgs.cmake
            pkgs.rustup

            pkgs.clang
            pkgs.llvmPackages.bintools

            pkgs.yaml-language-server
          ];

          RUSTC_VERSION = "nightly-2026-07-19";

          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages.libclang.lib];

          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          '';

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
        };
      }
    );
}
