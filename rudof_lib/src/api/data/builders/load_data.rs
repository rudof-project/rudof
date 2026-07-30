use crate::{
    Result, Rudof,
    api::data::DataOperations,
    formats::{BackendSpec, DataFormat, DataReaderMode, InputSpec},
};

/// Builder for `load_data` operation.
///
/// Provides a fluent interface for configuring and executing data loading
/// operations with optional parameters.
#[derive(Debug)]
pub struct LoadDataBuilder<'a> {
    rudof: &'a mut Rudof,
    data: Option<&'a [InputSpec]>,
    data_format: Option<&'a DataFormat>,
    base: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
    merge: Option<bool>,
    prefixes: Option<&'a [InputSpec]>,
    /// Which backend should hold the loaded data. Defaults to in-process memory.
    backend: BackendSpec,
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
            reader_mode: None,
            merge: None,
            prefixes: None,
            backend: BackendSpec::default(),
        }
    }

    /// Selects the RDF backend that will store the loaded data.
    pub fn with_backend(mut self, backend: BackendSpec) -> Self {
        self.backend = backend;
        self
    }

    /// Sets the data to be loaded.
    pub fn with_data(mut self, data: &'a [InputSpec]) -> Self {
        self.data = Some(data);
        self
    }

    /// Sets the data format for loading.
    pub fn with_data_format(mut self, data_format: &'a DataFormat) -> Self {
        self.data_format = Some(data_format);
        self
    }

    /// Sets the base URI for resolving relative URIs.
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Sets the reader mode for data loading.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Convenience shim — equivalent to `with_backend(BackendSpec::Endpoint(endpoint.into()))`.
    ///
    /// Provided so callers that only need the endpoint case don't have to
    /// construct a `BackendSpec` themselves.
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.backend = BackendSpec::Endpoint(endpoint.to_string());
        self
    }

    /// Sets whether to merge the loaded data with existing data.
    pub fn with_merge(mut self, merge: bool) -> Self {
        self.merge = Some(merge);
        self
    }

    pub fn with_prefixes(mut self, prefixes: &'a [InputSpec]) -> Self {
        self.prefixes = Some(prefixes);
        self
    }

    /// Executes the data loading operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        match &self.backend {
            BackendSpec::Memory => <Rudof as DataOperations>::load_data(
                self.rudof,
                self.data,
                self.data_format,
                self.base,
                None,
                self.reader_mode,
                self.merge,
                self.prefixes,
            ),
            BackendSpec::Endpoint(url) => <Rudof as DataOperations>::load_data(
                self.rudof,
                self.data,
                self.data_format,
                self.base,
                Some(url.as_str()),
                self.reader_mode,
                self.merge,
                self.prefixes,
            ),
            BackendSpec::Qlever => self.execute_qlever(),
        }
    }

    #[cfg(feature = "qlever")]
    fn execute_qlever(self) -> Result<()> {
        crate::api::data::implementations::load_data_via_qlever(
            self.rudof,
            self.data,
            self.data_format,
            self.base,
            self.prefixes,
        )
    }

    #[cfg(not(feature = "qlever"))]
    fn execute_qlever(self) -> Result<()> {
        Err(Box::new(crate::errors::DataError::DataSourceSpec {
            message: "--backend qlever was passed but this rudof binary was built without the \
                      `qlever` feature. Rebuild with `cargo install rudof_cli --features qlever`."
                .to_string(),
        })
        .into())
    }
}
