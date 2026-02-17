use crate::rdf_core::{RDFError, visualizer::RDFVisualizationConfig};
use std::{collections::HashMap, io, path::Path, str::FromStr};

use prefixmap::PrefixMap;

use iri_s::{IriS, error::IriSError};
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Configuration for RDF data readers and visualization settings.
///
/// This struct defines how RDF data should be processed, including base IRI resolution,
/// SPARQL endpoints for querying external data, and visualization preferences.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RdfDataConfig {
    /// Default base IRI to resolve relative IRIs. If `None`, relative IRIs will be treated as errors.
    pub base: Option<IriS>,

    /// SPARQL endpoints for querying RDF data. Each endpoint is identified by a unique name.
    pub endpoints: Option<HashMap<String, EndpointDescription>>,

    /// If true, automatically set the base IRI to the local file or URI of the document being processed.
    pub automatic_base: Option<bool>,

    /// Configuration for RDF visualization appearance and styling.
    pub rdf_visualization: Option<RDFVisualizationConfig>,
}

impl RdfDataConfig {
    /// Creates a new `RdfDataConfig` with default settings.
    ///
    /// The default configuration has no base IRI, no endpoints, automatic base detection enabled,
    /// and no custom visualization settings.
    pub fn new() -> RdfDataConfig {
        RdfDataConfig {
            base: None,
            endpoints: None,
            automatic_base: Some(true),
            rdf_visualization: None,
        }
    }

    /// Adds a Wikidata SPARQL endpoint to the configuration.
    ///
    /// This method configures the Wikidata query service endpoint with appropriate prefixes
    /// for convenient querying of Wikidata's knowledge graph.
    ///
    /// # Returns
    /// The modified `RdfDataConfig` with the Wikidata endpoint added.
    pub fn with_wikidata(mut self) -> Self {
        let wikidata_name = "wikidata";
        let wikidata_iri = "https://query.wikidata.org/sparql";
        let wikidata =
            EndpointDescription::new_unchecked(wikidata_iri).with_prefixmap(PrefixMap::wikidata());

        match self.endpoints {
            None => {
                self.endpoints = Some(HashMap::from([(wikidata_name.to_string(), wikidata)]));
            }
            Some(ref mut map) => {
                map.insert(wikidata_name.to_string(), wikidata);
            }
        };
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RdfDataConfig, RDFError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| RDFError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let s = read_string(f).map_err(|e| RDFError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let config: RdfDataConfig =
            toml::from_str(s.as_str()).map_err(|e| RDFError::TomlError {
                path_name: path_name.to_string(),
                error: e,
            })?;
        Ok(config)
    }

    /// Gets the RDF visualization configuration, using defaults if none is set.
    ///
    /// # Returns
    /// The `RDFVisualizationConfig` to use for visualization, either from this config
    /// or the default configuration if none is specified.
    pub fn rdf_visualization_config(&self) -> RDFVisualizationConfig {
        self.rdf_visualization.clone().unwrap_or_default()
    }
}

impl Default for RdfDataConfig {
    /// Returns the default RDF data configuration with Wikidata endpoint pre-configured.
    ///
    /// The default configuration includes the Wikidata SPARQL endpoint and automatic
    /// base IRI detection enabled.
    fn default() -> Self {
        Self::new().with_wikidata()
    }
}

/// Description of a SPARQL endpoint for querying RDF data.
///
/// This struct contains the necessary information to connect to and query a SPARQL endpoint,
/// including URLs for queries and updates, and optional prefix mappings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDescription {
    /// The URL of the SPARQL query endpoint.
    query_url: IriS,
    /// Optional URL for SPARQL update operations.
    update_url: Option<IriS>,
    /// Optional prefix map for abbreviating IRIs in queries.
    prefixmap: Option<PrefixMap>,
}

impl EndpointDescription {
    /// Creates a new `EndpointDescription` from a URL string without validation.
    ///
    /// # Arguments
    /// * `str` - The URL string for the SPARQL query endpoint.
    pub fn new_unchecked(str: &str) -> Self {
        EndpointDescription {
            query_url: IriS::new_unchecked(str),
            update_url: None,
            prefixmap: None,
        }
    }

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
    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone().unwrap_or_default()
    }

    /// Sets the prefix map for this endpoint.
    ///
    /// # Arguments
    /// * `prefixmap` - The `PrefixMap` to associate with this endpoint.
    ///
    /// # Returns
    /// The modified `EndpointDescription` with the new prefix map.
    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = Some(prefixmap);
        self
    }

    /// Adds or replaces the prefix map for this endpoint.
    ///
    /// # Arguments
    /// * `prefixmap` - The `PrefixMap` to set for this endpoint.
    pub fn add_prefixmap(&mut self, prefixmap: PrefixMap) {
        self.prefixmap = Some(prefixmap);
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
            update_url: None,
            prefixmap: None,
        })
    }
}

/// Reads the entire contents of a reader into a string.
///
/// # Arguments
/// * `reader` - The reader to read from.
///
/// # Returns
/// A `Result` containing the string content or an I/O error.
fn read_string<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}
