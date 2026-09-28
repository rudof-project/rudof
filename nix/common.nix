{
  pkgs,
  inputs,
  system,
  lib,
  rudof_version,
  ...
}: let
  crane = inputs.crane;
  fenix = inputs.fenix;
in rec {
  toolchain = fenix.packages.${system}.stable.withComponents [
    "cargo"
    "rustc"
    "rustfmt"
    "clippy"
    "rust-src"
  ];

  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

  src = lib.cleanSourceWith {
    src = lib.cleanSource ../.;
    filter = path: type:
      (craneLib.filterCargoSources path type)
      # Prevent crane from stripping .rustemo files, which are needed for rustemo
      || (lib.hasSuffix ".rustemo" path)
      # Prevent crane from stripping .md files, which are needed for shacl lib.rs docs
      || (lib.hasSuffix ".md" path)
      # Prevent crane from stripping files which are needed for tests
      || (lib.hasSuffix ".ttl" path)
      || (lib.hasSuffix ".shex" path)
      || (lib.hasSuffix ".sm" path)
      || (lib.hasSuffix ".rq" path)
      || (lib.hasSuffix ".pgs" path)
      || (lib.hasSuffix ".map" path)
      || (lib.hasSuffix ".pg" path)
      || (lib.hasSuffix ".csv" path);
  };

  commonArgs = {
    inherit src;

    pname = "rudof-workspace";
    version = rudof_version;
    strictDeps = true;

    # Needed for rustemo
    nativeBuildInputs = with pkgs; [
      git
      cmakeMinimal
      python3
    ];

    buildInputs = with pkgs; [ openssl ]
      ++ lib.optionals pkgs.stdenv.isDarwin [
      pkgs.libiconv
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  inherit pkgs;
}
