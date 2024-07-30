use std::collections::HashSet;

use oxigraph::sparql::QueryResults;

use crate::runner::oxigraph::OxigraphStore;

use super::{helper_error::SPARQLError, term::Term};

pub(crate) fn select(
    store: &OxigraphStore<'_>,
    query: String,
) -> Result<HashSet<Term>, SPARQLError> {
    let mut ans = HashSet::new();
    match store.as_ref().query(&query) {
        Ok(query_results) => match query_results {
            QueryResults::Solutions(solutions) => solutions.into_iter().for_each(|solution| {
                if let Ok(solution) = solution {
                    if let Some(this) = solution.get("this") {
                        ans.insert(this.to_owned().into());
                    }
                }
            }),
            _ => todo!(),
        },
        Err(error) => {
            eprintln!("{}", error);
            todo!()
        }
    };
    Ok(ans)
}

pub(crate) fn ask(store: &OxigraphStore<'_>, query: String) -> Result<bool, SPARQLError> {
    match store.as_ref().query(&query) {
        Ok(query_results) => match query_results {
            QueryResults::Boolean(bool) => Ok(bool),
            _ => todo!(),
        },
        Err(_) => todo!(),
    }
}
