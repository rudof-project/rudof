{ inputs, ... }: {
  flake-file.inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  perSystem = { system, inputs', ... }: {
    _module.args.pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [ inputs.fenix.overlays.default ];
    };

    packages.toolchain = inputs'.fenix.packages.stable.withComponents [
      "cargo"
      "rustc"
      "rustfmt"
      "clippy"
      "rust-src"
    ];
  };
}
