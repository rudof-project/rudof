#[cfg(sparql_validation)]
mod endpoint;
mod graph;
mod manager;

#[cfg(sparql_validation)]
pub use endpoint::Endpoint;
pub use graph::Graph;
pub use manager::ShaclDataManager;

pub trait Store<S> {
    fn store(&self) -> &S;
}
