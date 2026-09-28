{ self, lib, ... }: {
  perSystem = { pkgs, self', ... }:
    let
      optionsDoc = pkgs.nixosOptionsDoc {
        options = (lib.evalModules {
          modules = [
            {
              options.programs.rudof = self.lib.mkRudofOptions {
                inherit pkgs;
                rudofPkg = self'.packages.rudof;
              };
            }
          ];
        }).options.programs;
      };
    in {
      packages.rudof-options-doc = pkgs.runCommand "rudof-options-doc.md" {
        meta = {
          description = "Reference for the rudof modules options";
          license = self.lib.licenses;
          platforms = self.lib.systems;
        };
      } ''
        cat <<'HEADER' > $out
        <!-- WARNING: This file has been generated automatically from the
        module's option declarations — do not edit directly. -->

        # Nix module options

        These options are shared by the `rudof.nixosModules.default` and
        `rudof.homeModules.default` flake outputs.

        HEADER
        cat ${optionsDoc.optionsCommonMark} >> $out
      '';
    };
}
