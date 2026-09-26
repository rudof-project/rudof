//! # rudof_wasm
//!
//! WebAssembly bindings for rudof, built on [`rudof_lib`]. They mirror the
//! Python bindings: a stateful [`Session`] (the `Rudof` class in JavaScript)
//! loads RDF data, ShEx schemas, ShapeMaps, SHACL shapes and SPARQL queries,
//! validates, queries and serializes them. [`validate_shex`] and
//! [`validate_shacl`] do a whole validation in one call.
//!
//! Everything here is plain Rust, so it can be used and tested natively. On
//! `wasm` targets the `js` module exports it to JavaScript with `wasm-bindgen`.
//!
//! Inputs are passed as strings: anything that needs a filesystem, network
//! access or native tools (reading files, fetching URLs, dereferencing IRIs,
//! ShEx `IMPORT`s, remote SPARQL endpoints, rendering images) is not available
//! on `wasm`.

mod error;
mod reports;
mod session;

#[cfg(target_family = "wasm")]
pub mod js;

pub use error::{Error, Result};
pub use reports::{
    ExternalResolver, NeighborArc, NodeNeighborhood, PgSchemaValidationEntry, PgSchemaValidationReport, QueryResults,
    ShExCheck, ShExValidationEntry, ShExValidationReport, ShaclMessage, ShaclValidationEntry, ShaclValidationReport,
};
pub use session::Session;

use rudof_lib::RudofConfig;
use rudof_lib::config::CommonConfig;

/// The configuration used when none is given: rudof's defaults, with
/// `auto_base` on. There is no current directory to derive a base IRI from on
/// `wasm`, so relative IRIs are resolved against rudof's default base
/// (`http://base`) unless a base is given.
pub fn default_config() -> RudofConfig {
    let mut config = RudofConfig::default().with_common(CommonConfig::default().with_auto_base(true));
    config.resolve();
    config
}

/// Validates RDF `data` against a ShEx `schema` (ShExC) for the node/shape
/// associations in `shapemap` (e.g. `:alice@:Person`, or with query selectors
/// such as `{FOCUS :name _}@:Person`).
///
/// * `data_format` - RDF format of `data` (`turtle`, `ntriples`, `rdfxml`,
///   `trig`, `n3`, `nquads`, `jsonld`); defaults to Turtle.
/// * `base` - base IRI used to resolve relative IRIs in the data, the schema
///   and the shapemap.
pub fn validate_shex(
    data: &str,
    schema: &str,
    shapemap: &str,
    data_format: Option<&str>,
    base: Option<&str>,
) -> Result<ShExValidationReport> {
    let mut session = Session::new(None);
    session.read_data(data, data_format, base, None, None)?;
    session.read_shex(schema, None, base, None)?;
    session.read_shapemap(shapemap, None, base, base)?;
    session.validate_shex()
}

/// Validates RDF `data` against a SHACL shapes graph `shapes`, with the
/// `native` (default) or `sparql` engine. SHACL-SPARQL constraints
/// (`sh:sparql`) are supported in both modes.
///
/// * `data_format` / `shapes_format` - RDF formats of `data` and `shapes`
///   (`turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, `nquads`, `jsonld`);
///   default to Turtle.
/// * `base` - base IRI used to resolve relative IRIs in data and shapes.
pub fn validate_shacl(
    data: &str,
    shapes: &str,
    data_format: Option<&str>,
    shapes_format: Option<&str>,
    base: Option<&str>,
    mode: Option<&str>,
) -> Result<ShaclValidationReport> {
    let mut session = Session::new(None);
    session.read_data(data, data_format, base, None, None)?;
    session.read_shacl(Some(shapes), shapes_format, base, None)?;
    session.validate_shacl(mode)
}
