# `rudof_rdf`

The `rudof_rdf` crate is a core component of the Rudof project, providing foundational data structures, utilities, and algorithms for working with RDF (Resource Description Framework) data in Rust. It supports parsing, querying, manipulating, and visualizing RDF graphs, and serves as the backbone for higher-level crates in the Rudof ecosystem.

## Architecture and Package Structure

The crate is organized into several key modules:

- **rdf_core**: Core logic for RDF handling, including:
  - `term`: RDF terms (IRIs, blank nodes, literals, triples)
  - `parser`: Parsers for RDF nodes and documents
  - `query`: SPARQL query support and result handling
  - `vocab`: Common RDF, RDFS, XSD, and SHACL vocabulary constants
  - `utils`: Utilities such as regex helpers
  - `visualizer`: Tools for visualizing RDF graphs (UML, styles, etc.)
  - `matcher`, `focus_rdf`, `neighs_rdf`, etc.: Advanced graph navigation and matching
- **rdf_impl**: Implementations of RDF storage and access:
  - `in_memory_graph`: In-memory RDF graph implementation
  - `sparql_endpoint`: SPARQL endpoint integration
  - `oxrdf_impl`: Integration with the `oxrdf` crate

## Dependents and dependencies

This create depends mostly on:

- Internal Rudof crates:
  - [`iri_s`](./iri_s.md)
  - [`prefixmap`](./prefixmap.md)
- External:
  - `oxigraph`
  - `oxrdf`
  - `oxjsonld`
  - `oxiri`
  - `oxilangtag`
  - `oxrdfio`
  - `oxrdfxml`
  - `oxsdatatypes`
  - `oxttl`
  - `reqwest`
  - `tokio`

This crate is a foundational dependency for many other Rudof crates, including:

- [`rudof_lib`](./rudof_lib.md)
- [`rudof_cli`](./rudof_cli.md)
- [`shacl_ast`](./shacl_ast.md), [`shacl_ir`](./shacl_ir.md), [`shacl_rdf`](./shacl_rdf.md), [`shacl_validation`](./shacl_validation.md)
- [`shex_ast`](./shex_ast.md), [`shex_validation`](./shex_validation.md)
- [`shex_testsuite`](./shex_testsuite.md), [`shapes_comparator`](./shapes_comparator.md), [`shapes_converter`](./shapes_converter.md), [`sparql_service`](./sparql_service.md), and others.

## Usage

The following examples illustrate just one of the many features `rdf` provides — fluent parser composition:

### Composing Parsers with Fluent API

```rust
use rudof_rdf::rdf_core::{
    FocusRDF,
    parser::{
        RDFParse,
        rdf_node_parser::{
            RDFNodeParse, // Core trait
            ParserExt, // Extension trait (fluent API)
            constructors::{
                ObjectParser,               // captures the current focus node as an Object
                SingleStringPropertyParser, // reads a single string-valued property
                ListParser,                 // traverses an RDF list (rdf:first/rdf:rest)
            },
        },
    },
    term::{Object, literal::Lang},
};
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use rudof_rdf::rdf_core::RDFFormat;
use iri_s::IriS;

// The domain type we want to build from the RDF graph.
#[derive(Debug, Clone)]
struct PersonShape {
    id: Object,
    name: String,
    known_langs: Vec,
}

// --- Individual field parsers (reusable building blocks) ---

/// Reads the single string value of sh:name on the current focus node.
fn parse_name() -> impl RDFNodeParse {
    let sh_name = IriS::new_unchecked("http://www.w3.org/ns/shacl#name");
    SingleStringPropertyParser::new(sh_name)
}

/// Reads sh:languageIn ( "en" "fr" ) and returns a Vec.
fn parse_language_in() -> impl RDFNodeParse> {
    let sh_language_in = IriS::new_unchecked("http://www.w3.org/ns/shacl#languageIn");
    ListParser::new()
        .flat_map(|terms: Vec| {
            let langs: Vec = terms.iter().flat_map(RDF::term_as_lang).collect();
            Ok(langs)
        })
        .map_property(sh_language_in)
        .map(|mut vecs| vecs.pop().unwrap_or_default())
}

// --- Composite parser built with the fluent API ---

/// Combines all field parsers into a single `PersonShape`.
fn person_shape_parser() -> impl RDFNodeParse {
    ObjectParser::new()
        .then(move |id: Object| {
            parse_name()
                .and(parse_language_in())
                .flat_map(move |(name, langs)| {
                    Ok(PersonShape {
                        id: id.clone(),
                        name,
                        known_langs: langs,
                    })
                })
        })
}

fn main() {
    let turtle = r#"
        @prefix ex:  <http://example.org/> .
        @prefix sh:  <http://www.w3.org/ns/shacl#> .

        ex:Alice
            sh:name        "Alice" ;
            sh:languageIn  ( "en" "fr" ) .
    "#;

    // 1. Parse a Turtle string into an in-memory RDF graph.
    let graph = InMemoryGraph::from_str(
        turtle,
        &RDFFormat::Turtle,
        None,
        &ReaderMode::default(),
    )
    .expect("Failed to parse Turtle");

    // 2. Wrap the graph in RDFParse, which tracks the mutable focus node.
    let mut rdf_parse = RDFParse::new(graph);

    // 3. Point the focus at the node we want to parse.
    let alice: Object = IriS::new_unchecked("http://example.org/Alice").into();
    rdf_parse.rdf_mut().set_focus(&alice.clone().into());

    // 4. Run the composite parser.
    let person = person_shape_parser()
        .parse_focused(rdf_parse.rdf_mut())
        .expect("Parsing failed");

    println!("Parsed: {:?}", person);
    // Parsed: PersonShape { id: Iri { .. "http://example.org/Alice" },
    //                       name: "Alice",
    //                       known_langs: [Lang { lang: "en" }, Lang { lang: "fr" }] }
}
```

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_rdf).
