#[cfg(feature = "pgschema")]
use pgschema::pg::PropertyGraph;
use sparql_service::RdfData;

#[derive(Debug, Clone)]
pub enum Data {
    RDFData(Box<RdfData>),
    /// Property graph data, only available with the `pgschema` feature.
    #[cfg(feature = "pgschema")]
    PGData(Box<PropertyGraph>),
}

impl Data {
    #[cfg(feature = "pgschema")]
    pub fn empty_pg() -> Self {
        Data::PGData(Box::new(PropertyGraph::new()))
    }

    pub fn is_rdf(&self) -> bool {
        #[cfg(feature = "pgschema")]
        return matches!(self, Data::RDFData(_));
        #[cfg(not(feature = "pgschema"))]
        true
    }

    pub fn is_pg(&self) -> bool {
        #[cfg(feature = "pgschema")]
        return matches!(self, Data::PGData(_));
        #[cfg(not(feature = "pgschema"))]
        false
    }

    pub fn unwrap_rdf_mut(&mut self) -> &mut RdfData {
        match self {
            Data::RDFData(rdf) => rdf,
            #[cfg(feature = "pgschema")]
            _ => panic!("called unwrap_rdf_mut on PGData"),
        }
    }

    #[cfg(feature = "pgschema")]
    pub fn unwrap_pg_mut(&mut self) -> &mut PropertyGraph {
        match self {
            Data::PGData(pg) => pg,
            _ => panic!("called unwrap_pg_mut on RDFData"),
        }
    }
}
