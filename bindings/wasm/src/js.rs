//! JavaScript API, generated with `wasm-bindgen`. It follows the Python bindings
//! with JavaScript naming: `Rudof.read_shex` becomes `Rudof.readShex`, etc.
//!
//! Optional arguments can be omitted or passed as `undefined`/`null`. Reports
//! are returned as plain objects (see the TypeScript declarations below), and
//! errors are thrown as `Error`s whose `name` is the error category, e.g.
//! `ShExError` or `DataError`.

use crate::{Error, Session, default_config};
use rudof_lib::RudofConfig;
use serde::Serialize;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
/** The result of ShEx validation: one entry per node/shape association. */
export interface ShExValidationReport {
  /** `true` if every entry is conformant (vacuously, if there are none). */
  conforms: boolean;
  entries: ShExValidationEntry[];
  /** The entries that are not conformant. */
  violations: ShExValidationEntry[];
}

export interface ShExValidationEntry {
  node: string;
  shape: string;
  status: "conformant" | "nonconformant" | "pending" | "inconsistent";
  /** The reason or appinfo carried by the status, if any. */
  details: string | null;
}

/** The result of SHACL validation. */
export interface ShaclValidationReport {
  conforms: boolean;
  entries: ShaclValidationEntry[];
  /** Alias of `entries`. */
  violations: ShaclValidationEntry[];
}

export interface ShaclValidationEntry {
  focusNode: string;
  path: string | null;
  value: string | null;
  sourceShape: string | null;
  constraintComponent: string;
  /** `Violation`, `Warning`, `Info`, ... or the IRI of a custom severity. */
  severity: string;
  messages: { text: string; lang: string | null }[];
}

/** The result of a SPARQL query. */
export type QueryResults =
  | { kind: "select"; variables: string[]; rows: (string | null)[][] }
  | { kind: "ask"; boolean: boolean }
  | { kind: "graph"; graph: string };
"#;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        let js_error = js_sys::Error::new(error.message());
        js_error.set_name(error.name());
        js_error.into()
    }
}

/// Converts a report to a plain object (`null` for missing values, arrays for
/// lists and tuples).
fn to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| Error::new("RudofError", e.to_string()).into())
}

/// Configuration of a `Rudof` instance.
#[wasm_bindgen(js_name = RudofConfig)]
pub struct JsRudofConfig {
    inner: RudofConfig,
}

#[wasm_bindgen(js_class = RudofConfig)]
impl JsRudofConfig {
    /// The default configuration. As there is no current directory to derive a
    /// base IRI from, relative IRIs are resolved against rudof's default base
    /// (`http://base`, see `auto_base`) unless a base is given.
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsRudofConfig {
        JsRudofConfig {
            inner: default_config(),
        }
    }

    /// Parses a configuration in TOML, with the same keys as `rudof.toml`,
    /// e.g. `base_iri = "http://example.org/"`.
    #[wasm_bindgen(js_name = fromToml)]
    pub fn from_toml(toml: &str) -> Result<JsRudofConfig, JsValue> {
        let inner = RudofConfig::from_str(toml).map_err(Error::from)?;
        Ok(JsRudofConfig { inner })
    }
}

impl Default for JsRudofConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// A rudof session: loaded data, schemas and results persist across calls
/// until a `reset*` method clears them.
#[wasm_bindgen(js_name = Rudof)]
pub struct JsRudof {
    session: Session,
}

