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
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  in
    utils.lib.eachSystem systems (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      craneLib = crane.mkLib pkgs;
      lib = pkgs.lib;

      jsonFilter = path: _type: builtins.match ".*json$" path != null;
      jsonOrCargo = path: type:
        (jsonFilter path type) || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith {
        src = ./.;
        filter = jsonOrCargo;
        name = "source";
      };

      common-args = {
        inherit src;
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

      cargoArtifacts = craneLib.buildDepsOnly common-args;

      ironfetch = craneLib.buildPackage (common-args
        // {
          inherit cargoArtifacts;
        });

      pre-commit-check = hooks:
        pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          inherit hooks;
        };
    in rec {
      checks = {
        inherit ironfetch;

        ironfetch-clippy = craneLib.cargoClippy (common-args
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        ironfetch-fmt = craneLib.cargoFmt {
          inherit src;
        };

        ironfetch-deny = craneLib.cargoDeny {
          inherit src;
        };

        pre-commit-check = pre-commit-check {
          alejandra.enable = true;
        };
      };
      packages.ironfetch = ironfetch;
      packages.default = packages.ironfetch;

      apps.ironfetch = utils.lib.mkApp {
        drv = packages.ironfetch;
      };
      apps.default = apps.ironfetch;

      formatter = pkgs.alejandra;

      devShells.default = let
        checks = pre-commit-check {
          alejandra.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
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
            cargo-flamegraph
          ];
          shellHook = ''
            ${checks.shellHook}
          '';
        };
    })
    // {
      hydraJobs = {
        inherit (self) checks packages devShells;
      };
    };
}
