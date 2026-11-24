# ShEx AST

ShEx Abstract Syntax Tree.

The crate also contains:

- `compact`: the ShExC and ShapeMap parsers and printers.
- `ir`: the compiled form of a schema the validator runs on.
- `shexpath`: [ShExPath](https://shexspec.github.io/spec/ShExPath), a path language that
  addresses the elements of a schema. The language lives in the
  [`shape_path`](https://docs.rs/shape_path) crate, which this module re-exports; the module adds
  `shexj` and `shexj_document`, which write a schema as the ShExJ document a path is evaluated
  over. See [the ShExPath guide](https://rudof-project.github.io/rudof/using-rudof/shexpath.html).
