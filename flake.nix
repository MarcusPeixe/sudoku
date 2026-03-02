{
  description = "My Rust project with fenix and nix-darwin (aarch64-darwin)";

  inputs = {
    # nixpkgs for darwin
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # fenix provides Rust toolchains (rustc, cargo, etc.)
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # flake-utils to simplify multiple systems
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # import nixpkgs for this system
        pkgs = import nixpkgs {
          inherit system;
        };

        # get the fenix toolchain for this system
        rustToolchain = fenix.packages.${system}.complete.toolchain;

        # Use the same toolchain for nix builds
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        # devShell to give you a dev environment
        devShells.default = pkgs.mkShell {
          # include the Rust toolchain and any extra tools
          nativeBuildInputs = [
            rustToolchain
          ];

          # if you need extra build tools, list them here
          buildInputs = [
            pkgs.pkg-config
          ];
        };

        packages.default = rustPlatform.buildRustPackage {
          pname = "sudoku";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        # optional: nix run .#
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/sudoku";
        };
      });
}