#[wasm_bindgen(js_class = Rudof)]
impl JsRudof {
    /// Creates a session with `config`, or with the default configuration.
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<JsRudofConfig>) -> JsRudof {
        JsRudof {
            session: Session::new(config.map(|c| c.inner)),
        }
    }

    /// Replaces the configuration used by future operations.
    #[wasm_bindgen(js_name = updateConfig)]
    pub fn update_config(&mut self, config: &JsRudofConfig) {
        self.session.update_config(config.inner.clone());
    }

    /// The version of the underlying rudof library.
    #[wasm_bindgen(js_name = getVersion)]
    pub fn get_version(&self) -> String {
        self.session.version()
    }

    // RDF data

    /// Loads RDF data from a string. `format` defaults to `turtle`; with
    /// `merge`, the data is added to the current data instead of replacing it.
    #[wasm_bindgen(js_name = readData)]
    pub fn read_data(
        &mut self,
        data: &str,
        format: Option<String>,
        base: Option<String>,
        #[wasm_bindgen(js_name = "readerMode")] reader_mode: Option<String>,
        merge: Option<bool>,
    ) -> Result<(), JsValue> {
        Ok(self
            .session
            .read_data(data, format.as_deref(), base.as_deref(), reader_mode.as_deref(), merge)?)
    }

    /// Serializes the current RDF data, by default in `compact` format.
    #[wasm_bindgen(js_name = serializeData)]
    pub fn serialize_data(&mut self, format: Option<String>) -> Result<String, JsValue> {
        Ok(self.session.serialize_data(format.as_deref())?)
    }

    // ShEx

    /// Loads a ShEx schema from a string (`shexc` by default).
    #[wasm_bindgen(js_name = readShex)]
    pub fn read_shex(
        &mut self,
        schema: &str,
        format: Option<String>,
        base: Option<String>,
        #[wasm_bindgen(js_name = "readerMode")] reader_mode: Option<String>,
    ) -> Result<(), JsValue> {
        Ok(self
            .session
            .read_shex(schema, format.as_deref(), base.as_deref(), reader_mode.as_deref())?)
    }

    /// Serializes the current ShEx schema, or only the shape `shapeLabel`.
    #[wasm_bindgen(js_name = serializeCurrentShex)]
    pub fn serialize_current_shex(
        &self,
        format: Option<String>,
        #[wasm_bindgen(js_name = "shapeLabel")] shape_label: Option<String>,
    ) -> Result<String, JsValue> {
        Ok(self
            .session
            .serialize_current_shex(format.as_deref(), shape_label.as_deref())?)
    }

    /// Loads a ShapeMap from a string, e.g. `:alice@:Person` or
    /// `{FOCUS :name _}@:Person`.
    #[wasm_bindgen(js_name = readShapemap)]
    pub fn read_shapemap(
        &mut self,
        shapemap: &str,
        format: Option<String>,
        #[wasm_bindgen(js_name = "baseNodes")] base_nodes: Option<String>,
        #[wasm_bindgen(js_name = "baseShapes")] base_shapes: Option<String>,
    ) -> Result<(), JsValue> {
        Ok(self.session.read_shapemap(
            shapemap,
            format.as_deref(),
            base_nodes.as_deref(),
            base_shapes.as_deref(),
        )?)
    }

    /// Serializes the current ShapeMap.
    #[wasm_bindgen(js_name = serializeShapemap)]
    pub fn serialize_shapemap(&self, format: Option<String>) -> Result<String, JsValue> {
        Ok(self.session.serialize_shapemap(format.as_deref())?)
    }

    /// Validates the current data against the ShEx schema, for the
    /// associations in the current ShapeMap.
    #[wasm_bindgen(js_name = validateShex, unchecked_return_type = "ShExValidationReport")]
    pub fn validate_shex(&mut self) -> Result<JsValue, JsValue> {
        to_js(&self.session.validate_shex()?)
    }

    /// Serializes the results of the most recent ShEx validation.
    #[wasm_bindgen(js_name = serializeShexValidationResults)]
    pub fn serialize_shex_validation_results(
        &self,
        format: Option<String>,
        #[wasm_bindgen(js_name = "sortMode")] sort_mode: Option<String>,
    ) -> Result<String, JsValue> {
        Ok(self
            .session
            .serialize_shex_validation_results(format.as_deref(), sort_mode.as_deref())?)
    }

    // SHACL

    /// Loads a SHACL shapes graph from a string (`turtle` by default). Without
    /// `shapes`, the shapes are taken from the current data.
    #[wasm_bindgen(js_name = readShacl)]
    pub fn read_shacl(
        &mut self,
        shapes: Option<String>,
        format: Option<String>,
        base: Option<String>,
        #[wasm_bindgen(js_name = "readerMode")] reader_mode: Option<String>,
    ) -> Result<(), JsValue> {
        Ok(self.session.read_shacl(
            shapes.as_deref(),
            format.as_deref(),
            base.as_deref(),
            reader_mode.as_deref(),
        )?)
    }

    /// Serializes the current SHACL shapes graph.
    #[wasm_bindgen(js_name = serializeShacl)]
    pub fn serialize_shacl(&self, format: Option<String>) -> Result<String, JsValue> {
        Ok(self.session.serialize_shacl(format.as_deref())?)
    }

    /// Validates the current data against the SHACL shapes graph, with the
    /// `native` (default) or `sparql` engine.
    #[wasm_bindgen(js_name = validateShacl, unchecked_return_type = "ShaclValidationReport")]
    pub fn validate_shacl(&mut self, mode: Option<String>) -> Result<JsValue, JsValue> {
        to_js(&self.session.validate_shacl(mode.as_deref())?)
    }

    /// Serializes the results of the most recent SHACL validation.
    #[wasm_bindgen(js_name = serializeShaclValidationResults)]
    pub fn serialize_shacl_validation_results(
        &self,
        format: Option<String>,
        #[wasm_bindgen(js_name = "sortMode")] sort_mode: Option<String>,
    ) -> Result<String, JsValue> {
        Ok(self
            .session
            .serialize_shacl_validation_results(format.as_deref(), sort_mode.as_deref())?)
    }

    // SPARQL

    /// Loads a SPARQL query. Its type is detected when `queryType` is omitted.
    #[wasm_bindgen(js_name = readQuery)]
    pub fn read_query(
        &mut self,
        query: &str,
        #[wasm_bindgen(js_name = "queryType")] query_type: Option<String>,
    ) -> Result<(), JsValue> {
        Ok(self.session.read_query(query, query_type.as_deref())?)
    }

    /// Runs the current query against the current data.
    #[wasm_bindgen(js_name = runQuery, unchecked_return_type = "QueryResults")]
    pub fn run_query(&mut self) -> Result<JsValue, JsValue> {
        to_js(&self.session.run_query()?)
    }

    /// Serializes the results of the most recent query.
    #[wasm_bindgen(js_name = serializeQueryResults)]
    pub fn serialize_query_results(&self, format: Option<String>) -> Result<String, JsValue> {
        Ok(self.session.serialize_query_results(format.as_deref())?)
    }

    // Prefixes

    /// The default prefixes, as `[alias, iri]` pairs.
    #[wasm_bindgen(unchecked_return_type = "[string, string][]")]
    pub fn prefixes(&self) -> Result<JsValue, JsValue> {
        to_js(&self.session.prefixes())
    }

    #[wasm_bindgen(js_name = addPrefix)]
    pub fn add_prefix(&mut self, alias: &str, iri: &str) -> Result<(), JsValue> {
        Ok(self.session.add_prefix(alias, iri)?)
    }

    #[wasm_bindgen(js_name = removePrefix)]
    pub fn remove_prefix(&mut self, alias: &str) -> Result<(), JsValue> {
        Ok(self.session.remove_prefix(alias)?)
    }

    #[wasm_bindgen(js_name = renamePrefix)]
    pub fn rename_prefix(
        &mut self,
        #[wasm_bindgen(js_name = "oldAlias")] old_alias: &str,
        #[wasm_bindgen(js_name = "newAlias")] new_alias: &str,
    ) -> Result<(), JsValue> {
        Ok(self.session.rename_prefix(old_alias, new_alias)?)
    }

    #[wasm_bindgen(js_name = copyPrefix)]
    pub fn copy_prefix(
        &mut self,
        #[wasm_bindgen(js_name = "oldAlias")] old_alias: &str,
        #[wasm_bindgen(js_name = "newAlias")] new_alias: &str,
    ) -> Result<(), JsValue> {
        Ok(self.session.copy_prefix(old_alias, new_alias)?)
    }

    // Resets

    /// Clears all the state: data, schemas, queries and results.
    #[wasm_bindgen(js_name = resetAll)]
    pub fn reset_all(&mut self) {
        self.session.reset_all();
    }

    #[wasm_bindgen(js_name = resetData)]
    pub fn reset_data(&mut self) {
        self.session.reset_data();
    }

    /// Unloads the ShEx schema, ShapeMap and ShEx validation results.
    #[wasm_bindgen(js_name = resetShex)]
    pub fn reset_shex(&mut self) {
        self.session.reset_shex();
    }

    /// Unloads only the ShEx schema.
    #[wasm_bindgen(js_name = resetShexSchema)]
    pub fn reset_shex_schema(&mut self) {
        self.session.reset_shex_schema();
    }

    #[wasm_bindgen(js_name = resetShapemap)]
    pub fn reset_shapemap(&mut self) {
        self.session.reset_shapemap();
    }

    /// Unloads only the SHACL shapes graph.
    #[wasm_bindgen(js_name = resetShacl)]
    pub fn reset_shacl(&mut self) {
        self.session.reset_shacl();
    }

    /// Unloads the SHACL shapes graph and SHACL validation results.
    #[wasm_bindgen(js_name = resetShaclValidation)]
    pub fn reset_shacl_validation(&mut self) {
        self.session.reset_shacl_validation();
    }

    #[wasm_bindgen(js_name = resetQuery)]
    pub fn reset_query(&mut self) {
        self.session.reset_query();
    }

    #[wasm_bindgen(js_name = resetQueryResults)]
    pub fn reset_query_results(&mut self) {
        self.session.reset_query_results();
    }
}

