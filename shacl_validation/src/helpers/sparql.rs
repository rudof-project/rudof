use std::collections::HashSet;

use srdf::QuerySRDF;

use super::helper_error::SPARQLError;

pub fn select<S: QuerySRDF>(
    store: &S,
    query_str: String,
    index: &str,
) -> Result<HashSet<S::Term>, SPARQLError> {
    let mut ans = HashSet::new();
    let query = match store.query_select(&query_str) {
        Ok(ans) => ans,
        Err(e) => {
            return Err(SPARQLError::Query {
                query: format!("{query_str}"),
                error: format!("{e}"),
            })
        }
    };
    for solution in query.into_iter() {
        let solution = match solution {
            Ok(ans) => ans,
            Err(e) => {
                return Err(SPARQLError::Query {
                    error: format!("{e}"),
                    query: format!("{query_str}"),
                })
            }
        };
        if let Some(solution) = solution.find_solution(index) {
            ans.insert(solution.to_owned());
        }
    }
    Ok(ans)
}
