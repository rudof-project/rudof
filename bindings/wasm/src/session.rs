//! A stateful rudof session, wrapping [`rudof_lib::Rudof`] with the API of the
//! Python bindings: loaded data, schemas and results persist across calls until
//! a `reset_*` method clears them.
//!
//! Formats and modes are passed as strings (e.g. `"turtle"`), parsed with the
//! `FromStr` implementations of `rudof_lib::formats`. Inputs are strings: files,
//! URLs and SPARQL endpoints are not available on `wasm`.

use crate::error::{Error, Result};
use crate::reports::{QueryResults, ShExValidationReport, ShaclValidationReport};
use rudof_lib::formats::{
    DataFormat, DataReaderMode, InputSpec, QueryType, ResultDataFormat, ResultQueryFormat, ResultShExValidationFormat,
    ResultShaclValidationFormat, ShExFormat, ShExValidationSortByMode, ShaclFormat, ShaclValidationMode,
    ShaclValidationSortByMode, ShapeMapFormat,
};
use rudof_lib::{Rudof, RudofConfig};
use std::fmt::Display;
use std::str::FromStr;

pub struct Session {
    rudof: Rudof,
}

impl Session {
    /// Creates a session with the given configuration, or with
    /// [`default_config`](crate::default_config).
    pub fn new(config: Option<RudofConfig>) -> Self {
        Session {
            rudof: Rudof::new(config.unwrap_or_else(crate::default_config)),
        }
    }

    /// Replaces the configuration used by future operations.
    pub fn update_config(&mut self, config: RudofConfig) {
        self.rudof.update_config(config).execute();
    }

    /// The version of the underlying rudof library.
    pub fn version(&self) -> String {
        self.rudof.version().execute().to_string()
    }

    // ------------------------------------------------------------------------
    // RDF data
    // ------------------------------------------------------------------------

    /// Loads RDF data from a string. With `merge`, it is added to the current
    /// data instead of replacing it.
    pub fn read_data(
        &mut self,
        data: &str,
        format: Option<&str>,
        base: Option<&str>,
        reader_mode: Option<&str>,
        merge: Option<bool>,
    ) -> Result<()> {
        let input = [InputSpec::str(data)];
        let format: Option<DataFormat> = parse(format, "data format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        let mut b = self.rudof.load_data().with_data(&input);
        if let Some(f) = &format {
            b = b.with_data_format(f);
        }
        if let Some(base) = base {
            b = b.with_base(base);
        }
        if let Some(m) = &reader_mode {
            b = b.with_reader_mode(m);
        }
        if let Some(merge) = merge {
            b = b.with_merge(merge);
        }
        Ok(b.execute()?)
    }

    /// Serializes the current RDF data.
    pub fn serialize_data(&mut self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultDataFormat> = parse(format, "result data format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_data(w);
            if let Some(f) = &format {
                s = s.with_result_data_format(f);
            }
            s.execute()
        })
    }

    // ------------------------------------------------------------------------
    // ShEx
    // ------------------------------------------------------------------------

    /// Loads a ShEx schema from a string.
    pub fn read_shex(
        &mut self,
        schema: &str,
        format: Option<&str>,
        base: Option<&str>,
        reader_mode: Option<&str>,
    ) -> Result<()> {
        let input = InputSpec::str(schema);
        let format: Option<ShExFormat> = parse(format, "ShEx format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        let mut b = self.rudof.load_shex_schema(&input);
        if let Some(f) = &format {
            b = b.with_shex_schema_format(f);
        }
        if let Some(base) = base {
            b = b.with_base(base);
        }
        if let Some(m) = &reader_mode {
            b = b.with_reader_mode(m);
        }
        Ok(b.execute()?)
    }

    /// Serializes the current ShEx schema, or only the shape `shape_label`.
    pub fn serialize_current_shex(&self, format: Option<&str>, shape_label: Option<&str>) -> Result<String> {
        let format: Option<ShExFormat> = parse(format, "ShEx format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_shex_schema(w);
            if let Some(f) = &format {
                s = s.with_result_shex_format(f);
            }
            if let Some(l) = shape_label {
                s = s.with_shape(l);
            }
            s.execute()
        })
    }

