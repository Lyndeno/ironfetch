{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.ironfetch = naersk-lib.buildPackage {
        pname = "ironfetch";
        root = ./.;
        nativeBuildInputs = [ pkgs.libsmbios pkgs.clang ];
      };
      packages.default = packages.ironfetch;

      apps.ironfetch = utils.lib.mkApp {
        drv = packages.ironfetch;
      };
      apps.default = apps.ironfetch;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo libsmbios pkgs.stdenv.cc.libc llvmPackages.libclang ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
      };
    });
}
