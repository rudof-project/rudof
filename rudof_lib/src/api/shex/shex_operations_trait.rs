use crate::{
    Result,
    api::shex::implementations::{
        add_node_shape_to_shapemap, check_shex_schema, load_shapemap, load_shex_schema, reset_shapemap, reset_shex,
        reset_shex_schema, serialize_shapemap, serialize_shex_schema, serialize_shex_validation_results, validate_shex,
    },
    formats::{
        DataReaderMode, InputSpec, IriNormalizationMode, ResultShExValidationFormat, ShExFormat,
        ShExValidationSortByMode, ShapeMapFormat,
    },
};
use std::io;

/// Operations for ShEx (Shape Expressions) schema validation.
pub trait ShExOperations {
    /// Loads a ShEx schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Optional ShEx format (uses default if None)
    /// * `base_schema` - Optional base IRI for resolving relative IRIs in the schema (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed or loaded.
    fn load_shex_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: Option<&ShExFormat>,
        base_schema: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Checks if a ShEx schema is valid.
    ///
    /// # Arguments
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Optional ShEx format (uses default if None)
    /// * `base_schema` - Optional base IRI for resolving relative IRIs in the schema (uses default if None)
    /// * `writer` - The destination to write validation messages to
    ///
    /// # Errors
    /// Returns an error occurred while checking the schema
    fn check_shex_schema<W: io::Write>(
        &self,
        schema: &InputSpec,
        schema_format: Option<&ShExFormat>,
        base_schema: Option<&str>,
        writer: &mut W,
    ) -> Result<bool>;

    /// Serializes the current ShEx schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `shape_label` - Optional specific shape label to serialize (serializes entire schema if None)
    /// * `show_schema` - Whether to include the schema in the output (true by default)
    /// * `show_statistics` - Whether to include statistics in the output (false by default)
    /// * `show_dependencies` - Whether to show shape dependencies (false by default)
    /// * `show_time` - Whether to include timing information (false by default)
    /// * `show_colors` - Whether to use colors in the output (false by default)
    /// * `shex_format` - Optional format to serialize the schema (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    fn serialize_shex_schema<W: io::Write>(
        &self,
        shape_label: Option<&str>,
        show_schema: Option<bool>,
        show_statistics: Option<bool>,
        show_dependencies: Option<bool>,
        show_time: Option<bool>,
        show_colors: Option<bool>,
        shex_format: Option<&ShExFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the ShEx schema.
    fn reset_shex_schema(&mut self);

    /// Adds a node/shape association to the current shapemap, creating it if none is loaded.
    ///
    /// # Arguments
    ///
    /// * `node` - Node selector string (e.g. `<http://example.org/node>`)
    /// * `shape` - Optional shape label string; defaults to START if None
    /// * `base_nodes` - Optional base IRI for resolving node IRIs
    /// * `base_shapes` - Optional base IRI for resolving shape IRIs
    ///
    /// # Errors
    ///
    /// Returns an error if the node or shape selector cannot be parsed.
    fn add_node_shape_to_shapemap(
        &mut self,
        node: &str,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        iri_mode: IriNormalizationMode,
    ) -> Result<()>;

    /// Loads a shape map from an input specification.
    ///
    /// # Arguments
    ///
    /// * `shapemap` - Input specification defining the shape map source
    /// * `shapemap_format` - Optional shape map format (uses default if None)
    /// * `base_nodes` - Optional base IRI for resolving node IRIs (uses default if None)
    /// * `base_shapes` - Optional base IRI for resolving shape IRIs (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the shape map cannot be parsed or loaded.
    fn load_shapemap(
        &mut self,
        shapemap: &InputSpec,
        shapemap_format: Option<&ShapeMapFormat>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
    ) -> Result<()>;

    /// Serializes the current shape map to a writer.
    ///
    /// # Arguments
    ///
    /// * `shapemap_format` - Optional output format (uses default if None)
    /// * `show_colors` - Whether to use colors in the output (false by default)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no shape map is loaded or serialization fails.
    fn serialize_shapemap<W: io::Write>(
        &self,
        shapemap_format: Option<&ShapeMapFormat>,
        show_colors: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current shape map.
    fn reset_shapemap(&mut self);

    /// Validates the current RDF data using the loaded ShEx schema and shape map.
    ///
    /// # Errors
    ///
    /// Returns an error if no schema or shape map is loaded.
    fn validate_shex(&mut self) -> Result<()>;

    /// Serializes the ShEx validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - Optional sorting mode for the validation results (uses default order if None)
    /// * `result_shex_validation_format` - Optional format to serialize validation results (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    fn serialize_shex_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShExValidationSortByMode>,
        result_shex_validation_format: Option<&ResultShExValidationFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the shex validation.
    fn reset_shex(&mut self);
}

impl ShExOperations for crate::Rudof {
    fn add_node_shape_to_shapemap(
        &mut self,
        node: &str,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        iri_mode: IriNormalizationMode,
    ) -> Result<()> {
        add_node_shape_to_shapemap(self, node, shape, base_nodes, base_shapes, iri_mode)
    }

    fn load_shex_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: Option<&ShExFormat>,
        base_schema: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_shex_schema(self, schema, schema_format, base_schema, reader_mode)
    }

    fn check_shex_schema<W: io::Write>(
        &self,
        schema: &InputSpec,
        schema_format: Option<&ShExFormat>,
        base_schema: Option<&str>,
        writer: &mut W,
    ) -> Result<bool> {
        check_shex_schema(self, schema, schema_format, base_schema, writer)
    }

    fn serialize_shex_schema<W: io::Write>(
        &self,
        shape_label: Option<&str>,
        show_schema: Option<bool>,
        show_statistics: Option<bool>,
        show_dependencies: Option<bool>,
        show_time: Option<bool>,
        show_colors: Option<bool>,
        shex_format: Option<&ShExFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shex_schema(
            self,
            shape_label,
            show_schema,
            show_statistics,
            show_dependencies,
            show_time,
            show_colors,
            shex_format,
            writer,
        )
    }

    fn reset_shex_schema(&mut self) {
        reset_shex_schema(self)
    }

    fn load_shapemap(
        &mut self,
        shapemap: &InputSpec,
        shapemap_format: Option<&ShapeMapFormat>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
    ) -> Result<()> {
        load_shapemap(self, shapemap, shapemap_format, base_nodes, base_shapes)
    }

    fn serialize_shapemap<W: io::Write>(
        &self,
        shapemap_format: Option<&ShapeMapFormat>,
        show_colors: Option<bool>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shapemap(self, shapemap_format, show_colors, writer)
    }

    fn reset_shapemap(&mut self) {
        reset_shapemap(self)
    }

    fn validate_shex(&mut self) -> Result<()> {
        validate_shex(self)
    }

    fn serialize_shex_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShExValidationSortByMode>,
        result_shex_validation_format: Option<&ResultShExValidationFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shex_validation_results(self, sort_order, result_shex_validation_format, writer)
    }

    fn reset_shex(&mut self) {
        reset_shex(self)
    }
}
