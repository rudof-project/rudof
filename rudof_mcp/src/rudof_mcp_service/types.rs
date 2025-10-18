use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExamplePromptArgs {
    /// A message to put in the prompt
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataRequest {
    /// RDF data to load
    pub rdf_data: String,
    /// RDF format (e.g. "turtle", "jsonld")
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataRequest {
    /// RDF format (e.g. "turtle", "jsonld")
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoadRdfDataResponse {
    /// Serialized RDF data as a string
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExportRdfDataResponse {
    /// Serialized RDF data as a string
    pub data: String,
    /// Format used for serialization (e.g. "turtle", "jsonld")
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoRequest {
    /// Node IRI or prefixed name (e.g. ":a" or "http://example.org/a")
    pub node: String,
    /// Optional list of predicates to filter outgoing arcs
    pub predicates: Option<Vec<String>>,
    /// Optional mode: "incoming", "outgoing", or "both" (default "both")
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeInfoResponse {
    /// The qualified IRI of the RDF subject node.
    pub subject: String,
    /// List of outgoing arcs from the subject node.
    pub outgoing: Vec<NodePredicateObjects>,
    /// List of incoming arcs to the subject node.
    pub incoming: Vec<NodePredicateSubjects>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateObjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified object terms linked via this predicate.
    pub objects: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodePredicateSubjects {
    /// The qualified IRI of the predicate.
    pub predicate: String,
    /// List of qualified subject terms that point to the node via this predicate.
    pub subjects: Vec<String>,
}
