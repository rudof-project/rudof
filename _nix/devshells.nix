{
  pkgs,
  common,
  ...
}: {
  default = common.craneLib.devShell {
    shellHook = ''
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
