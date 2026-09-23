{ moduleWithSystem, self, inputs, lib, ... }: let
  mkOptions = { pkgs, rudofPkg }: {
    enable = lib.mkEnableOption "Whether to enable rudof";
    package = lib.mkOption {
      type = lib.types.package;
      default = rudofPkg;
      description = "The rudof package to use";
    };
    settings = lib.mkOption {
      type = lib.types.submodule {
        freeformType = (pkgs.formats.toml { }).type;
      };
      default = { };
      description = "Configuration for rudof in nix";
    };
    extraArgs = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "" ];
      description = "Additional arguments for rudof";
    };
  };
  mkWrapper = { pkgs, cfg }: pkgs.symlinkJoin {
    name = "rudof-wrapped-${cfg.package.version or "0.0.0"}";
    paths = [ cfg.package ];
    nativeBuildInputs = [ pkgs.makeWrapper ];
    postBuild = ''
      wrapProgram $out/bin/rudof \
        --add-flags ${lib.escapeShellArgs cfg.extraArgs}
    '';
  };
  mkConfig = { pkgs, cfg }: lib.mkIf (cfg.settings != {}) {
    source = (pkgs.formats.toml {}).generate "rudof-config.toml" cfg.settings;
  };
  configPath = "rudof/config.toml";
  mkOutputs = { pkgs, cfg }: {
    package = mkWrapper { inherit pkgs cfg; };
    file = mkConfig { inherit pkgs cfg; };
  };
in {
  flake-file.inputs = {
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  flake.nixosModules.default = moduleWithSystem (
    perSystem@{ self', pkgs, ... }:
    nixos@{ pkgs, ... }: let
      cfg = nixos.config.programs.rudof;
      outputs = mkOutputs { inherit (nixos) pkgs; inherit cfg; };
    in {
      options.programs.rudof = mkOptions {
        inherit (perSystem) pkgs;
        rudofPkg = self'.packages.rudof;
      };

      config = lib.mkIf cfg.enable {
        environment.systemPackages = [ outputs.package ];
        environment.etc.${configPath} = outputs.file;
      };
    });

  flake.homeModules.default = moduleWithSystem (
    perSystem@{ self', pkgs, ... }:
    hm@{ pkgs, ... }: let
      cfg = hm.config.programs.rudof;
      outputs = mkOutputs { inherit (hm) pkgs; inherit cfg; };
    in {
      options.programs.rudof = mkOptions {
        inherit (perSystem) pkgs;
        rudofPkg = self'.packages.rudof;
      };

      config = lib.mkIf cfg.enable {
        home.packages = [ outputs.package ];
        xdg.configFile.${configPath} = outputs.file;
      };
    });

  perSystem = { pkgs, lib, ... }:
    let
      testSettings = {
        version = "0.0.0";
        base_iri = "http://default1/";
        rdf.base_iri = "http://default2/";
      };
    in {
      checks = lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
        rudof-nixos-module = pkgs.testers.nixosTest {
          name = "rudof-nixos-module";
          nodes.machine = {
            imports = [ self.nixosModules.default ];
            programs.rudof = {
              enable = true;
              settings = testSettings;
            };
          };
          testScript = ''
            machine.succeed("test -f /etc/${configPath}")
            machine.succeed("rudof --version")
          '';
        };
      } // {
        rudof-home-manager-module =
          let
            hm = inputs.home-manager.lib.homeManagerConfiguration {
              inherit pkgs;
              modules = [
                self.homeModules.default
                {
                  home.username = "rudof";
                  home.homeDirectory = "/home/rudof";
                  home.stateVersion = "26.05";
                  programs.rudof = {
                    enable = true;
                    settings = testSettings;
                  };
                }
              ];
            };
          in
          pkgs.runCommand "rudof-home-manager-module-check" { } ''
            test -e ${hm.config.xdg.configFile.${configPath}.source}
            ${hm.config.home.path}/bin/rudof --version
            touch $out
          '';
      };
    };
}
