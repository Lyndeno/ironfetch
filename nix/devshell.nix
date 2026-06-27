{
  inputs,
  pkgs,
  system,
}: let
  craneLib = inputs.crane.mkLib pkgs;
  checks = inputs.pre-commit-hooks-nix.lib.${system}.run {
    src = ../.;
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
      cargo-flamegraph
    ];
    shellHook = ''
      ${checks.shellHook}
    '';
  }
