{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , naersk
    , fenix
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      toolchain = fenix.packages.${system}.stable;
      naersk' = pkgs.callPackage naersk {
        rustc = toolchain.rustc;
        cargo = toolchain.cargo;
      };
    in
    rec {
      packages = {
        forest-ds = naersk'.buildPackage {
          src = ./.;
          copyBins = false;
          copyLibs = true;
        };
        default = packages.forest-ds;
      };
      devShells.default = pkgs.mkShell {
        inputsFrom = [
          packages.forest-ds
        ];
        packages = with pkgs; [
          pre-commit
        ];
      };
    });
}
