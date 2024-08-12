use std::collections::HashSet;

use srdf::QuerySRDF;

use super::helper_error::SPARQLError;

pub fn select<S: QuerySRDF>(
    store: &S,
    query: String,
    index: &str,
) -> Result<HashSet<S::Term>, SPARQLError> {
    let mut ans = HashSet::new();
    let query = match store.query_select(&query) {
        Ok(ans) => ans,
        Err(_) => return Err(SPARQLError::Query),
    };
    for solution in query.into_iter() {
        let solution = match solution {
            Ok(ans) => ans,
            Err(_) => return Err(SPARQLError::Query),
        };
        if let Some(solution) = solution.find_solution(index) {
            ans.insert(solution.to_owned());
        }
    }
    Ok(ans)
}
