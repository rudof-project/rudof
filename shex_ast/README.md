# ShEx AST

ShEx Abstract Syntax Tree.

The crate also contains:

- `compact`: the ShExC and ShapeMap parsers and printers.
- `ir`: the compiled form of a schema the validator runs on.
- `shexpath`: [ShExPath](https://shexspec.github.io/spec/ShExPath), a path language that
  addresses the elements of a schema.

## ShExPath

A ShExPath such as `@<#IssueShape>~:category` selects elements of a ShEx schema: shape
expressions, triple constraints, value sets, annotations and so on. The path is evaluated over
the ShExJ form of the schema, and each selected element comes with its location in that document
as a JSON Pointer.

```rust
use rudof_iri::IriS;
use shex_ast::compact::ShExParser;
use shex_ast::shexpath::ShExJDocument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = ShExParser::parse(
        r#"PREFIX : <http://ex.example/ns#>
           BASE <http://ex.example/schema>
           <#IssueShape> { :name . ; :category . }"#,
        None,
        &IriS::new_unchecked("http://ex.example/schema"),
    )?;
    let doc = ShExJDocument::new(&schema)?;

    // The path is written with the prefixes and base of the schema.
    for item in doc.select("@<#IssueShape>~:category")? {
        println!("{} {}", item.location(), item.value());
    }
    // /shapes/0/shapeExpr/expression/expressions/1 {"predicate":"http://ex.example/ns#category","type":"TripleConstraint"}
    Ok(())
}
```

- `ShExPathParser::parse(src, base, prefixmap)` reads a path into a `ShExPath`, resolving its
  IRIs.
- `ShExJDocument::evaluate(&path)` evaluates it from the schema, and
  `ShExJDocument::evaluate_from(&path, items)` from other items, for relative paths.
- An `assert` that does not hold is an error, `ShExPathError::AssertionFailed`.

See [the ShExPath guide](https://rudof-project.github.io/rudof/using-rudof/shexpath.html) for the
language and the points the specification leaves open.
