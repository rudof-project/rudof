use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::{InputSpec, ShaclFormat, DataReaderMode}};

/// Builder for `load_shapes` operation.
///
/// Provides a fluent interface for configuring and executing shapes loading
/// operations with optional parameters.
pub struct LoadShapesBuilder<'a> {
    rudof: &'a mut Rudof,
    shapes: &'a InputSpec,
    format: Option<&'a ShaclFormat>,
    base: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a> LoadShapesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_shapes()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, shapes: &'a InputSpec) -> Self {
        Self {
            rudof,
            shapes,
            format: None,
            base: None,
            reader_mode: None,
        }
    }

    /// Sets the SHACL shapes format.
    ///
    /// # Arguments
    ///
    /// * `format` - The format to use when loading the shapes
    pub fn with_format(mut self, format: &'a ShaclFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the base IRI for resolving relative IRIs.
    ///
    /// # Arguments
    ///
    /// * `base` - The base IRI to use for resolution
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Sets the reader mode for shapes loading.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during shapes loading
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Executes the shapes loading operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the shapes cannot be loaded or parsed.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::load_shapes(
            self.rudof,
            self.shapes,
            self.format,
            self.base,
            self.reader_mode,
        )
    }
}
