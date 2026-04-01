mod data_manager;
mod graph;

pub(crate) trait Store<S> {
    fn store(&self) -> &S;
}