{common, ...}: let
  rudof = common.craneLib.buildPackage (common.commonArgs
    // {
      inherit (common) cargoArtifacts;
      pname = "rudof";
      cargoExtraArgs = "-p rudof_cli";
      meta.mainProgram = "rudof";
      preCheck = ''
        export LD_LIBRARY_PATH=${common.pkgs.stdenv.cc.cc.lib}/lib:${common.pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
        export OPENSSL_ROOT_DIR=${common.pkgs.openssl.dev}
        export OPENSSL_LIB_DIR=${common.pkgs.openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${common.pkgs.openssl.dev}/include
      '';
    });

  rudof-generate = common.craneLib.buildPackage (common.commonArgs
    // {
      inherit (common) cargoArtifacts;
      pname = "rudof-generate";
      cargoExtraArgs = "-p rudof_generate";
      meta.mainProgram = "rudof_generate";
      preCheck = ''
        export LD_LIBRARY_PATH=${common.pkgs.stdenv.cc.cc.lib}/lib:${common.pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
        export OPENSSL_ROOT_DIR=${common.pkgs.openssl.dev}
        export OPENSSL_LIB_DIR=${common.pkgs.openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${common.pkgs.openssl.dev}/include
      '';
    });
in {
  default = rudof;
  inherit rudof rudof-generate;
}
