{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    blueprint = {
      url = "github:numtide/blueprint";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.systems.follows = "";
    };

    ci.url = "github:Lyndeno/ci";
    ci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs: let
    systems = ["x86_64-linux" "aarch64-linux"];
    bp = inputs.blueprint {
      inherit inputs systems;
      prefix = "nix/";
    };
  in
    bp
    // {
      overlays.default = final: _prev: bp.mkPackagesFor final;
    };
}
