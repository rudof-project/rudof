pub trait QuerySRDF: SRDF {
    fn query(&self, query: &str) -> Result<impl Iterator<Item<QuerySolution>>, Self::Err>;

struct QuerySolution {
    solution: HashMap<String, RDFNode>,
}
}

