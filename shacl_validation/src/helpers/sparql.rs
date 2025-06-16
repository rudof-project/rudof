use std::collections::HashSet;
use srdf::QueryRDF;
use super::helper_error::SPARQLError;

pub fn select<S: QueryRDF>(
    store: &S,
    query_str: String,
    index: &str,
) -> Result<HashSet<S::Term>, SPARQLError> {
    let mut ans = HashSet::new();
    let query = match store.query_select(&query_str) {
        Ok(ans) => ans,
        Err(e) => {
            return Err(SPARQLError::Query {
                query: query_str.to_string(),
                error: format!("{e}"),
            })
        }
    };
    for solution in query.iter() {
        if let Some(solution) = solution.find_solution(index) {
            ans.insert(solution.to_owned());
        }
    }
    Ok(ans)
}
