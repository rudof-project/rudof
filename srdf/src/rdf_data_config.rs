use std::{collections::HashMap, io, path::Path, str::FromStr};

use prefixmap::PrefixMap;
use thiserror::Error;

use iri_s::{IriS, IriSError};
use serde::{Deserialize, Serialize};
use std::io::Read;

use crate::rdf_visualizer::rdf_visualizer_config::RDFVisualizationConfig;

/// This struct can be used to define configuration of RDF data readers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RdfDataConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    pub base: Option<IriS>,

    /// Endpoints to query RDF data. Each endpoint description is identified by a name
    pub endpoints: Option<HashMap<String, EndpointDescription>>,

    /// If true, the base IRI will be automatically set to the local file or URI of the document
    pub automatic_base: Option<bool>,

    pub rdf_visualization: Option<RDFVisualizationConfig>,
}

impl RdfDataConfig {
    pub fn new() -> RdfDataConfig {
        RdfDataConfig {
            base: None,
            endpoints: None,
            automatic_base: Some(true),
            rdf_visualization: None,
        }
    }

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

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RdfDataConfig, RdfDataConfigError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| RdfDataConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let s = read_string(f).map_err(|e| RdfDataConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let config: RdfDataConfig =
            toml::from_str(s.as_str()).map_err(|e| RdfDataConfigError::TomlError {
                path_name: path_name.to_string(),
                error: e,
            })?;
        Ok(config)
    }

    /*pub fn find_endpoint(&self, str: &str) -> Option<&EndpointDescription> {
        match &self.endpoints {
            None => None,
            Some(map) => match map.get(str) {
                Some(ed) => Some(ed),
                None => None,
            },
        }
    }*/

    pub fn rdf_visualization_config(&self) -> RDFVisualizationConfig {
        self.rdf_visualization.clone().unwrap_or_default()
    }
}

impl Default for RdfDataConfig {
    fn default() -> Self {
        Self::new().with_wikidata()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDescription {
    query_url: IriS,
    update_url: Option<IriS>,
    prefixmap: Option<PrefixMap>,
}

impl EndpointDescription {
    pub fn new_unchecked(str: &str) -> Self {
        EndpointDescription {
            query_url: IriS::new_unchecked(str),
            update_url: None,
            prefixmap: None,
        }
    }

    pub fn query_url(&self) -> &IriS {
        &self.query_url
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.prefixmap.clone().unwrap_or_default()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = Some(prefixmap);
        self
    }

    pub fn add_prefixmap(&mut self, prefixmap: PrefixMap) {
        self.prefixmap = Some(prefixmap);
    }
}

impl FromStr for EndpointDescription {
    type Err = IriSError;

    fn from_str(query_url: &str) -> Result<Self, Self::Err> {
        let iri = IriS::from_str(query_url)?;
        Ok(EndpointDescription {
            query_url: iri,
            update_url: None,
            prefixmap: None,
        })
    }
}

#[derive(Error, Debug)]
pub enum RdfDataConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError {
        path_name: String,
        error: toml::de::Error,
    },

    #[error("Converting to IRI the string {str}. Error: {error}")]
    ConvertingIriEndpoint { error: String, str: String },
}

fn read_string<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}
