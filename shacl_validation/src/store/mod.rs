pub mod graph;
pub mod sparql;

pub trait Store<S> {
    fn store(&self) -> &S;
}
