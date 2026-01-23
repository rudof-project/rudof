# `prefixmap`

The `prefixmap` crate contains an RDF prefix map implementation, where a prefix map is a list of alias declarations associated with [IRIs](./iri_s.md).
For example, in Turtle syntax, a prefix map can be declared as follows:

```turtle
@prefix schema: <https://schema.org/> .
@prefix : <https://example.org/> .
```

## Usage

For example, we can create a prefix map:

```rust
use prefixmap::PrefixMap;

let mut prefix_map = PrefixMap::new();

let other_map = PrefixMap::from_hashmap(
    HashMap::from([
        ("", "https://example.org/"),
        ("schema", "https://schema.org/")
    ])
)?;
```

And then register some prefixes:

```rust
use iri_s::IriS;

prefix_map.add_prefix("schema", "https://schema.org/");
prefix_map.add_prefix("ex", "https://example.org/");

// Also, we can register a prefix with the IriS type:
let default_iri = IriS::from_string("https://default.org/")?;
prefix_map.add_prefix("", default_iri);
```

This will allow use to qualify IRIs using the registered prefixes:

```rust
// This will return "schema:Person"
prefix_map.qualify("https://schema.org/Person")?;
```

And, if you need it, you get a basic prefix map or the [WikiData prefix map](https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Full_list_of_prefixes):

```rust
// Returns a basic prefix map
PrefixMap::basic();

// Returns the WikiData prefix map
PrefixMap::wikidata();
```

## Dependents and dependencies

This create depends mostly on the [`iri_s`](https://crates.io/crates/iri_s) and [`indexmap`](https://crates.io/crates/indexmap) crates.

This create is also used by other rudof modules that needs IRIs functionality, such as:
- [`rudof_lib`](./rudof_lib.md)
- [`shacl_ast`](./shacl_ast.md)
- [`shacl_ir`](./shacl_ir.md)
- [`shacl_validation`](./shacl_validation.md)
- [`shacl_rdf`](./shacl_rdf.md)
- [`shapes_converter`](./shapes_converter.md)
- [`shapes_comparator`](./shapes_comparator.md)
- [`shex_ast`](./shex_ast.md)
- [`shex_validation`](./shex_validation.md)
- [`sparql_service`](./sparql_service.md)
- [`srdf`](./srdf.md)

## Documentation

The crate documentation can be found [here](https://docs.rs/prefixmap).
