# `shape_path`

The `shape_path` crate implements [ShExPath](https://shexspec.github.io/spec/ShExPath), a path
language that addresses the elements of a ShEx schema. See [ShExPath](../../using-rudof/shexpath.md)
for the language as rudof implements it.

## What it contains

- `ShExPath` and its parts, the abstract syntax of a path, which follows the grammar of the
  specification.
- `ShExPathParser`, which reads a path and resolves its IRIs with a base and a prefix map.
- `ShExJDocument`, a schema written as ShExJ (a `serde_json::Value`) with its prefixes and base,
  which evaluates paths and returns the `ShExPathItem`s they select, each with its `Location` as a
  JSON Pointer.
- `SchemaPath`, the path of a shape expression or a triple constraint, written with the shortcuts
  of the language (`@:S~:p`). ShEx validation errors use it to say where a failure is.

## Dependents and dependencies

The crate depends on [`prefixmap`](./prefixmap.md), [`rudof_iri`](./rudof_iri.md), `nom` and
`serde_json`. It does not depend on [`shex_ast`](https://docs.rs/shex_ast): `shex_ast` depends on
it, because the compiled schema (`SchemaIR::path_of`) returns `SchemaPath`s.

`shex_ast::shexpath` re-exports this crate and adds what needs a parsed `Schema`: `shexj`, which
writes the schema as ShExJ, and `shexj_document`, which makes the `ShExJDocument`.

It is used by:
- [`shex_ast`](https://docs.rs/shex_ast)
- [`shex_validation`](https://docs.rs/shex_validation)
- [`rudof_lib`](./rudof_lib.md), through `shex_ast::shexpath`

The parser has its own copy of the IRI terminals of ShExC (`IRIREF`, `PNAME_NS`, `PNAME_LN`).
[#970](https://github.com/rudof-project/rudof/issues/970) is about sharing one copy between both
parsers.

## Documentation

The crate documentation can be found [here](https://docs.rs/shape_path).