# `rudof_lib`

  `rudof_lib` centralizes Rudof functionality for programmatic use. At a high level it supports:

  - Loading and serializing RDF and PG data.
  - Loading, checking, serializing and validating ShEx schemas.
  - Loading, serializing and validating SHACL shapes.
  - Loading, serializing and validating PGSchemas.
  - Loading, running and serializing SPARQL queries and query results.
  - Converting and comparing schemas between supported formats.
  - Loading and serializing DCTAP and Service Descriptions.
  - Generating synthetic data from schemas.

  **Primary goals**:
  - Provide a stable, ergonomic entry point (`Rudof`) that acts as a facade
    over specialized crates.
  - Expose operations via builder-style (fluent) interfaces so integrations
    can compose and reuse behavior safely.
  - Keep a clear separation between the public API, concrete implementations, and shared types/formats/errors.

  ## Architecture and Package Structure

  The package structure is organized to make the public API small and stable
  while keeping implementations modular and testable. Below is a more detailed description of the main files and folders, what to expect inside them and the conventions used by the crate.

  - **src/lib.rs**: crate root and public re-exports (`Rudof`, `RudofConfig`, errors and formats).

  - **src/rudof.rs**: the `Rudof` struct and the builder-returning API.
    This is the façade object: it holds the runtime state (loaded data, current schemas, etc.) and exposes domain-specific methods that return builders (e.g.`load_shex_schema()`, `validate_shacl()`, `run_query()`). The builders delegate to implementation functions located under `src/api/*/implementations`.

  - **src/rudof_config.rs**: configuration handling and defaults (`RudofConfig`).

  - **src/api/**: domain contracts (traits), builders and the bridge to
    implementations. Each domain (for example `data`, `shex`, `shacl`,
    `query`, `conversion`, etc.) contains three subparts:
    - **builders/**: the public builder types (`*Builder`) that callers
      instantiate via `Rudof` methods. Builders configure the operation
      and finally call an implementation function when executed.
    - **implementations/**: concrete operation code that performs the
      logic using `Rudof` and the configured inputs.
    - **trait definition** (in the domain module): describe the
      operations offered.

  - **src/errors/**: domain-specific error types and the unified
    `RudofError`. Errors are defined per-domain and composed into a top-level
    error to provide predictable error handling for callers.

  - **src/formats/**: enums for input/serialization formats.

  - **src/types/**: shared domain types (for example `Data`,
    `QueryResult`, `shex_statistics`, and other cross-cutting structs).

  - **src/utils/**: internal utility helpers.

  - **src/default_config.toml**: embedded TOML configuration used by
    `RudofConfig::new()`.

  - **Tests**: unit tests are located in the implementations submodules
    (`.../implementations/tests`).

  ## Main dependencies

  The following dependencies provide the foundational building blocks that rudof_lib composes to deliver its functionality. By delegating specialized tasks to these well-defined components, the library maintains a clean architecture, promotes reuse, and ensures that each concern remains independently extensible.

  - `dctap`
  - `rudof_iri`
  - `prefixmap`
  - `pgschema`
  - `rdf_config`
  - `rudof_generate`
  - `rudof_rdf`
  - `shacl_ast`
  - `shacl_ir`
  - `shacl_rdf`
  - `shacl_validation`
  - `shapes_comparator`
  - `shapes_converter`
  - `shex_ast`
  - `shex_validation`
  - `sparql_service`
