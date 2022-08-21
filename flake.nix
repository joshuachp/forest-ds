{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , naersk
    , fenix
    , ...
    }:
    let
      supportedSystems = with flake-utils.lib.system; [
        x86_64-linux
        x86_64-darwin
        aarch64-linux
        aarch64-darwin
      ];
      eachSystemMap = flake-utils.lib.eachSystemMap supportedSystems;
    in
    rec {
      packages = eachSystemMap (system:
        let
          naersk-lib = naersk.lib.${system};
          fenix-pkg = fenix.packages.${system}.stable;
        in
        {
          default = (naersk-lib.override {
            inherit (fenix-pkg) cargo rustc;
          }).buildPackage { root = ./.; };
        });

      apps = eachSystemMap (system: {
        default = flake-utils.lib.mkApp {
          drv = packages.${system}.default;
        };
      });

      devShells = eachSystemMap (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          fenix-pkg = fenix.packages.${system}.stable;
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              (fenix-pkg.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              pre-commit
            ];
          };
        });
    };
}
