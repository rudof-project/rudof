{ inputs, ... }: {
  flake-file.inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixpkgs.overlays = [ inputs.fenix.overlays.default ];

  perSystem = { inputs', ... }: {
    packages.toolchain = inputs'.fenix.packages.stable.withComponents [
      "cargo"
      "rustc"
      "rustfmt"
      "clippy"
      "rust-src"
    ];
  };
}
