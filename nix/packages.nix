{ rootPath, ... }: {
  perSystem = { self', craneLib, lib, pkgs, ... }: {
    packages.default = self'.packages.rudof;

    packages.rudof = let
      workspace = {
        src = lib.cleanSourceWith {
          src = lib.cleanSource rootPath;
          filter = path: type:
            (craneLib.filterCargoSources path type)
            || (lib.any (ext: lib.hasSuffix ".${ext}" path) [
              "rustemo" "md" "ttl"
              "shex" "sm" "rq" "pgs"
              "map" "pg" "csv"
            ]);
        };

        pname = "rudof-workspace";
        version = "0.3.21";
        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          git
          cmakeMinimal
          python3
        ];
        buildInputs = [ pkgs.openssl ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];
      };
      cargoArtifacts = craneLib.buildDepsOnly workspace;
    in craneLib.buildPackage (workspace
      // {
        inherit cargoArtifacts;
        pname = "rudof";
        meta.mainProgram = "rudof";
      });
  };
}
