inputs:
  inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    imports = [(import inputs.import-tree ./nix)];
    _module.args.rootPath = ./.;
  }
