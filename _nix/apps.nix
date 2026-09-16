{pkgs, ...}: let
  rudof = {
    type = "app";
    program = "${pkgs.rudof.rudof}/bin/rudof";
  };
  rudof-generate = {
    type = "app";
    program = "${pkgs.rudof.rudof-generate}/bin/rudof_generate";
  };
in {
  default = rudof;
  inherit rudof rudof-generate;
}
