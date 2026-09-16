{ rootPath, ... }: {
  perSystem = { self', craneLib, lib, pkgs, utils, ... }: {
    _module.args.utils.env = ''
      export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
      export OPENSSL_ROOT_DIR=${pkgs.openssl.dev}
      export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
      export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
    '';

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
          ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
          pkgs.libiconv
        ];
      };
      cargoArtifacts = craneLib.buildDepsOnly workspace;
    in craneLib.buildPackage (workspace
      // {
        inherit cargoArtifacts;
        pname = "rudof";
        meta.mainProgram = "rudof";
        preCheck = ''
          ${utils.env}
        '';
      });
  };
}
