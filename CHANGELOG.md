# CHANGE LOG

## Current changes without release yet

## [0.0.13] - 2024-06-22

- First attempt to add support for ShEx to SPARQL converter, issue #67

## [0.0.12] - 2024-06-17

- Changed CLI name from `sx` to `rdfsx`
- First attempt to added basic support for DCTap
- Code cleaned with Rustfmt and Clippy by [MarcAntoine-Arnaud](https://github.com/MarcAntoine-Arnaud).

## [0.0.11] - 2024-06-08

- This version in mainly a maintainance version updating some dependencies
- Started project DCTAP to handle DCTAP files
- Updated some dependency versions
  - oxrdf = "0.2.0-alpha.2"
  - regex = "1.10.4"

## [0.0.10] - 2024-01-29

- [issue 32](https://github.com/weso/shapes-rs/issues/32) ShEx parser works as an iterator per statement allowing to show debug information by statement. Debug information can be controlled by the environment variablt RUST_LOG. A value of "debug" for that variable will print more information.
- Updated dependency versions
    oxrdf = "0.2.0-alpha.2"
    oxttl = "0.1.0-alpha.2"
    oxrdfio = "0.1.0-alpha.2"

## [0.0.9] - 2024-01-19

- Removed `shex_pest`, `shex_antlr` and `validation_oxgraph` folders because their code is no longer used.
- Added time option to `sx_cli`
- Repaired bug in `shex_compact` that failed with node constraints followed by cardinality without space
- More support to read SHACL as RDF
- Merged [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf), the former crates will no longer be maintained as their code is integrated in `srdf`.
- Added option `--output` to CLI so the users can choose if the output goes to terminal or to a file
- Changed dependency from [rio_api](https://crates.io/crates/rio) and [rio_turtle](https://crates.io/crates/rio_turtle) to [oxttl](https://crates.io/crates/oxttl) and [oxrdfio](https://crates.io/crates/oxrdfio) which seem to be more actively maintained now.

## [0.0.7] - 2024-01-07

In this release we added support for SHACL by defining the [`shacl_ast`](https://crates.io/crates/shacl_ast) crate.

Other changes:

- Renamed the project from shex_rs to shapes_rs to indicate that the project intends to support both ShEx and SHACL.
- Merged the [srdf_graph](https://crates.io/crates/srdf_graph) and [srdf_sparql](https://crates.io/crates/srdf_sparql) crates into [srdf](https://crates.io/crates/srdf).
- Added more combinators and documentation examples to rdf_parser in order to document the RDF parser combinators approach. See, for example, the doc for the [map method](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFNodeParse.html#method.map).
