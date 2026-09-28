{ lib, ... }: {
  options.flake.lib = lib.mkOption {
    type = lib.types.lazyAttrsOf lib.types.raw;
    default = { };
  };

  config.flake.lib = {
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];

    name = "rudof";
    licenses = with lib.licenses; [ mit asl20 ];
    description = "RDF semantic-less processing tool";
    homepage = "https://rudof-project.github.io/rudof";
  };
}
