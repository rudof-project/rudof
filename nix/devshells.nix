{
  perSystem = { self', pkgs, ... }: {
    devShells.default = self'.craneLib.devShell {
      packages = with pkgs; [
        self'.packages.toolchain
        cargo-edit
        openssl
        pre-commit
      ];
      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
        export OPENSSL_ROOT_DIR=${pkgs.openssl.dev}
        export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
        echo "rudof development shell loaded"
      '';
    };
  };
}