    /// Loads a ShapeMap from a string.
    pub fn read_shapemap(
        &mut self,
        shapemap: &str,
        format: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
    ) -> Result<()> {
        let input = InputSpec::str(shapemap);
        let format: Option<ShapeMapFormat> = parse(format, "ShapeMap format")?;
        let mut b = self.rudof.load_shapemap(&input);
        if let Some(f) = &format {
            b = b.with_shapemap_format(f);
        }
        if let Some(n) = base_nodes {
            b = b.with_base_nodes(n);
        }
        if let Some(s) = base_shapes {
            b = b.with_base_shapes(s);
        }
        Ok(b.execute()?)
    }

    /// Serializes the current ShapeMap.
    pub fn serialize_shapemap(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ShapeMapFormat> = parse(format, "ShapeMap format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_shapemap(w);
            if let Some(f) = &format {
                s = s.with_result_shapemap_format(f);
            }
            s.execute()
        })
    }

    /// Validates the current data against the ShEx schema, for the
    /// associations in the current ShapeMap.
    pub fn validate_shex(&mut self) -> Result<ShExValidationReport> {
        self.rudof.validate_shex().execute()?;
        let results = self
            .rudof
            .shex_validation_results()
            .ok_or_else(|| Error::new("ValidationError", "ShEx validation produced no results"))?;
        Ok(results.into())
    }

    /// Serializes the results of the most recent ShEx validation.
    pub fn serialize_shex_validation_results(&self, format: Option<&str>, sort_mode: Option<&str>) -> Result<String> {
        let format: Option<ResultShExValidationFormat> = parse(format, "ShEx validation result format")?;
        let sort_mode: Option<ShExValidationSortByMode> = parse(sort_mode, "ShEx validation sort mode")?;
        capture(|w| {
            let mut s = self.rudof.serialize_shex_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_shex_validation_format(f);
            }
            if let Some(m) = &sort_mode {
                s = s.with_shex_validation_sort_order_mode(m);
            }
            s.execute()
        })
    }

    // ------------------------------------------------------------------------
    // SHACL
    // ------------------------------------------------------------------------

    /// Loads a SHACL shapes graph from a string. Without `shapes`, the shapes
    /// are taken from the current data.
    pub fn read_shacl(
        &mut self,
        shapes: Option<&str>,
        format: Option<&str>,
        base: Option<&str>,
        reader_mode: Option<&str>,
    ) -> Result<()> {
        let input = shapes.map(InputSpec::str);
        let format: Option<ShaclFormat> = parse(format, "SHACL format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        let mut b = self.rudof.load_shacl_shapes();
        if let Some(i) = &input {
            b = b.with_shacl_schema(i);
        }
        if let Some(f) = &format {
            b = b.with_shacl_schema_format(f);
        }
        if let Some(base) = base {
            b = b.with_base(base);
        }
        if let Some(m) = &reader_mode {
            b = b.with_reader_mode(m);
        }
        Ok(b.execute()?)
    }

    /// Serializes the current SHACL shapes graph.
    pub fn serialize_shacl(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ShaclFormat> = parse(format, "SHACL format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_shacl_shapes(w);
            if let Some(f) = &format {
                s = s.with_shacl_result_format(f);
            }
            s.execute()
        })
    }

    /// Validates the current data against the SHACL shapes graph, with the
    /// `native` (default) or `sparql` engine.
    pub fn validate_shacl(&mut self, mode: Option<&str>) -> Result<ShaclValidationReport> {
        let mode: Option<ShaclValidationMode> = parse(mode, "SHACL validation mode")?;
        let mut b = self.rudof.validate_shacl();
        if let Some(m) = &mode {
            b = b.with_shacl_validation_mode(m);
        }
        b.execute()?;
        let report = self
            .rudof
            .shacl_validation_results()
            .ok_or_else(|| Error::new("ValidationError", "SHACL validation produced no results"))?;
        Ok(report.into())
    }

    /// Serializes the results of the most recent SHACL validation.
    pub fn serialize_shacl_validation_results(&self, format: Option<&str>, sort_mode: Option<&str>) -> Result<String> {
        let format: Option<ResultShaclValidationFormat> = parse(format, "SHACL validation result format")?;
        let sort_mode: Option<ShaclValidationSortByMode> = parse(sort_mode, "SHACL validation sort mode")?;
        capture(|w| {
            let mut s = self.rudof.serialize_shacl_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_shacl_validation_format(f);
            }
            if let Some(m) = &sort_mode {
                s = s.with_shacl_validation_sort_order_mode(m);
            }
            s.execute()
        })
    }

    // ------------------------------------------------------------------------
    // SPARQL
    // ------------------------------------------------------------------------

    /// Loads a SPARQL query. Its type is detected when `query_type` is omitted.
    pub fn read_query(&mut self, query: &str, query_type: Option<&str>) -> Result<()> {
        let input = InputSpec::str(query);
        let query_type: Option<QueryType> = parse(query_type, "query type")?;
        let mut b = self.rudof.load_sparql_query(&input);
        if let Some(t) = &query_type {
            b = b.with_query_type(t);
        }
        Ok(b.execute()?)
    }

    /// Runs the current query against the current data.
    pub fn run_query(&mut self) -> Result<QueryResults> {
        self.rudof.run_query().execute()?;
        let results = self
            .rudof
            .query_results()
            .ok_or_else(|| Error::new("QueryError", "The query produced no results"))?;
        Ok(results.into())
    }

    /// Serializes the results of the most recent query.
    pub fn serialize_query_results(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultQueryFormat> = parse(format, "query result format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_query_results(w);
            if let Some(f) = &format {
                s = s.with_result_query_format(f);
            }
            s.execute()
        })
    }

    // ------------------------------------------------------------------------
    // Prefixes
    // ------------------------------------------------------------------------

    /// The default prefixes, as `(alias, IRI)` pairs.
    pub fn prefixes(&self) -> Vec<(String, String)> {
        self.rudof
            .prefixes()
            .execute()
            .iter()
            .map(|(alias, iri)| (alias.clone(), iri.to_string()))
            .collect()
    }

    pub fn add_prefix(&mut self, alias: &str, iri: &str) -> Result<()> {
        Ok(self.rudof.add_prefix(alias, iri).execute()?)
    }

    pub fn remove_prefix(&mut self, alias: &str) -> Result<()> {
        Ok(self.rudof.remove_prefix(alias).execute()?)
    }

    pub fn rename_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        Ok(self.rudof.rename_prefix(old_alias, new_alias).execute()?)
    }

    pub fn copy_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        Ok(self.rudof.copy_prefix(old_alias, new_alias).execute()?)
    }

    // ------------------------------------------------------------------------
    // Resets
    // ------------------------------------------------------------------------

    /// Clears all the state: data, schemas, queries and results.
    pub fn reset_all(&mut self) {
        self.rudof.reset_all().execute();
    }

    pub fn reset_data(&mut self) {
        self.rudof.reset_data().execute();
    }

    /// Unloads the ShEx schema, ShapeMap and ShEx validation results.
    pub fn reset_shex(&mut self) {
        self.rudof.reset_shex().execute();
    }

    /// Unloads only the ShEx schema.
    pub fn reset_shex_schema(&mut self) {
        self.rudof.reset_shex_schema().execute();
    }

    pub fn reset_shapemap(&mut self) {
        self.rudof.reset_shapemap().execute();
    }

    /// Unloads only the SHACL shapes graph.
    pub fn reset_shacl(&mut self) {
        self.rudof.reset_shacl_shapes().execute();
    }

    /// Unloads the SHACL shapes graph and SHACL validation results.
    pub fn reset_shacl_validation(&mut self) {
        self.rudof.reset_shacl().execute();
    }

    pub fn reset_query(&mut self) {
        self.rudof.reset_sparql_query().execute();
    }

    pub fn reset_query_results(&mut self) {
        self.rudof.reset_query_results().execute();
    }
}

/// Parses an optional string argument with `T`'s `FromStr`.
fn parse<T>(value: Option<&str>, what: &str) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    value
        .map(|v| T::from_str(v).map_err(|e| Error::invalid_argument(format!("Invalid {what} '{v}': {e}"))))
        .transpose()
}

/// Runs a serializer writing to a buffer, and returns what it wrote.
fn capture<E>(serialize: impl FnOnce(&mut Vec<u8>) -> std::result::Result<(), E>) -> Result<String>
where
    Error: From<E>,
{
    let mut buffer = Vec::new();
    serialize(&mut buffer)?;
    String::from_utf8(buffer).map_err(|e| Error::new("RudofError", format!("Output is not valid UTF-8: {e}")))
}
