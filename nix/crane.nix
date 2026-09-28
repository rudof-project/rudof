{ inputs, ... }: {
  flake-file.inputs = {
    crane.url = "github:ipetkov/crane";
  };

  perSystem = { self', pkgs, ... }: {
    _module.args.craneLib = (inputs.crane.mkLib pkgs).overrideToolchain
      self'.packages.toolchain;
  };
}
