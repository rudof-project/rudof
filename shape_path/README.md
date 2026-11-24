# shape_path

[ShExPath](https://shexspec.github.io/spec/ShExPath), a path language that addresses the elements
of a ShEx schema.

A ShExPath such as `@<#IssueShape>~:category` selects elements of a ShEx schema: shape
expressions, triple constraints, value sets, annotations and so on. The path is evaluated over
the ShExJ form of the schema, and each selected element comes with its location in that document
as a JSON Pointer.

The crate contains:

- the abstract syntax of paths, `ShExPath`, and their parser, `ShExPathParser`;
- the evaluator, over a `ShExJDocument`: a schema written as ShExJ, with its prefixes and base;
- `SchemaPath`, the path of a shape expression or a triple constraint, written with the shortcuts
  a person would use (`@:S~:p`), which ShEx validation errors use to say where a failure is.

See [the ShExPath guide](https://rudof-project.github.io/rudof/using-rudof/shexpath.html) for the
language and the points the specification leaves open.
