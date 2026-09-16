{common, ...}: let
  cargoArtifacts = common.cargoArtifacts;
  rudof = common.craneLib.buildPackage (common.commonArgs
    // {
      inherit cargoArtifacts;
      pname = "rudof";
      cargoExtraArgs = "-p rudof_cli";
      meta.mainProgram = "rudof";
    });

  rudof-generate = common.craneLib.buildPackage (common.commonArgs
    // {
      inherit cargoArtifacts;
      pname = "rudof-generate";
      cargoExtraArgs = "-p rudof_generate";
      meta.mainProgram = "rudof_generate";
    });
in {
  default = rudof;
  inherit rudof rudof-generate;
}
