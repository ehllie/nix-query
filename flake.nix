{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        parts.follows = "parts";
      };
    };
  };

  outputs = inputs@{ parts, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        parts.flakeModules.easyOverlay
        inputs.nci.flakeModule
        ./crates.nix
      ];

      perSystem = { config, pkgs, ... }:
        let crateOutputs = config.nci.outputs.nix-query; in
        {
          overlayAttrs.nix-query = config.packages.default;
          packages.default = crateOutputs.packages.release;
          devShells.default = crateOutputs.devShell.overrideAttrs (old: { });
        };
    };
}
