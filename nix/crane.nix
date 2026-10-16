{ inputs, ... }: {
  flake-file.inputs = {
    crane.url = "github:ipetkov/crane";
  };

  perSystem = { self', pkgs, ... }: {
    _module.args.craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain
      self'.packages.toolchain).overrideScope (_: _: {
        # lbug vendors Apache Thrift C++ sources that rely on implicit
        # <cstdint> includes, which newer GCC/libstdc++ no longer provides.
        stdenvSelector = p: p.gcc13Stdenv;
      });
  };
}
