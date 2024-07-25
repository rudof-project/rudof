use std::collections::HashSet;

use oxigraph::{
    model::Term,
    sparql::{QueryResults, QuerySolution},
    store::Store,
};

use super::helper_error::SPARQLError;

pub fn select(store: &Store, query: String) -> Result<QuerySolution, SPARQLError> {
    match store.query(&query) {
        Ok(QueryResults::Solutions(solutions)) => match solutions.into_iter().nth(0) {
            Some(Ok(solution)) => Ok(solution),
            _ => Err(SPARQLError::NoTripleFound),
        },
        _ => todo!(),
    }
}

pub fn select_many(store: &Store, query: String) -> Result<HashSet<Term>, SPARQLError> {
    let mut ans = HashSet::new();

    match store.query(&query) {
        Ok(QueryResults::Solutions(solutions)) => solutions.into_iter().for_each(|solution| {
            if let Ok(solution) = solution {
                if let Some(this) = solution.get("this") {
                    ans.insert(this.to_owned());
                }
            }
        }),
        _ => todo!(),
    };

    Ok(ans)
}
