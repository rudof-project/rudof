//! # rudof_wasm
//!
//! WebAssembly bindings for rudof, built on [`rudof_lib`]. They mirror the
//! Python bindings: a stateful [`Session`] (the `Rudof` class in JavaScript)
//! loads RDF data, ShEx schemas, ShapeMaps, SHACL shapes and SPARQL queries,
//! validates, queries and serializes them. [`validate_shex`] and
//! [`validate_shacl`] do a whole validation in one call, and [`generate`]
//! generates synthetic data from a schema.
//!
//! Everything here is plain Rust, so it can be used and tested natively. On
//! `wasm` targets the `js` module exports it to JavaScript with `wasm-bindgen`.
//!
//! Inputs are passed as strings: anything that needs a filesystem, network
//! access or native tools (reading files, fetching URLs, dereferencing IRIs,
//! rendering images) is not available on `wasm`. Remote SPARQL endpoints and
//! ShEx `IMPORT`s are, with the `endpoints` feature, through an asynchronous
//! API that relies on JSPI (see `js_endpoints`).

mod error;
mod reports;
mod session;

#[cfg(target_family = "wasm")]
pub mod js;
#[cfg(all(target_family = "wasm", feature = "endpoints"))]
pub mod js_endpoints;

pub use error::{Error, Result};
#[cfg(feature = "generate")]
pub use reports::GeneratedData;
pub use reports::{
    ExternalResolver, GraphDiff, NeighborArc, NodeNeighborhood, QueryResults, ShExCheck, ShExValidationEntry,
    ShExValidationReport, ShaclMessage, ShaclValidationEntry, ShaclValidationReport,
};
#[cfg(feature = "pgschema")]
pub use reports::{PgSchemaValidationEntry, PgSchemaValidationReport};
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
    session.validate_shex(false, None)
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
    session.validate_shacl(mode, None)
}

/// Generates synthetic data from a `schema` given as text, in `schema_format`
/// (`shex`, `shacl` or `pgschema`): RDF (Turtle) and the ShapeMap of every
/// generated node, and from a PG-Schema the property graph and its type map.
///
/// * `config` - a generator configuration in TOML (`version = 2`, the format of
///   `rudof generate --generator-config`); its `[output]` section is ignored.
///   `[violations]` adds the reports of what a validator must flag, and
///   `[targets]` asks for a coherence.
/// * `entities` / `seed` - override the configuration; 20 entities without either.
#[cfg(feature = "generate")]
pub fn generate(
    schema: &str,
    schema_format: &str,
    config: Option<&str>,
    entities: Option<usize>,
    seed: Option<u64>,
) -> Result<GeneratedData> {
    use rudof_generate::{DataGenerator, GeneratorConfig, SchemaFormat};
    let generate_error = |e: rudof_generate::errors::DataGeneratorError| Error::new("GenerateError", e.to_string());
    let format = match schema_format.to_ascii_lowercase().as_str() {
        "shex" | "shexc" => SchemaFormat::ShEx,
        "shacl" => SchemaFormat::Shacl,
        "pgschema" | "pg-schema" | "pgs" => SchemaFormat::PgSchema,
        other => {
            return Err(Error::invalid_argument(format!(
                "unknown schema format `{other}` (use shex, shacl or pgschema)"
            )));
        },
    };
    let mut generator_config = match config {
        Some(toml) => GeneratorConfig::from_toml_str(toml).map_err(generate_error)?,
        None => {
            let mut c = GeneratorConfig::default();
            c.generation.entity_count = 20;
            c
        },
    };
    if let Some(n) = entities {
        generator_config.generation.entity_count = n;
    }
    if seed.is_some() {
        generator_config.generation.seed = seed;
    }
    let mut generator = DataGenerator::new(generator_config).map_err(generate_error)?;
    let generated = futures::executor::block_on(async {
        generator.load_schema_str(schema, format).await?;
        generator.generate_in_memory().await
    })
    .map_err(generate_error)?;
    Ok(GeneratedData::from(generated))
}
