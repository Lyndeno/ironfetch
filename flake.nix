{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
    pre-commit-hooks-nix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      craneLib = crane.mkLib pkgs;
      lib = pkgs.lib;

      jsonFilter = path: _type: builtins.match ".*json$" path != null;
      jsonOrCargo = path: type:
        (jsonFilter path type) || (craneLib.filterCargoSources path type);

      common-args = {
        src = lib.cleanSourceWith {
          src = ./.;
          filter = jsonOrCargo;
          name = "source";
        };
        strictDeps = true;

        buildInputs = [pkgs.udev];
        nativeBuildInputs = [pkgs.installShellFiles pkgs.pkg-config];

        postInstall = ''
          installShellCompletion --cmd ironfetch \
            --bash ./target/release/build/ironfetch-*/out/ironfetch.bash \
            --fish ./target/release/build/ironfetch-*/out/ironfetch.fish \
            --zsh ./target/release/build/ironfetch-*/out/_ironfetch
          installManPage ./target/release/build/ironfetch-*/out/ironfetch.1
        '';
      };

      ironfetch = craneLib.buildPackage (common-args
        // {
          cargoArtifacts = craneLib.buildDepsOnly common-args;
        });
    in rec {
      checks = {
        inherit ironfetch;
      };
      packages.ironfetch = ironfetch;
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
            clippy.enable = true;
          };
        };
      in
        craneLib.devShell {
          packages = with pkgs; [
            rustfmt
            clippy
            cargo-deny
            cargo-about
            termshot
            pkg-config
            udev
          ];
          shellHook = ''
            ${pre-commit-format.shellHook}
          '';
        };
    });
}
