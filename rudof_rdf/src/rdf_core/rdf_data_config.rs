use crate::rdf_core::EndpointDescription;
use crate::rdf_core::RDFError;
use crate::rdf_core::visualizer::RDFVisualizationConfig;
use include_dir::{Dir, include_dir};
use std::{collections::HashMap, path::Path};

use rudof_config::TomlConfig;
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

/// Every `*.toml` file in `rudof_rdf/endpoints/`, embedded at compile time so
/// a built `rudof` binary doesn't depend on that folder existing on disk.
/// Every file found here is registered as a default endpoint (see
/// [`RdfDataConfig::default`]) — to add or adjust a built-in endpoint, add or
/// edit a file there and open a pull request; no Rust code change needed.
static ENDPOINTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/endpoints");

/// Configuration for RDF data readers and visualization settings.
///
/// This struct defines how RDF data should be processed, including base IRI resolution,
/// SPARQL endpoints for querying external data, and visualization preferences.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RdfDataConfig {
    /// Default base IRI to resolve relative IRIs. If `None`, relative IRIs will be treated as errors.
    #[serde(rename = "base_iri", skip_serializing_if = "Option::is_none")]
    pub(crate) base: Option<IriS>,

    /// SPARQL endpoints for querying RDF data. Each endpoint is identified by a unique name.
    #[serde(rename = "endpoints", skip_serializing_if = "HashMap::is_empty")]
    pub(crate) endpoints: HashMap<String, EndpointDescription>,

    /// If true, automatically set the base IRI to the local file or URI of the document being processed.
    #[serde(rename = "local_base")]
    pub(crate) automatic_base: bool,

    /// Configuration for RDF visualization appearance and styling.
    #[serde(rename = "visualization")]
    pub(crate) rdf_visualization: RDFVisualizationConfig,

    /// Optional QLever backend configuration. Reading this section from TOML only records the user's preferences, the QLever container is not started
    /// until the caller explicitly invokes [`QleverGraphContainer::from_path`](crate::rdf_impl::QleverGraphContainer::from_path) or `from_reader`.
    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    #[serde(rename = "qlever", skip_serializing_if = "Option::is_none")]
    pub(crate) qlever: Option<crate::rdf_impl::QleverConfig>,
}

impl RdfDataConfig {
    /// Creates a new `RdfDataConfig` with default settings.
    ///
    /// The default configuration has no base IRI, no endpoints, automatic base detection enabled,
    /// and no custom visualization settings.
    pub fn new() -> Self {
        RdfDataConfig {
            base: Self::default_base(),
            endpoints: Self::default_endpoints(),
            automatic_base: Self::default_automatic_base(),
            rdf_visualization: Self::default_rdf_visualization(),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            qlever: Self::default_qlever(),
        }
    }

    /// Loads an [`EndpointDescription`] from a local TOML file and registers
    /// it under its own `name` field, overwriting any existing endpoint
    /// already registered under that name (matched case-insensitively — see
    /// [`RdfDataConfig::find_endpoint`]). Returns the registered name.
    ///
    /// The file has the same shape as one `[rdf.endpoints.<name>]` table in
    /// `rudof.toml`: `name`, `query_url`, an optional `update_url`, and an
    /// optional `[prefixmap]` table. See `rudof_rdf/endpoints/*.toml` for
    /// examples — those are the very files this loads to build the default
    /// endpoints.
    pub fn load_endpoint_description<P: AsRef<Path>>(&mut self, path: P) -> Result<String, RDFError> {
        let path = path.as_ref();
        let toml_str = std::fs::read_to_string(path).map_err(|error| RDFError::ReadingConfigError {
            path_name: path.display().to_string(),
            error,
        })?;

        self.register_endpoint_description(&path.display().to_string(), &toml_str)
    }

