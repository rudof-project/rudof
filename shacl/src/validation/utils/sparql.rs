use std::collections::HashSet;
use rudof_rdf::rdf_core::query::{QueryRDF, QuerySolutions};
use rudof_rdf::rdf_core::Rdf;
use crate::validation::utils::error::SparqlError;

pub(crate) fn select<S: QueryRDF>(store: &S, query: &String, index: &str) -> Result<HashSet<S::Term>, SparqlError> {
    let mut out = HashSet::new();

    let q = match store.query_select(&query) {
        Ok(sol) => sol,
        Err(e) => {
            return Err(SparqlError::Query {
                query: query.to_string(),
                err: e.to_string(),
            })
        }
    };

    for sol in query.iter() {
        if let Some(sol) = sol.find_solution(index) {
            out.insert(sol.to_owned());
        }
    }

    Ok(out)
}