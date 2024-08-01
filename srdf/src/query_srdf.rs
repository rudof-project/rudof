use std::collections::HashMap;

use crate::{SRDFBasic, SRDF};

pub trait QuerySRDF: SRDF {
    // Select ?x ?p ?y { where ?x ?p ?y }
    // Iterator
    //    HashMap<x -> pepe, p -> loves, y -> ana>
    //    HashMap<x -> luis, p -> worksFor, y -> Coorp>
    fn query_select(&self, query: &str) -> Result<QuerySolutionIter<Self>, Self::Err>;

    fn query_select(&self, query: &str) -> Result<QuerySolutionIter<Self>, Self::Err>;

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}

pub struct QuerySolution<T> {
    map: HashMap<VarName, T>,
}

impl<T> QuerySolution<T> {
    pub fn find_solution(&self, name: VarName) -> Option<&T> {
        self.map.get(&name)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct VarName {
    str: String,
}

pub struct QuerySolutionIter<S>
where
    S: SRDFBasic,
{
    value: S::Term,
}

impl<T> Iterator for QuerySolutionIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Result<T>> {
        todo!()
    }
}
