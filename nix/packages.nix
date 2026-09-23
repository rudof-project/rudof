{ rootPath, ... }: {
  perSystem = { self', craneLib, lib, pkgs, utils, ... }: let
    workspace = {
      src = lib.cleanSourceWith {
        src = lib.cleanSource rootPath;
        filter = path: type:
          (craneLib.filterCargoSources path type)
          || (lib.any (ext: lib.hasSuffix ext path) [
            ".rustemo" ".md" ".ttl"
            ".shex" ".sm" ".rq" ".pgs"
            ".map" ".pg" ".csv" ".json"
          ]);
      };

      pname = "rudof-workspace";
      version = "0.3.21";
      strictDeps = true;
      doCheck = false;

      nativeBuildInputs = with pkgs; [
        git
        cmakeMinimal
        python3
        installShellFiles
        autoPatchelfHook
      ];
      buildInputs = with pkgs; [
        openssl
        stdenv.cc.cc.lib
      ]
        ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
        pkgs.libiconv
      ];
    };
    cargoArtifacts = craneLib.buildDepsOnly workspace;
    rudofPkg = craneLib.buildPackage (workspace
      // {
        inherit cargoArtifacts;
        pname = "rudof";
        meta.mainProgram = "rudof";
        preCheck = ''
          ${utils.env}
        '';
        preInstall = ''
          ${utils.env}
        '';
        postInstall = ''
          echo "Generating shell completion scripts"

          ./target/release/rudof completion bash > rudof.bash
          ./target/release/rudof completion zsh > _rudof
          ./target/release/rudof completion fish > rudof.fish
          ./target/release/rudof completion elvish > rudof.elvish
          ./target/release/rudof completion nushell > rudof.nu
          ./target/release/rudof completion powershell > rudof.ps1

          installShellCompletion --bash rudof.bash
          installShellCompletion --zsh _rudof
          installShellCompletion --fish rudof.fish

          mkdir -p \
            $out/share/powershell/completions \
            $out/share/elvish/lib \
            $out/share/nushell/vendor/autoload

          cp rudof.ps1 $out/share/powershell/completions/rudof.ps1
          cp rudof.elvish $out/share/elvish/lib/rudof.elv
          cp rudof.nu $out/share/nushell/vendor/autoload/rudof.nu
        '';
      });
    rudofClippy = craneLib.cargoClippy (workspace
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets --all-features --workspace -- -D warnings";
      });
    rudofTest = craneLib.cargoTest (workspace
      // {
        inherit cargoArtifacts;
        cargoTestExtraArgs = "--workspace";
      });
    rudofFmt = craneLib.cargoFmt (workspace
      // {
        inherit cargoArtifacts;
        cargoExtraArgs = "--all";
      });
  in {
    _module.args.utils.env = ''
      export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
      export OPENSSL_ROOT_DIR=${pkgs.openssl.dev}
      export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
      export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
    '';

    packages.default = self'.packages.rudof;
    packages.rudof = rudofPkg;

    checks = { inherit rudofPkg rudofClippy rudofTest rudofFmt; };
  };
}
