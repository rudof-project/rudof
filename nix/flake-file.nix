{ inputs, ... }:
{
  flake-file.inputs = {
    flake-file.url = "github:denful/flake-file";
  };

  imports = [ inputs.flake-file.flakeModules.default ];
}