/// Validates RDF data against a ShExC schema for the associations in a
/// ShapeMap, in one call.
#[wasm_bindgen(js_name = validateShex, unchecked_return_type = "ShExValidationReport")]
pub fn validate_shex(
    data: &str,
    schema: &str,
    shapemap: &str,
    #[wasm_bindgen(js_name = "dataFormat")] data_format: Option<String>,
    base: Option<String>,
) -> Result<JsValue, JsValue> {
    to_js(&crate::validate_shex(
        data,
        schema,
        shapemap,
        data_format.as_deref(),
        base.as_deref(),
    )?)
}

/// Validates RDF data against a SHACL shapes graph, with the `native`
/// (default) or `sparql` engine, in one call.
#[wasm_bindgen(js_name = validateShacl, unchecked_return_type = "ShaclValidationReport")]
pub fn validate_shacl(
    data: &str,
    shapes: &str,
    #[wasm_bindgen(js_name = "dataFormat")] data_format: Option<String>,
    #[wasm_bindgen(js_name = "shapesFormat")] shapes_format: Option<String>,
    base: Option<String>,
    mode: Option<String>,
) -> Result<JsValue, JsValue> {
    to_js(&crate::validate_shacl(
        data,
        shapes,
        data_format.as_deref(),
        shapes_format.as_deref(),
        base.as_deref(),
        mode.as_deref(),
    )?)
}
