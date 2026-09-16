{
  pkgs,
  common,
  ...
}: {
  default = common.craneLib.devShell {
    shellHook = ''
      export LD_LIBRARY_PATH=${common.pkgs.stdenv.cc.cc.lib}/lib:${common.pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
      export OPENSSL_ROOT_DIR=${common.pkgs.openssl.dev}
      export OPENSSL_LIB_DIR=${common.pkgs.openssl.out}/lib
      export OPENSSL_INCLUDE_DIR=${common.pkgs.openssl.dev}/include
      echo "rudof development shell loaded"
    '';
    packages = with pkgs; [
      common.toolchain
      cargo-edit
      openssl
      pre-commit
    ];
  };
}
