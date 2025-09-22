use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const DCT_STR: &str = "http://purl.org/dc/terms/";
pub const DCT_TITLE_STR: &str = concatcp!(DCT_STR, "title");

pub const SD_STR: &str = "http://www.w3.org/ns/sparql-service-description#";
pub const SD_SERVICE_STR: &str = concatcp!(SD_STR, "Service");
pub const SD_DEFAULT_GRAPH_STR: &str = concatcp!(SD_STR, "defaultGraph");
pub const SD_NAMED_GRAPH_STR: &str = concatcp!(SD_STR, "namedGraph");
pub const SD_NAME_STR: &str = concatcp!(SD_STR, "name");
pub const SD_GRAPH_STR: &str = concatcp!(SD_STR, "graph");
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
pub const SD_AVAILABLE_GRAPHS_STR: &str = concatcp!(SD_STR, "availableGraphs");

pub const VOID_STR: &str = "http://rdfs.org/ns/void#";
pub const VOID_TRIPLES_STR: &str = concatcp!(VOID_STR, "triples");
pub const VOID_ENTITIES_STR: &str = concatcp!(VOID_STR, "entities");
pub const VOID_PROPERTIES_STR: &str = concatcp!(VOID_STR, "properties");
pub const VOID_PROPERTY_STR: &str = concatcp!(VOID_STR, "property");
pub const VOID_CLASSES_STR: &str = concatcp!(VOID_STR, "classes");
pub const VOID_CLASS_STR: &str = concatcp!(VOID_STR, "class");
pub const VOID_DOCUMENTS_STR: &str = concatcp!(VOID_STR, "documents");
pub const VOID_CLASS_PARTITION_STR: &str = concatcp!(VOID_STR, "classPartition");
pub const VOID_PROPERTY_PARTITION_STR: &str = concatcp!(VOID_STR, "propertyPartition");
pub const VOID_DISJOINT_SUBJECTS_STR: &str = concatcp!(VOID_STR, "disjointSubjects");
pub const VOID_DISJOINT_OBJECTS_STR: &str = concatcp!(VOID_STR, "disjointObjects");

lazy_static! {
    pub static ref DCT_TITLE: IriS = IriS::new_unchecked(DCT_TITLE_STR);
    pub static ref SD: IriS = IriS::new_unchecked(SD_STR);
    pub static ref SD_SERVICE: IriS = IriS::new_unchecked(SD_SERVICE_STR);
    pub static ref SD_AVAILABLE_GRAPHS: IriS = IriS::new_unchecked(SD_AVAILABLE_GRAPHS_STR);
    pub static ref SD_DEFAULT_GRAPH: IriS = IriS::new_unchecked(SD_DEFAULT_GRAPH_STR);
    pub static ref SD_NAME: IriS = IriS::new_unchecked(SD_NAME_STR);
    pub static ref SD_GRAPH: IriS = IriS::new_unchecked(SD_GRAPH_STR);
    pub static ref SD_NAMED_GRAPH: IriS = IriS::new_unchecked(SD_NAMED_GRAPH_STR);
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
    pub static ref VOID: IriS = IriS::new_unchecked(VOID_STR);
    pub static ref VOID_TRIPLES: IriS = IriS::new_unchecked(VOID_TRIPLES_STR);
    pub static ref VOID_ENTITIES: IriS = IriS::new_unchecked(VOID_ENTITIES_STR);
    pub static ref VOID_PROPERTIES: IriS = IriS::new_unchecked(VOID_PROPERTIES_STR);
    pub static ref VOID_PROPERTY: IriS = IriS::new_unchecked(VOID_PROPERTY_STR);
    pub static ref VOID_CLASSES: IriS = IriS::new_unchecked(VOID_CLASSES_STR);
    pub static ref VOID_CLASS: IriS = IriS::new_unchecked(VOID_CLASS_STR);
    pub static ref VOID_DOCUMENTS: IriS = IriS::new_unchecked(VOID_DOCUMENTS_STR);
    pub static ref VOID_CLASS_PARTITION: IriS = IriS::new_unchecked(VOID_CLASS_PARTITION_STR);
    pub static ref VOID_PROPERTY_PARTITION: IriS = IriS::new_unchecked(VOID_PROPERTY_PARTITION_STR);
    pub static ref VOID_DISJOINT_SUBJECTS: IriS = IriS::new_unchecked(VOID_DISJOINT_SUBJECTS_STR);
    pub static ref VOID_DISJOINT_OBJECTS: IriS = IriS::new_unchecked(VOID_DISJOINT_OBJECTS_STR);
}
