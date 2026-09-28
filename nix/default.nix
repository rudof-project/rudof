{
  flake-file.description = "rudof - An RDF data shapes implementation in Rust";

  flake-file.inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    import-tree.url = "github:vic/import-tree";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}
