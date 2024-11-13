use prefixmap::PrefixMap;
use srdf::{QuerySolutions, Rdf};

use crate::RdfData;

#[derive(Clone)]
pub struct QueryProcessor {
    rdf_data: RdfData,
}

impl QueryProcessor {
    pub fn new(rdf_data: RdfData) -> QueryProcessor {
        QueryProcessor {
            rdf_data: rdf_data.clone(),
        }
    }

    pub fn prefix_map(&self) -> Option<PrefixMap> {
        Some(self.rdf_data.prefixmap_in_memory())
    }

    pub fn query_select<S: Rdf>(_str: &str) -> QuerySolutions<S> {
        todo!()
    }
}
