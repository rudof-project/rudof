use const_format::concatcp;
use iri_s::{IriS, iri_once};

pub const SHT_STR: &str = "http://www.w3.org/ns/shacl-test#";
pub const MF_STR: &str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#";
pub const SHT_DATA_GRAPH_STR: &str = concatcp!(SHT_STR, "dataGraph");
pub const SHT_SHAPES_GRAPH_STR: &str = concatcp!(SHT_STR, "shapesGraph");
pub const SHT_FAILURE_STR: &str = concatcp!(SHT_STR, "Failure");
pub const MF_ACTION_STR: &str = concatcp!(MF_STR, "action");
pub const MF_RESULT_STR: &str = concatcp!(MF_STR, "result");
pub const MF_ENTRIES_STR: &str = concatcp!(MF_STR, "entries");
pub const MF_INCLUDE_STR: &str = concatcp!(MF_STR, "include");

iri_once!(sht_data_graph, SHT_DATA_GRAPH_STR);
iri_once!(sht_shapes_graph, SHT_SHAPES_GRAPH_STR);
iri_once!(sht_failure, SHT_FAILURE_STR);
iri_once!(mf_action, MF_ACTION_STR);
iri_once!(mf_result, MF_RESULT_STR);
iri_once!(mf_entries, MF_ENTRIES_STR);
iri_once!(mf_include, MF_INCLUDE_STR);
