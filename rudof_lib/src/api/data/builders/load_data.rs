use crate::{
    Result, Rudof,
    api::data::DataOperations,
    formats::{DataFormat, DataReaderMode, InputSpec},
};

/// Builder for `load_data` operation.
///
/// Provides a fluent interface for configuring and executing data loading
/// operations with optional parameters.
pub struct LoadDataBuilder<'a> {
    rudof: &'a mut Rudof,
    data: Option<&'a [InputSpec]>,
    data_format: Option<&'a DataFormat>,
    base: Option<&'a str>,
    endpoint: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
    merge: Option<bool>,
}

impl<'a> LoadDataBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_data()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            data: None,
            data_format: None,
            base: None,
            endpoint: None,
            reader_mode: None,
            merge: None,
        }
    }

    /// Sets the data to be loaded.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `InputSpec` defining the data sources to load
    pub fn with_data(mut self, data: &'a [InputSpec]) -> Self {
        self.data = Some(data);
        self
    }

    /// Sets the data format for loading.
    ///
    /// # Arguments
    ///
    /// * `data_format` - The format to use when loading the data
    pub fn with_data_format(mut self, data_format: &'a DataFormat) -> Self {
        self.data_format = Some(data_format);
        self
    }

    /// Sets the base URI for resolving relative URIs.
    ///
    /// # Arguments
    ///
    /// * `base` - The base URI to use for resolution
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Sets the reader mode for data loading.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during data loading
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets the SPARQL endpoint for loading data.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The SPARQL endpoint URL to load data from
    pub fn with_endpoint(mut self, endpoint: &'a str) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Sets whether to merge the loaded data with existing data.
    ///
    /// # Arguments
    ///
    /// * `merge` - If true, the loaded data will be merged with existing data; if false, it will replace existing data
    pub fn with_merge(mut self, merge: bool) -> Self {
        self.merge = Some(merge);
        self
    }

    /// Executes the data loading operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::load_data(
            self.rudof,
            self.data,
            self.data_format,
            self.base,
            self.endpoint,
            self.reader_mode,
            self.merge,
        )
    }
}
