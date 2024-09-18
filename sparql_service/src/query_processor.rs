use prefixmap::PrefixMap;
use srdf::{QuerySolutionIter, RdfData, SRDF};

#[derive(Debug, Clone)]
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
        Some(self.rdf_data.prefixmap())
    }

    pub fn query_select<S: SRDF>(_str: &str) -> QuerySolutionIter<S> {
        todo!()
    }
}
