use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const SHT_STR: &str = "http://www.w3.org/ns/shacl-test#";
pub const MF_STR: &str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#";
pub const SHT_DATA_GRAPH_STR: &str = concatcp!(SHT_STR, "dataGraph");
pub const SHT_SHAPES_GRAPH_STR: &str = concatcp!(SHT_STR, "shapesGraph");
pub const SHT_FAILURE_STR: &str = concatcp!(SHT_STR, "Failure");
pub const MF_ACTION_STR: &str = concatcp!(MF_STR, "action");
pub const MF_RESULT_STR: &str = concatcp!(MF_STR, "result");
pub const MF_ENTRIES_STR: &str = concatcp!(MF_STR, "entries");
pub const MF_INCLUDE_STR: &str = concatcp!(MF_STR, "include");

lazy_static! {
    pub static ref SHT_DATA_GRAPH: IriS = IriS::new_unchecked(SHT_DATA_GRAPH_STR);
    pub static ref SHT_SHAPES_GRAPH: IriS = IriS::new_unchecked(SHT_SHAPES_GRAPH_STR);
    pub static ref SHT_FAILURE: IriS = IriS::new_unchecked(SHT_FAILURE_STR);
    pub static ref MF_ACTION: IriS = IriS::new_unchecked(MF_ACTION_STR);
    pub static ref MF_RESULT: IriS = IriS::new_unchecked(MF_RESULT_STR);
    pub static ref MF_ENTRIES: IriS = IriS::new_unchecked(MF_ENTRIES_STR);
    pub static ref MF_INCLUDE: IriS = IriS::new_unchecked(MF_INCLUDE_STR);
}
