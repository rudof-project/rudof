# `prefixmap`

The `prefixmap` crate contains an RDF prefix map implementation, where a prefix map is a list of alias declarations associated with [IRIs](./rudof_iri.md).
For example, in Turtle syntax, a prefix map can be declared as follows:

```turtle
@prefix schema: <https://schema.org/> .
@prefix : <https://example.org/> .
```

## Usage

For example, we can create a prefix map:

```rust
use prefixmap::PrefixMap;
use std::collections::HashMap;

let mut prefix_map = PrefixMap::new();

let other_map: PrefixMap = HashMap::from([
    ("", "https://example.org/"),
    ("schema", "https://schema.org/")
]).try_into()?;
```

And then register some prefixes:

```rust
use rudof_iri::IriS;
use std::str::FromStr;

prefix_map.add_prefix("schema", IriS::new_unchecked("https://schema.org/"));
prefix_map.add_prefix("ex", IriS::new_unchecked("https://example.org/"));

// Also, we can register a prefix with the IriS type:
let default_iri = IriS::from_str("https://default.org/")?;
prefix_map.add_prefix("", default_iri);
```

This will allow use to qualify IRIs using the registered prefixes:

```rust
// This will return "schema:Person"
let person = IriS::from_str("https://schema.org/Person")?;
prefix_map.qualify(&person);
```

And, if you need it, you get a basic prefix map or the [WikiData prefix map](https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Full_list_of_prefixes):

```rust
// Returns a basic prefix map
PrefixMap::basic();

// Returns the WikiData prefix map
PrefixMap::wikidata();
```

## Dependents and dependencies

This create depends mostly on the [`rudof_iri`](https://crates.io/crates/rudof_iri) and [`indexmap`](https://crates.io/crates/indexmap) crates.

This create is also used by other rudof modules that needs IRIs functionality, such as:
- [`rudof_lib`](./rudof_lib.md)
- [`shacl`](https://docs.rs/shacl)
- [`shapes_converter`](https://docs.rs/shapes_converter)
- [`shapes_comparator`](https://docs.rs/shapes_comparator)
- [`shex_ast`](https://docs.rs/shex_ast)
- [`shex_validation`](https://docs.rs/shex_validation)
- [`sparql_service`](https://docs.rs/sparql_service)
- [`rudof_rdf`](./rudof_rdf.md)

## Documentation

The crate documentation can be found [here](https://docs.rs/prefixmap).
