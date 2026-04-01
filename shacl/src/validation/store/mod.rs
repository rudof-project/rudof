mod data_manager;
mod graph;
mod sparql;

pub(crate) trait Store<S> {
    fn store(&self) -> &S;
}