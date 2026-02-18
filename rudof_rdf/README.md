<div align="center">

# rudof_rdf

</div>

## 🚀 Overview

The `rudof_rdf` crate is a core component of the Rudof project, providing foundational data structures, utilities, and algorithms for working with RDF (Resource Description Framework) data in Rust. It is designed to support parsing, querying, manipulating, and visualizing RDF graphs, and serves as the backbone for higher-level crates in the Rudof ecosystem.

## 📦 Architecture and Package Structure

The crate is organized into several key modules:

- `rdf_core`: Core logic for RDF handling, including:
  - `term`: RDF terms (IRIs, blank nodes, literals, triples)
  - `parser`: Parsers for RDF nodes and documents
  - `query`: SPARQL query support and result handling
  - `vocab`: Common RDF, RDFS, XSD, and SHACL vocabulary constants
  - `utils`: Utilities such as regex helpers
  - `visualizer`: Tools for visualizing RDF graphs (UML, styles, etc.)
  - `matcher`, `focus_rdf`, `neighs_rdf`, etc.: Advanced graph navigation and matching
- rdf_impl: Implementations of RDF storage and access:
  - `in_memory_graph`: In-memory RDF graph implementation
  - `sparql_endpoint`: SPARQL endpoint integration
  - `oxrdf_impl`: Integration with the oxrdf crate

## 📚 Dependencies

### Main Dependencies

The `rudof_rdf` crate depends on several key libraries, both internal and external:

- Main Internal Rudof crates dependecies:
  - `iri_s`: IRI handling
  - `prefixmap`: Prefix mapping for compact IRI representation
- Main External dependencies:
  - `oxigraph`, `oxrdf`, `oxjsonld`, `oxiri`, `oxilangtag`, `oxrdfio`, `oxrdfxml`, `oxsdatatypes`, `oxttl`: Libraries for RDF parsing, serialization, and datatype support
  - `reqwest`: HTTP client for remote data access
  - `tokio`: Asynchronous runtime

### Crates That Depend on `rudof_rdf`
The `rudof_rdf` crate is a foundational dependency for many other Rudof crates:

- `rudof_lib`: The main library crate for Rudof, which re-exports and builds upon rdf for higher-level features.
- `rudof_cli`: The command-line interface for Rudof, enabling users to interact with RDF data and perform validation, conversion, and querying.
- `shacl_ast`, `shacl_ir`, `shacl_rdf`, `shacl_validation`: SHACL (Shapes Constraint Language) support.
- `shex_ast`, `shex_validation`: ShEx (Shape Expressions) support.
- `shex_testsuite`, `shapes_comparator`, `shapes_converter`, `sparql_service`, and others.

## 🛠️ Usage
The `rudof_rdf` crate is not typically used directly by end-users, but rather as a building block for higher-level libraries and applications in the Rudof project. It provides the essential types and traits for representing RDF graphs, parsing and serializing RDF data, executing SPARQL queries, and integrating with other semantic web technologies.
