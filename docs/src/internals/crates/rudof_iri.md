# rudof_iri

The `rudof_iri` crate contains a simple wrapper to work with IRIs (Internationalized Resource Identifiers). The main goal is that we can use a simple interface to work with IRIs without having to deal with the complexity of the underlying implementation.
This allows us to easily switch between different IRI implementations if needed.

## Usage

For example, we can create IRIs from URLs, Paths or Strings:

```rust
use rudof_iri::IriS;

let iri = IriS::from_str_base("https://example.org/name", None).unwrap();
println!("IRI: {}", iri.as_str());
```

Or extend a base IRI with a suffix:

```rust
use rudof_iri::IriS;

let base = IriS::from_str_base("https://example.org/", None).unwrap();
let extended = base.extend("course").unwrap();
println!("Extended iri: {}", extended.as_str());
```

## Dependents and dependencies

This create depends mostly on the [`oxiri`](https://crates.io/crates/oxiri) and [`oxrdf`](https://crates.io/crates/oxrdf) crates.

This create is also used by other rudof modules that needs IRIs functionality, such as:
- [`rudof_rdf`](./rudof_rdf.md)
- [`prefixmap`](./prefixmap.md)
- [`sparql_service`](https://docs.rs/sparql_service)
- [`dctap`](https://docs.rs/dctap)
- [`rudof_generate`](https://docs.rs/rudof_generate)
- [`rudof_lib`](./rudof_lib.md)
- [`rudof_mcp`](./rudof_mcp.md)
- [`shacl`](https://docs.rs/shacl)
- [`shapes_converter`](https://docs.rs/shapes_converter)
- [`shex_ast`](https://docs.rs/shex_ast)
- [`shex_validation`](https://docs.rs/shex_validation)

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_iri).
