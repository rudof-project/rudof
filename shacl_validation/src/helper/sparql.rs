use std::collections::HashSet;

use oxigraph::{model::Term, sparql::QueryResults, store::Store};

use super::helper_error::SPARQLError;

pub fn select(store: &Store, query: String) -> Result<HashSet<Term>, SPARQLError> {
    let mut ans = HashSet::new();
    match store.query(&query) {
        Ok(query_results) => match query_results {
            QueryResults::Solutions(solutions) => solutions.into_iter().for_each(|solution| {
                if let Ok(solution) = solution {
                    if let Some(this) = solution.get("this") {
                        ans.insert(this.to_owned());
                    }
                }
            }),
            _ => todo!(),
        },
        Err(_) => todo!(),
    };
    Ok(ans)
}

pub fn ask(store: &Store, query: String) -> Result<bool, SPARQLError> {
    match store.query(&query) {
        Ok(query_results) => match query_results {
            QueryResults::Boolean(bool) => Ok(bool),
            _ => todo!(),
        },
        Err(_) => todo!(),
    }
}
