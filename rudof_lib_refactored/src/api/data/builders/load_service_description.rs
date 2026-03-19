use crate::{Rudof, Result, api::data::DataOperations, formats::{InputSpec, DataFormat, DataReaderMode}};

/// Builder for `load_service_description` operation.
///
/// Provides a fluent interface for configuring and executing service description
/// loading operations with optional parameters.
pub struct LoadServiceDescriptionBuilder<'a> {
    rudof: &'a mut Rudof,
    service: &'a InputSpec,
    data_format: Option<&'a DataFormat>,
    reader_mode: Option<&'a DataReaderMode>,
    base: Option<&'a str>,
}

impl<'a> LoadServiceDescriptionBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_service_description()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, service: &'a InputSpec) -> Self {
        Self {
            rudof,
            service,
            data_format: None,
            reader_mode: None,
            base: None,
        }
    }

    /// Sets the service description format.
    ///
    /// # Arguments
    ///
    /// * `data_format` - The format to use when loading the service description
    pub fn with_data_format(mut self, data_format: &'a DataFormat) -> Self {
        self.data_format = Some(data_format);
        self
    }

    /// Sets the reader mode for service description loading.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during service description loading
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets the base URI for resolving relative IRIs in the service description.
    /// 
    /// # Arguments
    /// 
    /// * `base` - The base URI to use for resolving relative IRIs in the service description
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Executes the service description loading operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the service description cannot be loaded or parsed.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::load_service_description(
            self.rudof,
            self.service,
            self.data_format,
            self.reader_mode,
            self.base,
        )
    }
}
