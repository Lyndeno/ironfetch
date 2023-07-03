{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    pre-commit-hooks-nix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.ironfetch = naersk-lib.buildPackage {
        nativeBuildInputs = [pkgs.installShellFiles];
        postInstall = ''
          installShellCompletion --cmd ironfetch \
            --bash $releaseDir/build/ironfetch-*/out/ironfetch.bash \
            --fish $releaseDir/build/ironfetch-*/out/ironfetch.fish \
            --zsh $releaseDir/build/ironfetch-*/out/_ironfetch
        '';
        pname = "ironfetch";
        root = ./.;
      };
      packages.default = packages.ironfetch;

      apps.ironfetch = utils.lib.mkApp {
        drv = packages.ironfetch;
      };
      apps.default = apps.ironfetch;

      formatter = pkgs.alejandra;

      devShells.default = let
        pre-commit-format = pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
          };
        };
      in
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo rustfmt clippy];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            ${pre-commit-format.shellHook}
          '';
        };
    });
}
