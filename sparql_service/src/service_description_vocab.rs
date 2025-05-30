use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const SD_STR: &str = "http://www.w3.org/ns/sparql-service-description#";
pub const SD_SERVICE_STR: &str = concatcp!(SD_STR, "Service");
pub const SD_DEFAULT_GRAPH_STR: &str = concatcp!(SD_STR, "defaultGraph");
pub const SD_DEFAULT_DATASET_STR: &str = concatcp!(SD_STR, "defaultDataset");
pub const SD_ENDPOINT_STR: &str = concatcp!(SD_STR, "endpoint");
pub const SD_FEATURE_STR: &str = concatcp!(SD_STR, "feature");
pub const SD_SUPPORTED_LANGUAGE_STR: &str = concatcp!(SD_STR, "supportedLanguage");
pub const SD_RESULT_FORMAT_STR: &str = concatcp!(SD_STR, "resultFormat");

// Supported languages
pub const SD_SPARQL10_QUERY_STR: &str = concatcp!(SD_STR, "SPARQL10Query");
pub const SD_SPARQL11_QUERY_STR: &str = concatcp!(SD_STR, "SPARQL11Query");
pub const SD_SPARQL11_UPDATE_STR: &str = concatcp!(SD_STR, "SPARQL11Update");

// Feature instances
pub const SD_BASIC_FEDERATED_QUERY_STR: &str = concatcp!(SD_STR, "BasicFederatedQuery");
pub const SD_UNION_DEFAULT_GRAPH_STR: &str = concatcp!(SD_STR, "UnionDefaultGraph");
pub const SD_EMPTY_GRAPHS_STR: &str = concatcp!(SD_STR, "EmptyGraphs");
pub const SD_REQUIRES_DATASET_STR: &str = concatcp!(SD_STR, "RequiresDataset");
pub const SD_DEREFERENCES_URIS_STR: &str = concatcp!(SD_STR, "DereferencesURIs");

lazy_static! {
    pub static ref SD: IriS = IriS::new_unchecked(SD_STR);
    pub static ref SD_SERVICE: IriS = IriS::new_unchecked(SD_SERVICE_STR);
    pub static ref SD_DEFAULT_GRAPH: IriS = IriS::new_unchecked(SD_DEFAULT_GRAPH_STR);
    pub static ref SD_DEFAULT_DATASET: IriS = IriS::new_unchecked(SD_DEFAULT_DATASET_STR);
    pub static ref SD_ENDPOINT: IriS = IriS::new_unchecked(SD_ENDPOINT_STR);
    pub static ref SD_FEATURE: IriS = IriS::new_unchecked(SD_FEATURE_STR);
    pub static ref SD_SUPPORTED_LANGUAGE: IriS = IriS::new_unchecked(SD_SUPPORTED_LANGUAGE_STR);
    pub static ref SD_RESULT_FORMAT: IriS = IriS::new_unchecked(SD_RESULT_FORMAT_STR);
    pub static ref SD_SPARQL10_QUERY: IriS = IriS::new_unchecked(SD_SPARQL10_QUERY_STR);
    pub static ref SD_SPARQL11_QUERY: IriS = IriS::new_unchecked(SD_SPARQL11_QUERY_STR);
    pub static ref SD_SPARQL11_UPDATE: IriS = IriS::new_unchecked(SD_SPARQL11_UPDATE_STR);
    pub static ref SD_BASIC_FEDERATED_QUERY: IriS =
        IriS::new_unchecked(SD_BASIC_FEDERATED_QUERY_STR);
    pub static ref SD_UNION_DEFAULT_GRAPH: IriS = IriS::new_unchecked(SD_UNION_DEFAULT_GRAPH_STR);
    pub static ref SD_REQUIRES_DATASET: IriS = IriS::new_unchecked(SD_REQUIRES_DATASET_STR);
    pub static ref SD_EMPTY_GRAPHS: IriS = IriS::new_unchecked(SD_EMPTY_GRAPHS_STR);
    pub static ref SD_DEREFERENCES_URIS: IriS = IriS::new_unchecked(SD_DEREFERENCES_URIS_STR);
}
