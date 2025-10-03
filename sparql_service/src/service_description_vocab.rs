use const_format::concatcp;
use iri_s::IriS;
use iri_s::iri_once;

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

iri_once!(dct_title, DCT_TITLE_STR);
iri_once!(sd, SD_STR);
iri_once!(sd_service, SD_SERVICE_STR);
iri_once!(sd_available_graphs, SD_AVAILABLE_GRAPHS_STR);
iri_once!(sd_default_graph, SD_DEFAULT_GRAPH_STR);
iri_once!(sd_name, SD_NAME_STR);
iri_once!(sd_graph, SD_GRAPH_STR);
iri_once!(sd_named_graph, SD_NAMED_GRAPH_STR);
iri_once!(sd_default_dataset, SD_DEFAULT_DATASET_STR);
iri_once!(sd_endpoint, SD_ENDPOINT_STR);
iri_once!(sd_feature, SD_FEATURE_STR);
iri_once!(sd_supported_language, SD_SUPPORTED_LANGUAGE_STR);
iri_once!(sd_result_format, SD_RESULT_FORMAT_STR);
iri_once!(sd_sparql10_query, SD_SPARQL10_QUERY_STR);
iri_once!(sd_sparql11_query, SD_SPARQL11_QUERY_STR);
iri_once!(sd_sparql11_update, SD_SPARQL11_UPDATE_STR);
iri_once!(sd_basic_federated_query, SD_BASIC_FEDERATED_QUERY_STR);
iri_once!(sd_union_default_graph, SD_UNION_DEFAULT_GRAPH_STR);
iri_once!(sd_requires_dataset, SD_REQUIRES_DATASET_STR);
iri_once!(sd_empty_graphs, SD_EMPTY_GRAPHS_STR);
iri_once!(sd_dereferences_uris, SD_DEREFERENCES_URIS_STR);
iri_once!(void, VOID_STR);
iri_once!(void_triples, VOID_TRIPLES_STR);
iri_once!(void_entities, VOID_ENTITIES_STR);
iri_once!(void_properties, VOID_PROPERTIES_STR);
iri_once!(void_property, VOID_PROPERTY_STR);
iri_once!(void_classes, VOID_CLASSES_STR);
iri_once!(void_class, VOID_CLASS_STR);
iri_once!(void_documents, VOID_DOCUMENTS_STR);
iri_once!(void_class_partition, VOID_CLASS_PARTITION_STR);
iri_once!(void_property_partition, VOID_PROPERTY_PARTITION_STR);
iri_once!(void_disjoint_subjects, VOID_DISJOINT_SUBJECTS_STR);
iri_once!(void_disjoint_objects, VOID_DISJOINT_OBJECTS_STR);
