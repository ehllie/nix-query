{ nixpkgs, input }:
let

  lib = import "${input}/modules/lib/stdlib-extended.nix" nixpkgs.lib;

  hm-modules = import "${input}/modules/modules.nix" { pkgs = nixpkgs.legacyPackages.aarch64-darwin; inherit lib; check = false; };

  readModules = lib.evalModules {
    modules = hm-modules ++
      [
        ({ ... }: {
          _module.check = false;
          home = {
            stateVersion = "24.05";
          };
        })
      ];
  };
in
{
  options.home-manager."base home-manager" = readModules.options;
}
