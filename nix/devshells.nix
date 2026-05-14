{
  pkgs,
  common,
  ...
}: {
  default = common.craneLib.devShell {
    packages = with pkgs; [
      common.toolchain
      cargo-edit
      openssl
    ];
  };
}
