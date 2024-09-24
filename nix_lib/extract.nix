{ nixpkgs-flake, input-flake, extractor }:
let
  input = builtins.getFlake input-flake;
  nixpkgs = builtins.getFlake nixpkgs-flake;
  lib = nixpkgs.lib;
  extraction-fn = import extractor;
  extracted = extraction-fn { inherit nixpkgs input; };
in
{
  results = {
    options = lib.foldlAttrs
      (acc: kind: sources:
        acc ++
        lib.foldlAttrs
          (acc: source: opts:
            acc ++
            lib.concatMap
              (opt:
                if !opt.internal && opt.visible
                then [ (opt // { inherit source kind; }) ]
                else [ ]
              )
              (lib.optionAttrSetToDocList opts)
          )
          [ ]
          sources
      )
      [ ]
      extracted.options;
  };
}
