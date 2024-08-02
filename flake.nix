{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/stable";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        app = pkgs.rustPlatform.buildRustPackage {
          name = "discord-dearrow-bot";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        layeredImageOptions = {
          name = "discord-dearrow-bot";
          tag = "latest";
          contents = [ pkgs.cacert ];
          config = {
            Cmd = [ "${app}/bin/discord-dearrow-bot" ];
          };
        };

        dockerImage = pkgs.dockerTools.buildLayeredImage layeredImageOptions;
        dockerImageStreaming = pkgs.dockerTools.streamLayeredImage layeredImageOptions;
      in with pkgs; rec {
        devShells.default = mkShell rec {
          nativeBuildInputs = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            })
            cargo-bloat
          ];
          buildInputs = [];
          packages = [];
        };
        packages = {
          inherit app dockerImage dockerImageStreaming;
          default = app;
        };
      });
}
