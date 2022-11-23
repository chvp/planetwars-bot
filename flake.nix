{
  description = "Planetwars";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devshell = {
      url = "github:numtide/devshell";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, devshell }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ devshell.overlay (import rust-overlay) ]; };
        planetwars-client = pkgs.rustPlatform.buildRustPackage rec {
          pname = "planetwars-client";
          version = "unstable";

          cargoBuildFlags = "--bin planetwars-client";
          nativeBuildInputs = [ pkgs.cmake ];
          buildInputs = [ pkgs.postgresql pkgs.openssl.dev ];
          doCheck = false;

          src = pkgs.fetchFromGitHub {
            name = "planetwars.dev";
            owner = "iasoon";
            repo = "planetwars.dev";
            rev = "7154b16dedd29f0188c3040816e0013198e3bf63";
            sha256 = "sha256-mVIQecg2MEs8ZFTzv3RNM8tmEmg0jYJWYxIBQULHfD0=";
          };

          cargoSha256 = "sha256-co1yxuH5BE18mV4Lhu9UhQy/Pk3dzyjfwmBQ4eAkOJs=";

          meta = with pkgs.lib; {
            description = "Planetwars client";
            homepage = "planetwars.dev";
          };
        };
      in
      {
        devShells.default = pkgs.devshell.mkShell {
          imports = [ "${devshell}/extra/language/c.nix" ];
          name = "Planetwars bot";
          packages = with pkgs; [
            (rust-bin.stable.latest.default.override { extensions = [ "rust-analyzer" "rust-src" ]; })
            cargo-watch
            ffmpeg
            nixpkgs-fmt
            planetwars-client
          ];
          commands = [
            {
              name = "testbot";
              help = "Build and test";
              command = "cargo build --release && planetwars-client charlotte.toml $@";
            }
          ];
        };
        language.c = {
          compiler = pkgs.clang;
        };
      }
    );
}
