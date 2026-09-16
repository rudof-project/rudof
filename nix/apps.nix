{
  perSystem = { self', lib, ... }: {
    apps.default = self'.apps.rudof;

    apps.rudof = {
      type = "app";
      program = lib.getExe self'.packages.rudof;
    };

    apps.rudof-generate = {
      type = "app";
      program = lib.getExe' self'.packages.rudof "rudof-generate";
    };
  };
}
