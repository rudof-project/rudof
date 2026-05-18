{
  description = "rudof - A RDF data shapes implementation in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # TODO - if we unify config files across crates, we could generate nix / home-manager
  # TODO - modules to auto generate config files for users (#659)
  outputs = {
    self,
    fenix,
    crane,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        lib = nixpkgs.lib;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
            self.overlays.default
          ];
        };
        common = import ./nix/common.nix {inherit pkgs inputs system lib rudof_version;};
        rudof_version = "0.3.1";
      in {
        packages = import ./nix/packages.nix {inherit common;};
        apps = import ./nix/apps.nix {inherit pkgs;};
        devShells = import ./nix/devshells.nix {inherit pkgs common;};
      }
    )
    // {
      overlays = import ./nix/overlays.nix {inherit self;};
    };
}
