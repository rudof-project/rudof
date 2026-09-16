{
  perSystem = { self', pkgs, craneLib, utils, ... }: {
    devShells.default = craneLib.devShell {
      packages = with pkgs; [
        self'.packages.toolchain
        cargo-edit
        openssl
        pre-commit
      ];
      shellHook = ''
        ${utils.env}
        echo "rudof development shell loaded"
      '';
    };
  };
}
