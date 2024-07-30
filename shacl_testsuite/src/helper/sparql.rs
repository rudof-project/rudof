use std::collections::HashSet;

use oxigraph::{
    model::Term,
    sparql::{QueryResults, QuerySolution},
    store::Store,
};

use super::helper_error::HelperError;

pub fn select(store: &Store, query: String) -> Result<QuerySolution, HelperError> {
    match store.query(&query) {
        Ok(QueryResults::Solutions(solutions)) => match solutions.into_iter().next() {
            Some(Ok(solution)) => Ok(solution),
            _ => Err(HelperError::NoTripleFound),
        },
        _ => todo!(),
    }
}

pub fn select_many(store: &Store, query: String) -> Result<HashSet<Term>, HelperError> {
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
