{
  inputs,
  system,
  ...
}:
inputs.pre-commit-hooks-nix.lib.${system}.run {
  src = ../../.;
  hooks.alejandra.enable = true;
}
