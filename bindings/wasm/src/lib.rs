//! # rudof_wasm
//!
//! WebAssembly bindings for rudof, exposing ShEx and SHACL validation of RDF
//! data held in memory.
//!
//! The functions in this module are plain Rust, so they can be used and tested
//! natively. On `wasm` targets they are also exported to JavaScript (see the
//! `js` module) as `validateShex` and `validateShacl`, which return the
//! validation results as a JSON string and throw an `Error` on failure.
//!
//! Anything that needs a filesystem or network access (reading files,
//! dereferencing IRIs, resolving ShEx `IMPORT`s, SPARQL endpoints) is not
//! supported: all inputs are passed as strings.

use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use serde_json::{Value, json};
use shacl::ir::IRSchema;
use shacl::validator::processor::{GraphValidation, ShaclProcessor};
use shacl::validator::report::ValidationReport;
use shacl::validator::store::Graph;
use shacl::validator::{ShaclConfig, ShaclValidationMode};
use shex_ast::ResolveMethod;
use shex_ast::compact::{ShExParser, ShapeMapParser};
use shex_ast::ir::actions::semantic_actions_registry::SemanticActionsRegistry;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_validation::{Validator, ValidatorConfig};
use std::str::FromStr;

/// IRI identifying a ShEx schema passed as a string, used when no base IRI is given.
const DEFAULT_SCHEMA_SOURCE: &str = "urn:rudof-wasm:schema";

/// Validates RDF `data` against a ShEx `schema` in ShExC syntax, for the
/// node/shape associations in `shapemap` (ShapeMap compact syntax, e.g.
/// `:alice@:Person`).
///
/// * `data_format` - RDF format of `data` (`turtle`, `ntriples`, `rdfxml`,
///   `trig`, `n3`, `nquads`, `jsonld`); defaults to Turtle.
/// * `base` - base IRI used to resolve relative IRIs in the data, the schema
///   and the shapemap.
///
/// Returns a JSON object `{ "conforms": bool, "results": [...] }`, where each
/// result has the `node`, `shape`, `status` (`conformant`/`nonconformant`),
/// `reason` and `appInfo` of one association.
pub fn validate_shex(
    data: &str,
    schema: &str,
    shapemap: &str,
    data_format: Option<&str>,
    base: Option<&str>,
) -> Result<String, String> {
    let base = parse_base(base)?;
    let rdf = parse_data(data, data_format, base.as_ref())?;

    let source_iri = base
        .clone()
        .unwrap_or_else(|| IriS::new_unchecked(DEFAULT_SCHEMA_SOURCE));
    let schema =
        ShExParser::parse(schema, base.clone(), &source_iri).map_err(|e| format!("Error parsing ShEx schema: {e}"))?;
    let config = ValidatorConfig::default();
    let mut schema_ir = SchemaIR::new(SemanticActionsRegistry::default());
    schema_ir
        .populate_from_schema_json(&schema, config.external_resolvers(), &ResolveMethod::default(), &base)
        .map_err(|e| format!("Error compiling ShEx schema: {e}"))?;
    let validator = Validator::new(&schema_ir, &config).map_err(|e| format!("Error creating ShEx validator: {e}"))?;

    let nodes_prefixmap: Option<PrefixMap> = Some(rdf.prefixmap().clone());
    let shapes_prefixmap = Some(schema_ir.prefixmap());
    let shapemap = ShapeMapParser::parse(shapemap, &nodes_prefixmap, &base, &shapes_prefixmap, &base)
        .map_err(|e| format!("Error parsing shapemap: {e}"))?;

    let result = validator
        .validate_shapemap(&shapemap, &rdf, &schema_ir, &nodes_prefixmap)
        .map_err(|e| format!("Error during ShEx validation: {e}"))?;

    let conforms = result.iter().all(|(_, _, status)| status.is_conformant());
    let results = serde_json::to_value(&result).map_err(|e| format!("Error serializing ShEx results: {e}"))?;
    Ok(json!({ "conforms": conforms, "results": results }).to_string())
}

