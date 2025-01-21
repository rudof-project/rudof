use std::collections::HashSet;

use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::helpers::helper_error::SPARQLError;

pub fn select<S: Rdf + Sparql>(
    store: &S,
    query_str: String,
    index: &str,
) -> Result<HashSet<Object<S>>, SPARQLError> {
    let mut ans = HashSet::new();
    let query = match store.select(query_str.clone()) {
        Ok(ans) => ans,
        Err(e) => {
            return Err(SPARQLError::Query {
                query: query_str,
                error: format!("{e}"),
            })
        }
    };
    for solution in query.iter() {
        // if let Some(solution) = solution.find_solution(index) {
        //     ans.insert(solution.to_owned());
        // }
        todo!()
    }
    Ok(ans)
}
