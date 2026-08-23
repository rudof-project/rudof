use prefixmap::PrefixMap;
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

/// Description of a SPARQL endpoint for querying RDF data.
///
/// This struct contains the necessary information to connect to and query a SPARQL endpoint,
/// including URLs for queries and updates, and optional prefix mappings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDescription {
    /// The display name of this endpoint (e.g. "Wikidata") — also the key it's
    /// registered under, matched case-insensitively (`endpoint`/`--endpoint`
    /// accept `wikidata`, `Wikidata`, or `WikiData` interchangeably).
    #[serde(rename = "name")]
    pub(crate) name: String,

    /// The URL of the SPARQL query endpoint.
    #[serde(rename = "query_url")]
    pub(crate) query_url: IriS,

    /// Optional URL for SPARQL update operations.
    #[serde(
        rename = "update_url",
        default = "EndpointDescription::default_update_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) update_url: Option<IriS>,

    /// Optional prefix map for abbreviating IRIs in queries.
    #[serde(
        rename = "prefixmap",
        default = "EndpointDescription::default_prefixmap",
        skip_serializing_if = "PrefixMap::is_empty"
    )]
    pub(crate) prefixmap: PrefixMap,
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl EndpointDescription {
    #[inline]
    fn default_update_url() -> Option<IriS> { None }
    #[inline]
    fn default_prefixmap() -> PrefixMap { PrefixMap::default() }
}

impl EndpointDescription {
    /// Creates a new `EndpointDescription` from a name and URL string without validation.
    ///
    /// # Arguments
    /// * `name` - The display name this endpoint is registered/matched under.
    /// * `str` - The URL string for the SPARQL query endpoint.
    pub fn new_unchecked(name: &str, str: &str) -> Self {
        EndpointDescription {
            name: name.to_string(),
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
    /// Returns the display name this endpoint is registered/matched under.
    pub fn name(&self) -> &str {
        &self.name
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
    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn update_url(&self) -> Option<&IriS> {
        self.update_url.as_ref()
    }
}