/// Validates RDF `data` against a SHACL shapes graph `shapes`, using the
/// native SHACL engine.
///
/// * `data_format` / `shapes_format` - RDF formats of `data` and `shapes`
///   (`turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, `nquads`, `jsonld`);
///   default to Turtle.
/// * `base` - base IRI used to resolve relative IRIs in data and shapes.
///
/// Returns a JSON object `{ "conforms": bool, "results": [...] }`, where each
/// result has the `focusNode`, `path`, `value`, `sourceShape`,
/// `constraintComponent`, `severity` and `messages` (a list of
/// `{ "text", "lang" }`) of one validation result.
pub fn validate_shacl(
    data: &str,
    shapes: &str,
    data_format: Option<&str>,
    shapes_format: Option<&str>,
    base: Option<&str>,
) -> Result<String, String> {
    let base = parse_base(base)?;
    let rdf = parse_data(data, data_format, base.as_ref())?;
    let shapes_format = parse_format(shapes_format)?;
    let schema = IRSchema::from_str(
        shapes,
        &shapes_format,
        base.as_ref().map(IriS::as_str),
        &ReaderMode::default(),
    )
    .map_err(|e| format!("Error parsing SHACL shapes: {e}"))?;

    // Infallible when `shacl` is built without `sparql` (as this crate asks
    // for), but fallible when a native workspace build unifies that feature in.
    #[allow(clippy::unnecessary_fallible_conversions)]
    let graph = Graph::try_from(rdf).map_err(|e| format!("Error preparing RDF data: {e}"))?;
    let report = GraphValidation::new(graph)
        .validate(&schema, &ShaclValidationMode::Native, &ShaclConfig::default())
        .map_err(|e| format!("Error during SHACL validation: {e}"))?;

    Ok(shacl_report_to_json(&report).to_string())
}

fn shacl_report_to_json(report: &ValidationReport) -> Value {
    let results: Vec<Value> = report
        .results()
        .iter()
        .map(|result| {
            let mut messages: Vec<Value> = result
                .message()
                .iter()
                .map(|(lang, text)| json!({ "text": text, "lang": lang.as_ref().map(ToString::to_string) }))
                .collect();
            messages.sort_by_key(|m| m["lang"].as_str().map(str::to_string));
            json!({
                "focusNode": result.focus_node().to_string(),
                "path": result.path().map(ToString::to_string),
                "value": result.value().map(ToString::to_string),
                "sourceShape": result.source().map(ToString::to_string),
                "constraintComponent": result.constraint_component().to_string(),
                "severity": result.severity().to_string(),
                "messages": messages,
            })
        })
        .collect();
    json!({ "conforms": report.conforms(), "results": results })
}

fn parse_base(base: Option<&str>) -> Result<Option<IriS>, String> {
    base.map(|b| IriS::from_str(b).map_err(|e| format!("Invalid base IRI '{b}': {e}")))
        .transpose()
}

fn parse_format(format: Option<&str>) -> Result<RDFFormat, String> {
    format.map_or(Ok(RDFFormat::Turtle), |f| {
        RDFFormat::from_str(f).map_err(|e| format!("Unsupported RDF format '{f}': {e}"))
    })
}

fn parse_data(data: &str, format: Option<&str>, base: Option<&IriS>) -> Result<OxigraphInMemory, String> {
    let format = parse_format(format)?;
    OxigraphInMemory::from_str(data, &format, base.map(IriS::as_str), &ReaderMode::default())
        .map_err(|e| format!("Error parsing RDF data: {e}"))
}

/// JavaScript API, generated with `wasm-bindgen`.
#[cfg(target_family = "wasm")]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    fn start() {
        console_error_panic_hook::set_once();
    }

    /// Validates RDF data against a ShExC schema for the associations in a
    /// ShapeMap. Returns the results as a JSON string.
    #[wasm_bindgen(js_name = validateShex)]
    pub fn validate_shex(
        data: &str,
        schema: &str,
        shapemap: &str,
        data_format: Option<String>,
        base: Option<String>,
    ) -> Result<String, JsError> {
        super::validate_shex(data, schema, shapemap, data_format.as_deref(), base.as_deref())
            .map_err(|e| JsError::new(&e))
    }

    /// Validates RDF data against a SHACL shapes graph. Returns the
    /// validation report as a JSON string.
    #[wasm_bindgen(js_name = validateShacl)]
    pub fn validate_shacl(
        data: &str,
        shapes: &str,
        data_format: Option<String>,
        shapes_format: Option<String>,
        base: Option<String>,
    ) -> Result<String, JsError> {
        super::validate_shacl(
            data,
            shapes,
            data_format.as_deref(),
            shapes_format.as_deref(),
            base.as_deref(),
        )
        .map_err(|e| JsError::new(&e))
    }
}
