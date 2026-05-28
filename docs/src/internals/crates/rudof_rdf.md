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
  - `oxigraph`: Oxigraph-based backends
    - `in_memory`: In-memory RDF graph implementation (`OxigraphInMemory`)
    - `endpoint`: SPARQL endpoint integration (`OxigraphEndpoint`)
    - `oxrdf_impl`: Integration with the `oxrdf` crate
  - `qlever`: Locally-launched QLever Docker container backend (`QleverGraphContainer`)

## Dependents and dependencies

This create depends mostly on:

- Internal Rudof crates:
  - [`rudof_iri`](./rudof_iri.md)
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
- External (only when the `qlever` feature is enabled):
  - `testcontainers`
  - `bollard`
  - `nix`
  - `futures`
  - `tracing`

This crate is a foundational dependency for many other Rudof crates, including:

- [`rudof_lib`](./rudof_lib.md)
- [`rudof_cli`](./rudof_cli.md)
- [`shacl_ast`](./shacl_ast.md), [`shacl_ir`](./shacl_ir.md), [`shacl_rdf`](./shacl_rdf.md), [`shacl_validation`](./shacl_validation.md)
- [`shex_ast`](./shex_ast.md), [`shex_validation`](./shex_validation.md)
- [`shex_testsuite`](./shex_testsuite.md), [`shapes_comparator`](./shapes_comparator.md), [`shapes_converter`](./shapes_converter.md), [`sparql_service`](./sparql_service.md), and others.

## Cargo features

| Feature              | Pulls in                                                                            | Notes                                                                                                                            |
|----------------------|-------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| `sparql` (default)   | `oxigraph`, `reqwest`, `tokio`, `spargebra`, `sparesults`                           | Enables `OxigraphEndpoint` (remote SPARQL client).                                                                               |
| `qlever`             | implies `sparql`; adds `testcontainers`, `bollard`, `nix`, `futures`, `tracing`     | Enables the `qlever` submodule and re-exports (`QleverGraphContainer`, `QleverConfig`, `QleverError`, …).                        |
| `qlever-docker-tests`| implies `qlever`                                                                    | Gates the Docker-dependent integration tests under `rdf_impl/tests/qlever_docker.rs`. Compiles without Docker; running needs it. |

The `qlever` family of features is gated on `cfg(not(target_family = "wasm"))`.

## Backends in `rdf_impl`

Every backend exposes the same trait surface (`Rdf`, `NeighsRDF`, `QueryRDF`, `FocusRDF`, `BuildRDF`, `AsyncRDF`), so higher-level code can swap one for another with minimal awareness of where the data actually lives.

### `oxigraph` (`OxigraphInMemory` and `OxigraphEndpoint`)

Both backends live under `rdf_impl/oxigraph/`.

- `OxigraphInMemory`: `oxrdf::Graph` plus an optional Oxigraph `Store` for SPARQL evaluation. Default backend everywhere.
- `OxigraphEndpoint`: read-only client for a remote SPARQL endpoint. Caches HTTP clients per `QueryResultFormat`.

The `oxrdf_impl` submodule provides the `oxrdf`-typed implementations of the `rdf_core` traits that both Oxigraph backends share.

### `qlever` (`QleverGraphContainer`)

Available when the `qlever` feature is enabled (and on non-WASM targets). The backend wraps a locally-launched `QLever` Docker container and exposes it as just another `Rdf` implementation. From the caller's perspective it is interchangeable with `OxigraphInMemory`: it produces the same `oxrdf` types and implements the same trait set, but the data lives in a QLever index on disk and is queried via the container's HTTP SPARQL endpoint.

#### Module layout (`rdf_impl/qlever/`)

| File                | What it owns                                                                                                                                                              |
|---------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `mod.rs`            | Public re-exports. The container, config, errors, and the `IndexHandle` / `CliKind` helpers all surface here.                                                             |
| `config.rs`         | `QleverConfig`, `InputFile`, `NativeFormat`. The config maps 1:1 onto QLever's `IndexBuilderMain` / `ServerMain` flags (`-m`, `-c`, `-e`, `-j`, `-P`, `-T`, …).           |
| `cli_probe.rs`      | Detects whether the running image exposes the v1 (`IndexBuilderMain` / `ServerMain`) or v2 (`qlever-index` / `qlever-server`) CLI. Also pings Docker and pulls the image. |
| `index_builder.rs`  | One-shot `bollard` invocations that build (or skip building) the on-disk index. Implements `IndexHandle::is_built` (checks for `<name>.meta`) and `convert_to_ntriples`.  |
| `server.rs`         | `QleverServer`, long-running container managed via `testcontainers-rs`. Owns port mapping and the HTTP readiness probe.                                                  |
| `graph_container.rs`| `QleverGraphContainer`, the public façade. Composes an `OxigraphEndpoint` pointed at the container so the `NeighsRDF` / `QueryRDF` impls are just SPARQL-over-HTTP.      |
| `error.rs`          | `QleverError` covering pre-flight, Docker, container, HTTP, and format-conversion errors.                                                                                 |

Key choices:

- **Idempotent indexing.** The index dir holds a `<name>.meta` marker file. `IndexHandle::is_built()` checks for it before re-running QLever, so repeated `rudof` invocations skip indexing.
- **Multi-file support.** The primary constructor is `from_paths(paths, format, config)`: it accepts any number of file-system paths and feeds them to QLever's `IndexBuilderMain` in a single pass (one `-f / -F / -g` triple per file). `from_path` and `from_reader` are thin shims over `from_paths`.
- **Optional explicit format.** When `from_paths` is called with `Some(&RDFFormat)`, that format overrides the per-file extension sniffing — useful for inputs with non-canonical extensions. When `None`, format is guessed per path.
- **Format coverage by transparent conversion.** QLever's `IndexBuilderMain` only accepts `ttl` / `nt` / `nq` natively (`NativeFormat`). Anything else (RDF/XML, JSON-LD, TriG, N3) is streamed through `oxrdfio` into N-Triples written under a shared conversion dir (fingerprinted from the input paths), then handed to QLever.
- **Read-only.** `BuildRDF::add_triple`, `remove_triple`, `add_type` and `add_bnode` all return `QleverError::ReadOnly`. `BuildRDF::empty()` panics by the moment.
- **Sync trait surface, async work underneath.** Methods on `Rdf` / `NeighsRDF` / `QueryRDF` are synchronous, but the heavy lifting (Docker, HTTP) is async. `QleverGraphContainer` therefore exposes async constructors (`from_paths`, `from_path`, `from_reader`, `open`) and async variants of the SPARQL methods (`query_select_async`, `query_construct_async`, `query_ask_async`) in addition to the trait-required sync methods, which delegate through the shared `OxigraphEndpoint`.
- **Resource ownership.** The `server` field is wrapped in `Arc` so clones cheaply share both the container and the HTTP keep-alive pool. `Drop` tears down the container automatically (via `testcontainers`); the on-disk index is removed only when `auto_delete_if_created` was set and this run actually created it.

#### Configuring via TOML

Setting `[qlever]` in the rudof config TOML deserializes into a `QleverConfig`. The struct is exposed through `RdfDataConfig::qlever` and is only present when the `qlever` feature is compiled in.

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
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use rudof_rdf::rdf_core::RDFFormat;
use rudof_iri::IriS;

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
    let graph = OxigraphInMemory::from_str(
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
