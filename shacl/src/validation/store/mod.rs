mod data_manager;
mod graph;
mod sparql;

pub(crate) use graph::Graph;
pub(crate) use sparql::Endpoint;

pub(crate) trait Store<S> {
    fn store(&self) -> &S;
}