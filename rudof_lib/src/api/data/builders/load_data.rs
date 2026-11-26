use crate::{
    Result, Rudof,
    api::data::DataOperations,
    formats::{BackendSpec, DataFormat, DataReaderMode, InputSpec},
};
use rudof_rdf::rdf_core::EndpointLimits;
use rudof_rdf::rdf_impl::EndpointStrategy;

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
    /// How a `BackendSpec::Endpoint` should answer lookups. Ignored for every
    /// other backend. Defaults to `EndpointStrategy::Sparql`.
    endpoint_strategy: EndpointStrategy,
    /// Request limits for a `BackendSpec::Endpoint`, overriding the configured
    /// ones where set. Ignored for every other backend.
    endpoint_limits: EndpointLimits,
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
            endpoint_strategy: EndpointStrategy::default(),
            endpoint_limits: EndpointLimits::default(),
        }
    }

    /// Selects the RDF backend that will store the loaded data.
    pub fn with_backend(mut self, backend: BackendSpec) -> Self {
        self.backend = backend;
        self
    }

    /// Selects how a `BackendSpec::Endpoint` backend answers lookups —
    /// SPARQL queries (the default) or HTTP dereferencing. No effect unless
    /// the backend is (or becomes, via [`Self::with_endpoint`]) an endpoint.
    pub fn with_endpoint_strategy(mut self, strategy: EndpointStrategy) -> Self {
        self.endpoint_strategy = strategy;
        self
    }

    /// Sets request limits for a `BackendSpec::Endpoint` backend (minimum
    /// interval between requests, concurrent requests, nodes per query). Each
    /// limit set here overrides both the endpoint's own and the `[rdf]` default
    /// from the config; unset ones keep the configured value. No effect unless
    /// the backend is an endpoint.
    pub fn with_endpoint_limits(mut self, limits: EndpointLimits) -> Self {
        self.endpoint_limits = limits;
        self
    }

    /// Shorthand for [`Self::with_endpoint_limits`] setting only the minimum
    /// interval, in milliseconds, between two requests.
    pub fn with_endpoint_min_request_interval_ms(mut self, ms: Option<u64>) -> Self {
        self.endpoint_limits = self.endpoint_limits.with_min_request_interval_ms(ms);
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
                self.endpoint_strategy,
            ),
            BackendSpec::Endpoint(url) => {
                <Rudof as DataOperations>::load_data(
                    self.rudof,
                    self.data,
                    self.data_format,
                    self.base,
                    Some(url.as_str()),
                    self.reader_mode,
                    self.merge,
                    self.prefixes,
                    self.endpoint_strategy,
                )?;
                if !self.endpoint_limits.is_empty() {
                    set_endpoint_limits(self.rudof, url, &self.endpoint_limits);
                }
                Ok(())
            },
            BackendSpec::Qlever => self.execute_qlever(),
            BackendSpec::Lbug => Err(Box::new(crate::errors::DataError::DataSourceSpec {
                message: "the 'lbug' backend cannot supply RDF data yet: rudof has no PG\u{2192}RDF \
                          materialization (only RDF\u{2192}PG, via `rudof load`/`ddl`, exists today). \
                          Use `rudof connect`/`load`/`query --dialect cypher` to work with LadybugDB \
                          directly, or pick --backend memory|qlever|endpoint=... for RDF sources. \
                          See https://github.com/rudof-project/rudof/discussions/747"
                    .to_string(),
            })
            .into()),
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

/// Applies `limits` to the endpoint `load_data` just registered under `name`.
#[cfg(any(not(target_family = "wasm"), feature = "jspi"))]
fn set_endpoint_limits(rudof: &mut Rudof, name: &str, limits: &EndpointLimits) {
    if let Some(crate::types::Data::RDFData(rdf_data)) = rudof.data.as_mut()
        && let Some(endpoint) = rdf_data.use_endpoints().get(name).cloned()
    {
        rdf_data.use_endpoint(name, endpoint.with_limits(limits));
    }
}

/// Endpoints are unavailable here, so `load_data` has already failed before this
/// could be reached.
#[cfg(all(target_family = "wasm", not(feature = "jspi")))]
fn set_endpoint_limits(_rudof: &mut Rudof, _name: &str, _limits: &EndpointLimits) {}
