use crate::rdf_core::{RDFError, visualizer::RDFVisualizationConfig};
use std::{collections::HashMap, path::Path, str::FromStr};

use prefixmap::PrefixMap;

use rudof_iri::{IriS, error::IriSError};
use serde::{Deserialize, Deserializer, Serialize};
use std::io::Read;

/// Configuration for RDF data readers and visualization settings.
///
/// This struct defines how RDF data should be processed, including base IRI resolution,
/// SPARQL endpoints for querying external data, and visualization preferences.
#[derive(PartialEq, Debug, Clone)]
pub struct RdfDataConfig {
    /// Default base IRI to resolve relative IRIs. If `None`, relative IRIs will be treated as errors.
    pub(crate) base: Option<IriS>,

    /// SPARQL endpoints for querying RDF data. Each endpoint is identified by a unique name.
    pub(crate) endpoints: HashMap<String, EndpointDescription>,

    /// If true, automatically set the base IRI to the local file or URI of the document being processed.
    pub(crate) automatic_base: bool,

    /// Configuration for RDF visualization appearance and styling.
    pub(crate) rdf_visualization: RDFVisualizationConfig,

    /// Optional QLever backend configuration. Reading this section from TOML only records the user's preferences, the QLever container is not started
    /// until the caller explicitly invokes [`QleverGraphContainer::from_path`](crate::rdf_impl::QleverGraphContainer::from_path) or `from_reader`.
    #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
    pub qlever: Option<crate::rdf_impl::QleverConfig>,
}

impl RdfDataConfig {
    /// Creates a new `RdfDataConfig` with default settings.
    ///
    /// The default configuration has no base IRI, no endpoints, automatic base detection enabled,
    /// and no custom visualization settings.
    pub fn new() -> RdfDataConfig {
        RdfDataConfig {
            base: Self::default_base(),
            endpoints: Self::default_endpoints(),
            automatic_base: Self::default_automatic_base(),
            rdf_visualization: Self::default_rdf_visualization(),
            #[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
            qlever: None,
        }
    }

    /// Adds a Wikidata SPARQL endpoint to the configuration.
    pub fn with_wikidata(mut self) -> Self {
        let wikidata_name = "wikidata";
        let wikidata_iri = "https://query.wikidata.org/sparql";
        let wikidata = EndpointDescription::new_unchecked(wikidata_iri).with_prefixmap(PrefixMap::wikidata().into());

        self.endpoints.insert(wikidata_name.to_string(), wikidata);
        self
    }

    /// Adds a DBpedia SPARQL endpoint to the configuration.
    pub fn with_dbpedia(mut self) -> Self {
        let dbpedia_name = "dbpedia";
        let dbpedia_iri = "https://dbpedia.org/sparql";
        let dbpedia = EndpointDescription::new_unchecked(dbpedia_iri).with_prefixmap(PrefixMap::dbpedia().into());

        self.endpoints.insert(dbpedia_name.to_string(), dbpedia);
        self
    }

    /// Adds a Uniprot SPARQL endpoint to the configuration.
    pub fn with_uniprot(mut self) -> Self {
        let uniprot_name = "uniprot";
        let uniprot_iri = "https://sparql.uniprot.org/sparql";
        let uniprot = EndpointDescription::new_unchecked(uniprot_iri).with_prefixmap(PrefixMap::uniprot().into());

        self.endpoints.insert(uniprot_name.to_string(), uniprot);
        self
    }

    /// Loads an `RdfDataConfig` from a TOML file at the specified path.
    ///
    /// # Arguments
    /// * `path` - Path to the TOML configuration file.
    ///
    /// # Returns
    /// A `Result` containing the parsed configuration or an error if reading/parsing fails.
    ///
    /// # Errors
    /// Returns `RDFError` if the file cannot be read or the TOML is invalid.
    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RdfDataConfig, RDFError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| RDFError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| RDFError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;
        let config: RdfDataConfig = toml::from_str(s.as_str()).map_err(|e| RDFError::TomlError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        Ok(config)
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
}

impl RdfDataConfig {
    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }

    pub fn endpoints(&self) -> &HashMap<String, EndpointDescription> {
        &self.endpoints
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
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl RdfDataConfig {
    #[inline]
    fn default_base() -> Option<IriS> { None }
    #[inline]
    fn default_endpoints() -> HashMap<String, EndpointDescription> { HashMap::default() }
    #[inline]
    fn default_automatic_base() -> bool { true }
    #[inline]
    fn default_rdf_visualization() -> RDFVisualizationConfig { RDFVisualizationConfig::default() }
}

impl Default for RdfDataConfig {
    fn default() -> Self {
        Self::new()
            .with_wikidata()
            .with_dbpedia()
            .with_uniprot()
    }
}


/// Description of a SPARQL endpoint for querying RDF data.
///
/// This struct contains the necessary information to connect to and query a SPARQL endpoint,
/// including URLs for queries and updates, and optional prefix mappings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDescription {
    /// The URL of the SPARQL query endpoint.
    #[serde(rename = "query_url")]
    pub(crate) query_url: IriS,
    /// Optional URL for SPARQL update operations.
    #[serde(rename = "update_url", default = "EndpointDescription::default_update_url", skip_serializing_if = "Option::is_none")]
    pub(crate) update_url: Option<IriS>,
    /// Optional prefix map for abbreviating IRIs in queries.
    #[serde(rename = "prefixmap", default = "EndpointDescription::default_prefixmap", skip_serializing_if = "PrefixMap::is_empty")]
    pub(crate) prefixmap: PrefixMap,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl EndpointDescription {
    #[inline]
    fn default_update_url() -> Option<IriS> { None }
    #[inline]
    fn default_prefixmap() -> PrefixMap { PrefixMap::default() }
}

impl EndpointDescription {
    /// Creates a new `EndpointDescription` from a URL string without validation.
    ///
    /// # Arguments
    /// * `str` - The URL string for the SPARQL query endpoint.
    pub fn new_unchecked(str: &str) -> Self {
        EndpointDescription {
            query_url: IriS::new_unchecked(str),
            update_url: Self::default_update_url(),
            prefixmap: Self::default_prefixmap(),
        }
    }

    /// Sets the prefix map for this endpoint.
    ///
    /// # Arguments
    /// * `prefixmap` - The `PrefixMap` to associate with this endpoint.
    ///
    /// # Returns
    /// The modified `EndpointDescription` with the new prefix map.
    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_update_query(mut self, iri: Option<IriS>) -> Self {
        self.update_url = iri;
        self
    }
}

impl EndpointDescription {

    /// Returns the query URL for this endpoint.
    ///
    /// # Returns
    /// A reference to the `IriS` representing the SPARQL query endpoint URL.
    pub fn query_url(&self) -> &IriS {
        &self.query_url
    }

    /// Returns the prefix map for this endpoint, or a default empty map if none is set.
    ///
    /// # Returns
    /// The `PrefixMap` containing IRI prefixes for query abbreviation.
    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn update_url(&self) -> Option<&IriS> {
        self.update_url.as_ref()
    }
}

impl FromStr for EndpointDescription {
    type Err = IriSError;

    /// Parses an `EndpointDescription` from a URL string.
    ///
    /// This validates that the provided string is a valid IRI before creating the endpoint description.
    ///
    /// # Arguments
    /// * `query_url` - The URL string to parse as the SPARQL query endpoint.
    ///
    /// # Returns
    /// A `Result` containing the parsed `EndpointDescription` or an `IriSError` if parsing fails.
    fn from_str(query_url: &str) -> Result<Self, Self::Err> {
        let iri = IriS::from_str(query_url)?;
        Ok(EndpointDescription {
            query_url: iri,
            update_url: Self::default_update_url(),
            prefixmap: Self::default_prefixmap(),
        })
    }
}
