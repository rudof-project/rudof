use pgschema::pg::PropertyGraph;
use sparql_service::RdfData;

#[derive(Debug, Clone)]
pub enum Data {
    RDFData(RdfData),
    PGData(PropertyGraph),
}

impl Data {
    pub fn empty_rdf() -> Self {
        Data::RDFData(RdfData::new())
    }

    pub fn empty_pg() -> Self {
        Data::PGData(PropertyGraph::new())
    }

    pub fn is_rdf(&self) -> bool {
        matches!(self, Data::RDFData(_))
    }

    pub fn is_pg(&self) -> bool {
        matches!(self, Data::PGData(_))
    }

    pub fn unwrap_rdf_mut(&mut self) -> &mut RdfData {
        match self {
            Data::RDFData(rdf) => rdf,
            _ => panic!("called unwrap_rdf_mut on PGData"),
        }
    }

    pub fn unwrap_pg_mut(&mut self) -> &mut PropertyGraph {
        match self {
            Data::PGData(pg) => pg,
            _ => panic!("called unwrap_pg_mut on RDFData"),
        }
    }
}