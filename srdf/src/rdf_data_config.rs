use std::{collections::HashMap, io, path::Path, str::FromStr};

use prefixmap::PrefixMap;
use thiserror::Error;

use iri_s::{error::GenericIriError, IriS};
use serde_derive::{Deserialize, Serialize};

/// This struct can be used to define configuration of RDF data readers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RdfDataConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    pub base: Option<IriS>,
    pub endpoints: Option<HashMap<String, EndpointDescription>>,
}

impl RdfDataConfig {
    pub fn new() -> RdfDataConfig {
        RdfDataConfig {
            base: None,
            endpoints: None,
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

        let config: RdfDataConfig =
            serde_yml::from_reader(f).map_err(|e| RdfDataConfigError::YamlError {
                path_name: path_name.to_string(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn find_endpoint(&self, str: &str) -> Option<&EndpointDescription> {
        match &self.endpoints {
            None => None,
            Some(map) => match map.get(str) {
                Some(ed) => Some(ed),
                None => None,
            },
        }
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
    prefixmap: PrefixMap,
}

impl EndpointDescription {
    pub fn new_unchecked(str: &str) -> Self {
        EndpointDescription {
            query_url: IriS::new_unchecked(str.to_owned()),
            update_url: None,
            prefixmap: PrefixMap::default(),
        }
    }

    pub fn query_url(&self) -> &IriS {
        &self.query_url
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn add_prefixmap(&mut self, prefixmap: PrefixMap) {
        self.prefixmap = prefixmap;
    }
}

impl FromStr for EndpointDescription {
    type Err = GenericIriError;

    fn from_str(query_url: &str) -> Result<Self, Self::Err> {
        let iri = IriS::from_str(query_url)?;
        Ok(EndpointDescription {
            query_url: iri,
            update_url: None,
            prefixmap: PrefixMap::default(),
        })
    }
}

#[derive(Error, Debug)]
pub enum RdfDataConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yml::Error,
    },

    #[error("Converting to IRI the string {str}. Error: {error}")]
    ConvertingIriEndpoint { error: String, str: String },
}
