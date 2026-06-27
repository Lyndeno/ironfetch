{
  inputs,
  pkgs,
  ...
}: let
  craneLib = inputs.crane.mkLib pkgs;
  lib = pkgs.lib;

  jsonFilter = path: _type: builtins.match ".*json$" path != null;
  jsonOrCargo = path: type:
    (jsonFilter path type) || (craneLib.filterCargoSources path type);

  src = lib.cleanSourceWith {
    src = ./../../.;
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

      passthru.tests = {
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
      };
    });
in
  ironfetch