    /// Parses `toml_str` as an [`EndpointDescription`] and registers it under
    /// its own `name` field (returned on success), applying the same
    /// prefix-display styling (`without_default_colors().with_hyperlink(true)`)
    /// the built-in endpoints have always had — `PrefixMap`'s TOML
    /// (de)serialization only round-trips the prefix→IRI pairs themselves,
    /// not that styling, so every endpoint loaded this way (built-in or
    /// user-supplied) gets it applied uniformly rather than only the
    /// built-ins having it.
    ///
    /// `source` is only used to name `toml_str`'s origin in a parse error
    /// message (a file path, or a bundled endpoint's own file name).
    fn register_endpoint_description(&mut self, source: &str, toml_str: &str) -> Result<String, RDFError> {
        let mut endpoint: EndpointDescription = toml::from_str(toml_str).map_err(|error| RDFError::TomlError {
            path_name: source.to_string(),
            error,
        })?;
        endpoint.prefixmap = endpoint.prefixmap.without_default_colors().with_hyperlink(true);
        let name = endpoint.name().to_string();
        self.endpoints.insert(name.clone(), endpoint);
        Ok(name)
    }

    pub fn with_base(mut self, iri: Option<IriS>) -> Self {
        self.base = iri;
        self
    }

    pub fn with_endpoints(mut self, endpoints: HashMap<String, EndpointDescription>) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn with_automatic_base(mut self, flag: bool) -> Self {
        self.automatic_base = flag;
        self
    }

    pub fn with_rdf_visualization(mut self, cfg: RDFVisualizationConfig) -> Self {
        self.rdf_visualization = cfg;
        self
    }

    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    pub fn with_qlever(mut self, cfg: Option<crate::rdf_impl::QleverConfig>) -> Self {
        self.qlever = cfg;
        self
    }
}

