//! A stateful rudof session, wrapping [`rudof_lib::Rudof`] with the API of the
//! Python bindings: loaded data, schemas and results persist across calls until
//! a `reset_*` method clears them.
//!
//! Formats and modes are passed as strings (e.g. `"turtle"`), parsed with the
//! `FromStr` implementations of `rudof_lib::formats`. Inputs are strings: files,
//! URLs and SPARQL endpoints are not available on `wasm`.

use crate::error::{Error, Result};
#[cfg(feature = "pgschema")]
use crate::reports::PgSchemaValidationReport;
use crate::reports::{
    ExternalResolver, NodeNeighborhood, QueryResults, ShExCheck, ShExValidationReport, ShaclValidationReport,
};
#[cfg(feature = "comparison")]
use rudof_lib::formats::{ComparisonFormat, ComparisonMode};
#[cfg(feature = "conversion")]
use rudof_lib::formats::{ConversionFormat, ConversionMode, ResultConversionFormat, ResultConversionMode};
#[cfg(feature = "dctap")]
use rudof_lib::formats::{DCTapFormat, ResultDCTapFormat};
use rudof_lib::formats::{
    DataFormat, DataReaderMode, InputSpec, IriNormalizationMode, NodeInspectionMode, QueryType, ResultDataFormat,
    ResultQueryFormat, ResultServiceFormat, ResultShExValidationFormat, ResultShaclValidationFormat, ShExFormat,
    ShExValidationSortByMode, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode, ShapeMapFormat,
};
#[cfg(feature = "pgschema")]
use rudof_lib::formats::{PgSchemaFormat, ResultPgSchemaValidationFormat};
#[cfg(feature = "rdf-config")]
use rudof_lib::formats::{RdfConfigFormat, ResultRdfConfigFormat};
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

    #[cfg(feature = "dctap")]
    pub fn reset_dctap(&mut self) {
        self.rudof.reset_dctap().execute();
    }

    #[cfg(feature = "rdf-config")]
    pub fn reset_rdf_config(&mut self) {
        self.rudof.reset_rdf_config().execute();
    }

    pub fn reset_service_description(&mut self) {
        self.rudof.reset_service_description().execute();
    }

    #[cfg(feature = "pgschema")]
    pub fn reset_pgschema(&mut self) {
        self.rudof.reset_pg_schema().execute();
    }

    #[cfg(feature = "pgschema")]
    pub fn reset_typemap(&mut self) {
        self.rudof.reset_typemap().execute();
    }

    #[cfg(feature = "pgschema")]
    pub fn reset_pgschema_validation(&mut self) {
        self.rudof.reset_pg_schema_validation().execute();
    }

    /// Clears the ShEx, SHACL and property graph schema validation state: the
    /// results, and the schemas and ShapeMap they were computed with.
    pub fn reset_validation_results(&mut self) {
        self.rudof.reset_shex().execute();
        self.rudof.reset_shacl().execute();
        #[cfg(feature = "pgschema")]
        self.rudof.reset_pg_schema_validation().execute();
    }

    // ------------------------------------------------------------------------
    // More RDF data operations
    // ------------------------------------------------------------------------

    /// Dereferences an IRI and adds the retrieved triples to the current data.
    /// Not available on `wasm`, where it fails with a `DataError`.
    pub fn dereference(&mut self, uri: &str, reader_mode: Option<&str>, merge: Option<bool>) -> Result<()> {
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        let mut b = self.rudof.dereference(uri);
        if let Some(m) = &reader_mode {
            b = b.with_reader_mode(m);
        }
        if let Some(merge) = merge {
            b = b.with_merge(merge);
        }
        Ok(b.execute()?)
    }

    /// The known SPARQL endpoints, as `(name, url)` pairs. Always empty on
    /// `wasm`, where SPARQL endpoints are not available.
    pub fn list_endpoints(&mut self) -> Result<Vec<(String, String)>> {
        Ok(self.rudof.list_endpoints().execute()?)
    }

    /// Describes the nodes selected by `node_selector` (e.g. `:alice`), with
    /// their outgoing and/or incoming arcs.
    pub fn node_info(
        &mut self,
        node_selector: &str,
        predicates: Option<&[String]>,
        mode: Option<&str>,
        show_colors: Option<bool>,
        depth: Option<usize>,
    ) -> Result<String> {
        let mode: Option<NodeInspectionMode> = parse(mode, "node inspection mode")?;
        capture(|w| {
            let mut b = self.rudof.show_node_info(node_selector, w);
            if let Some(p) = predicates {
                b = b.with_predicates(p);
            }
            if let Some(m) = &mode {
                b = b.with_show_node_mode(m);
            }
            if let Some(c) = show_colors {
                b = b.with_show_colors(c);
            }
            if let Some(d) = depth {
                b = b.with_depth(d);
            }
            b.execute()
        })
    }

    /// The arcs around the nodes selected by `node_selector`, up to `depth`
    /// hops away. With `limit`, at most that many arcs are returned.
    pub fn node_neighborhood(
        &self,
        node_selector: &str,
        predicates: Option<&[String]>,
        mode: Option<&str>,
        depth: Option<usize>,
        strict_iris: Option<bool>,
        limit: Option<usize>,
    ) -> Result<NodeNeighborhood> {
        let mode: Option<NodeInspectionMode> = parse(mode, "node inspection mode")?;
        let mut b = self.rudof.node_neighborhood(node_selector);
        if let Some(p) = predicates {
            b = b.with_predicates(p);
        }
        if let Some(m) = &mode {
            b = b.with_mode(m);
        }
        if let Some(d) = depth {
            b = b.with_depth(d);
        }
        if strict_iris.unwrap_or(false) {
            b = b.with_iri_mode(IriNormalizationMode::Strict);
        }
        let arcs = b.execute()?;
        // One arc past `limit`, to tell whether the neighborhood was truncated.
        let mut arcs = match limit {
            Some(limit) => arcs
                .take(limit.saturating_add(1))
                .collect::<std::result::Result<Vec<_>, _>>(),
            None => arcs.collect(),
        }?;
        let truncated = limit.is_some_and(|limit| arcs.len() > limit);
        if let Some(limit) = limit {
            arcs.truncate(limit);
        }
        Ok(NodeNeighborhood {
            arcs: arcs.iter().map(Into::into).collect(),
            truncated,
        })
    }

    /// Materializes the RDF graph described by the ShEx map extension
    /// (`Map` semantic actions) of the last ShEx validation.
    pub fn materialize(&self, format: Option<&str>, node: Option<&str>) -> Result<String> {
        let format: Option<ResultDataFormat> = parse(format, "result data format")?;
        capture(|w| {
            let mut m = self.rudof.materialize(w);
            if let Some(f) = &format {
                m = m.with_result_format(f);
            }
            if let Some(n) = node {
                m = m.with_initial_node_iri(n);
            }
            m.execute()
        })
    }

    // ------------------------------------------------------------------------
    // More ShEx operations
    // ------------------------------------------------------------------------

    /// Checks whether a ShEx schema is well formed, without loading it.
    pub fn check_shex(&self, schema: &str, format: Option<&str>, base: Option<&str>) -> Result<ShExCheck> {
        let input = InputSpec::str(schema);
        let format: Option<ShExFormat> = parse(format, "ShEx format")?;
        let mut output = Vec::new();
        let valid = {
            let mut b = self.rudof.check_shex_schema(&input, &mut output);
            if let Some(f) = &format {
                b = b.with_shex_schema_format(f);
            }
            if let Some(base) = base {
                b = b.with_base(base);
            }
            b.execute()?
        };
        let message = String::from_utf8(output)
            .map_err(|e| Error::new("RudofError", format!("Output is not valid UTF-8: {e}")))?;
        Ok(ShExCheck { valid, message })
    }

    /// Adds an external shape resolver, configured by `spec` (see
    /// [`Session::list_external_resolvers`] for the syntax of each kind).
    pub fn add_external_resolver(&mut self, spec: &str) -> Result<()> {
        Ok(self.rudof.add_external_resolver(spec)?)
    }

    pub fn clear_external_resolvers(&mut self) {
        self.rudof.clear_external_resolvers();
    }

    /// The kinds of external shape resolvers available.
    pub fn list_external_resolvers() -> Vec<ExternalResolver> {
        Rudof::list_external_resolvers()
            .into_iter()
            .map(|info| ExternalResolver {
                name: info.name.to_string(),
                description: info.description.to_string(),
                spec_syntax: info.spec_syntax.to_string(),
            })
            .collect()
    }

    // ------------------------------------------------------------------------
    // Schema conversion and comparison
    // ------------------------------------------------------------------------

    /// Compares two schemas (`mode1`/`mode2`: `shex`, `shacl`, ...), or only
    /// the shapes `label1` and `label2`.
    #[cfg(feature = "comparison")]
    #[allow(clippy::too_many_arguments)]
    pub fn compare_schemas(
        &mut self,
        schema1: &str,
        schema2: &str,
        mode1: &str,
        mode2: &str,
        format1: &str,
        format2: &str,
        base1: Option<&str>,
        base2: Option<&str>,
        label1: Option<&str>,
        label2: Option<&str>,
        reader_mode: Option<&str>,
    ) -> Result<String> {
        let (input1, input2) = (InputSpec::str(schema1), InputSpec::str(schema2));
        let mode1 = required::<ComparisonMode>(mode1, "comparison mode")?;
        let mode2 = required::<ComparisonMode>(mode2, "comparison mode")?;
        let format1 = required::<ComparisonFormat>(format1, "comparison format")?;
        let format2 = required::<ComparisonFormat>(format2, "comparison format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        capture(|w| {
            let mut c = self
                .rudof
                .show_schema_comparison(&input1, &input2, &format1, &format2, &mode1, &mode2, w);
            if let Some(m) = &reader_mode {
                c = c.with_reader_mode(m);
            }
            if let Some(b) = base1 {
                c = c.with_base1(b);
            }
            if let Some(b) = base2 {
                c = c.with_base2(b);
            }
            if let Some(l) = label1 {
                c = c.with_shape1(l);
            }
            if let Some(l) = label2 {
                c = c.with_shape2(l);
            }
            c.execute()
        })
    }

    /// Converts a schema between modes (e.g. `shex` to `uml`, `shacl` to
    /// `shex`, `dctap` to `shex`). Conversions that write to a folder (HTML)
    /// or render images are not available on `wasm`.
    #[cfg(feature = "conversion")]
    #[allow(clippy::too_many_arguments)]
    pub fn convert_schemas(
        &mut self,
        schema: &str,
        input_mode: &str,
        output_mode: &str,
        input_format: &str,
        output_format: &str,
        base: Option<&str>,
        reader_mode: Option<&str>,
        shape: Option<&str>,
    ) -> Result<String> {
        let input = InputSpec::str(schema);
        let input_mode = required::<ConversionMode>(input_mode, "conversion mode")?;
        let output_mode = required::<ResultConversionMode>(output_mode, "result conversion mode")?;
        let input_format = required::<ConversionFormat>(input_format, "conversion format")?;
        let output_format = required::<ResultConversionFormat>(output_format, "result conversion format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        capture(|w| {
            let mut c =
                self.rudof
                    .show_schema_conversion(&input, &input_mode, &output_mode, &input_format, &output_format, w);
            if let Some(b) = base {
                c = c.with_base(b);
            }
            if let Some(m) = &reader_mode {
                c = c.with_reader_mode(m);
            }
            if let Some(s) = shape {
                c = c.with_shape(s);
            }
            c.execute()
        })
    }

    // ------------------------------------------------------------------------
    // DCTAP, rdf-config and service descriptions
    // ------------------------------------------------------------------------

    /// Loads a DCTAP profile (`csv` by default).
    #[cfg(feature = "dctap")]
    pub fn read_dctap(&mut self, dctap: &str, format: Option<&str>) -> Result<()> {
        let input = InputSpec::str(dctap);
        let format: Option<DCTapFormat> = parse(format, "DCTAP format")?;
        let mut b = self.rudof.load_dctap(&input);
        if let Some(f) = &format {
            b = b.with_dctap_format(f);
        }
        Ok(b.execute()?)
    }

    #[cfg(feature = "dctap")]
    pub fn serialize_dctap(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultDCTapFormat> = parse(format, "result DCTAP format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_dctap(w);
            if let Some(f) = &format {
                s = s.with_result_dctap_format(f);
            }
            s.execute()
        })
    }

    /// Loads an rdf-config document (YAML).
    #[cfg(feature = "rdf-config")]
    pub fn read_rdf_config(&mut self, rdf_config: &str, format: Option<&str>) -> Result<()> {
        let input = InputSpec::str(rdf_config);
        let format: Option<RdfConfigFormat> = parse(format, "rdf-config format")?;
        let mut b = self.rudof.load_rdf_config(&input);
        if let Some(f) = &format {
            b = b.with_rdf_config_format(f);
        }
        Ok(b.execute()?)
    }

    #[cfg(feature = "rdf-config")]
    pub fn serialize_rdf_config(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultRdfConfigFormat> = parse(format, "result rdf-config format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_rdf_config(w);
            if let Some(f) = &format {
                s = s.with_result_rdf_config_format(f);
            }
            s.execute()
        })
    }

    /// Loads a SPARQL service description, in RDF.
    pub fn read_service_description(
        &mut self,
        service_description: &str,
        format: Option<&str>,
        base: Option<&str>,
        reader_mode: Option<&str>,
    ) -> Result<()> {
        let input = InputSpec::str(service_description);
        let format: Option<DataFormat> = parse(format, "data format")?;
        let reader_mode: Option<DataReaderMode> = parse(reader_mode, "reader mode")?;
        let mut b = self.rudof.load_service_description(&input);
        if let Some(f) = &format {
            b = b.with_data_format(f);
        }
        if let Some(base) = base {
            b = b.with_base(base);
        }
        if let Some(m) = &reader_mode {
            b = b.with_reader_mode(m);
        }
        Ok(b.execute()?)
    }

    pub fn serialize_service_description(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultServiceFormat> = parse(format, "service description format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_service_description(w);
            if let Some(f) = &format {
                s = s.with_result_service_format(f);
            }
            s.execute()
        })
    }

    // ------------------------------------------------------------------------
    // Property graph schemas
    // ------------------------------------------------------------------------

    /// Loads a property graph schema (PGSchemaC).
    #[cfg(feature = "pgschema")]
    pub fn read_pgschema(&mut self, pgschema: &str, format: Option<&str>) -> Result<()> {
        let input = InputSpec::str(pgschema);
        let format: Option<PgSchemaFormat> = parse(format, "property graph schema format")?;
        let mut b = self.rudof.load_pg_schema(&input);
        if let Some(f) = &format {
            b = b.with_pg_schema_format(f);
        }
        Ok(b.execute()?)
    }

    #[cfg(feature = "pgschema")]
    pub fn serialize_pgschema(&self, format: Option<&str>) -> Result<String> {
        let format: Option<PgSchemaFormat> = parse(format, "property graph schema format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_pg_schema(w);
            if let Some(f) = &format {
                s = s.with_result_pg_schema_format(f);
            }
            s.execute()
        })
    }

    /// Loads a type map, associating property graph nodes with schema types.
    #[cfg(feature = "pgschema")]
    pub fn read_typemap(&mut self, typemap: &str) -> Result<()> {
        let input = InputSpec::str(typemap);
        Ok(self.rudof.load_typemap(&input).execute()?)
    }

    /// Validates the current property graph data (loaded with `read_data` in
    /// `pg` format) against the property graph schema, for the type map.
    #[cfg(feature = "pgschema")]
    pub fn validate_pgschema(&mut self) -> Result<PgSchemaValidationReport> {
        self.rudof.validate_pgschema().execute()?;
        let result = self.rudof.pgschema_validation_results().ok_or_else(|| {
            Error::new(
                "ValidationError",
                "Property graph schema validation produced no results",
            )
        })?;
        Ok(result.into())
    }

    #[cfg(feature = "pgschema")]
    pub fn serialize_pgschema_validation_results(&self, format: Option<&str>) -> Result<String> {
        let format: Option<ResultPgSchemaValidationFormat> =
            parse(format, "property graph schema validation result format")?;
        capture(|w| {
            let mut s = self.rudof.serialize_pgschema_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_pg_schema_validation_format(f);
            }
            s.execute()
        })
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

/// Parses a required string argument with `T`'s `FromStr`.
#[cfg(any(feature = "comparison", feature = "conversion"))]
fn required<T>(value: &str, what: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    T::from_str(value).map_err(|e| Error::invalid_argument(format!("Invalid {what} '{value}': {e}")))
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
