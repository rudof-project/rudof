use srdf_graph::SRDFGraph;
use srdf_sparql::SRDFSparql;
use prefixmap::PrefixMap;

pub enum Data {
    Endpoint(SRDFSparql),
    RDFData(SRDFGraph)
}

impl Data {
    pub fn prefixmap(&self) -> Option<PrefixMap> {
        match self {
            Data::RDFData(data) => Some(data.prefixmap()),
            Data::Endpoint(_) => None
        }
    }
}