impl RdfDataConfig {
    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }

    pub fn endpoints(&self) -> &HashMap<String, EndpointDescription> {
        &self.endpoints
    }

    /// Looks up a registered endpoint by name, case-insensitively — `wikidata`,
    /// `Wikidata` and `WikiData` all match an endpoint registered as `"Wikidata"`.
    pub fn find_endpoint(&self, name: &str) -> Option<&EndpointDescription> {
        self.endpoints
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, endpoint)| endpoint)
    }

    pub fn automatic_base(&self) -> bool {
        self.automatic_base
    }

    /// Gets the RDF visualization configuration, using defaults if none is set.
    ///
    /// # Returns
    /// The `RDFVisualizationConfig` to use for visualization, either from this config
    /// or the default configuration if none is specified.
    pub fn rdf_visualization_config(&self) -> &RDFVisualizationConfig {
        &self.rdf_visualization
    }

    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    pub fn qlever(&self) -> Option<&crate::rdf_impl::QleverConfig> {
        self.qlever.as_ref()
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl RdfDataConfig {
    #[inline]
    fn default_base() -> Option<IriS> { None }
    #[inline]
    fn default_endpoints() -> HashMap<String, EndpointDescription> { HashMap::default() }
    #[inline]
    fn default_automatic_base() -> bool { true }
    #[inline]
    fn default_rdf_visualization() -> RDFVisualizationConfig { RDFVisualizationConfig::default() }

    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    #[inline]
    fn default_qlever() -> Option<crate::rdf_impl::QleverConfig> { None }
}

impl Default for RdfDataConfig {
    fn default() -> Self {
        let mut config = Self::new();
        for file in ENDPOINTS_DIR.files() {
            let path = file.path().display().to_string();
            let toml_str = file
                .contents_utf8()
                .unwrap_or_else(|| panic!("bundled endpoint file '{path}' is not valid UTF-8"));
            // These are compile-time-embedded, maintainer-controlled files —
            // a parse failure here is a build/CI-time bug (a bad edit to
            // `rudof_rdf/endpoints/*.toml`), not something an end user can
            // trigger at runtime, so panicking with a clear message beats
            // threading a `Result` through `Default`.
            config
                .register_endpoint_description(&path, toml_str)
                .unwrap_or_else(|err| panic!("bundled endpoint file '{path}' failed to parse: {err}"));
        }
        config
    }
}

impl TomlConfig for RdfDataConfig {}

#[cfg(test)]
mod tests {
    use super::RdfDataConfig;
    use std::io::Write as _;

    #[test]
    fn default_bundles_the_three_builtin_endpoints() {
        let endpoints = RdfDataConfig::default();
        // Case-insensitive lookup: the registered key is the capitalized
        // `name` field ("Wikidata"), but `find_endpoint` matches regardless.
        let wikidata = endpoints
            .find_endpoint("wikidata")
            .expect("wikidata should be registered");
        assert_eq!(wikidata.name(), "Wikidata");
        assert_eq!(wikidata.query_url().as_str(), "https://query.wikidata.org/sparql");
        assert!(!wikidata.prefixmap().is_empty());
        assert!(endpoints.find_endpoint("DBPEDIA").is_some());
        assert!(endpoints.find_endpoint("UniProt").is_some());
    }

    #[test]
    fn load_endpoint_description_registers_under_its_name_field() {
        let mut file = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
        writeln!(file, r#"name = "Example""#).unwrap();
        writeln!(file, r#"query_url = "https://example.org/sparql""#).unwrap();
        writeln!(file, "[prefixmap]").unwrap();
        writeln!(file, r#"ex = "http://example.org/""#).unwrap();

        let mut config = RdfDataConfig::new();
        let name = config.load_endpoint_description(file.path()).unwrap();

        assert_eq!(name, "Example");
        // Case-insensitive: found regardless of the casing used to look it up.
        let endpoint = config.find_endpoint("example").unwrap();
        assert_eq!(endpoint.name(), "Example");
        assert_eq!(endpoint.query_url().as_str(), "https://example.org/sparql");
        assert_eq!(
            endpoint.prefixmap().map.get("ex").map(|iri| iri.as_str().to_string()),
            Some("http://example.org/".to_string())
        );
    }

    #[test]
    fn load_endpoint_description_reports_a_missing_file() {
        let mut config = RdfDataConfig::new();
        let err = config.load_endpoint_description("/no/such/endpoint.toml").unwrap_err();
        assert!(err.to_string().contains("endpoint.toml"));
    }

    #[test]
    fn load_endpoint_description_reports_malformed_toml() {
        let mut file = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
        writeln!(file, "not valid toml [[[").unwrap();

        let mut config = RdfDataConfig::new();
        let err = config.load_endpoint_description(file.path()).unwrap_err();
        assert!(err.to_string().to_lowercase().contains("toml") || err.to_string().contains("expected"));
    }

    #[test]
    fn partial_toml_fills_remaining_defaults() {
        let c: RdfDataConfig = toml::from_str(
            r#"
            base_iri = "http://ex/"
            local_base = false
        "#,
        )
        .unwrap();
        assert_eq!(c.base().map(|i| i.as_str()), Some("http://ex/"));
        assert!(!c.automatic_base());
        assert_eq!(c.endpoints(), RdfDataConfig::default().endpoints());
    }

    #[test]
    fn toml_round_trip_with_endpoints() {
        let s = r#"
            base_iri = "http://ex/"
            [endpoints.demo]
            name = "Demo"
            query_url = "https://example.org/sparql"
        "#;
        let c: RdfDataConfig = toml::from_str(s).unwrap();
        let out = toml::to_string(&c).unwrap();
        let d: RdfDataConfig = toml::from_str(&out).unwrap();
        assert_eq!(c, d);
        assert!(d.endpoints().contains_key("demo"));
    }

    #[cfg(feature = "qlever")]
    #[test]
    fn qlever_section_parses_from_rdf_qlever() {
        let s = r#"
            base_iri = "http://ex/"
            [qlever]
            host_port = 7042
        "#;
        let c: RdfDataConfig = toml::from_str(s).unwrap();
        assert!(c.qlever.is_some());
    }
}
