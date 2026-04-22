#[cfg(feature = "sparql")]
mod endpoint;
mod graph;
mod manager;

#[cfg(feature = "sparql")]
pub use endpoint::Endpoint;
pub use graph::Graph;
pub use manager::ShaclDataManager;

pub trait Store<S> {
    fn store(&self) -> &S;
}
