# iri_s

The `iri_s` crate contains a simple wrapper to work with IRIs (Internationalized Resource Identifiers). The main goal is that we can use a simple interface to work with IRIs without having to deal with the complexity of the underlying implementation.
This allows us to easily switch between different IRI implementations if needed.

## Usage

For example, we can create IRIs from URLs, Paths or Strings:

```rust
use iri_s::IriS;

let iri2 = IriS::from_str_base("https://example.org/name", None).unwrap();
println!("IRI: {}", iri.as_str());
```

Or extend a base IRI with a suffix:

```rust
use iri_s::IriS;

let iri2 = IriS::from_str_base("https://example.org/name", None).unwrap();
let extended = base.extend("subrecurso").unwrap();
println!("IRI extendido: {}", extended.as_str());
```

## Dependents and dependencies

This create depends mostly on the [`oxiri`](https://crates.io/crates/oxiri) and [`oxrdf`](https://crates.io/crates/oxrdf) crates.

This create is also used by other rudof modules that needs IRIs functionality, such as:
- [srdf](./srdf.md)
- [prefixmap](./prefixmap.md)
- [sparql_service](./sparql_service.md)
- [dctap](./dctap.md)
- [rudof_generate](./rudof_generate.md)
- [rudof_lib](./rudof_lib.md)
- [rudof_mcp](./rudof_mcp.md)
- [shacl_ast](./shacl_ast.md)
- [shacl_rdf](./shacl_rdf.md)
- [shacl_validation](./shacl_validation.md)
- [shapes_converter](./shapes_converter.md)
- [shex_ast](./shex_ast.md)
- [shex_validation](./shex_validation.md)

## Documentation

The crate documentation can be found [here](https://docs.rs/iri_s).
