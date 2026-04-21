
mod manager;
#[cfg(feature = "sparql")]
mod endpoint;
mod graph;

pub use manager::ShaclDataManager;
#[cfg(feature = "sparql")]
pub use endpoint::Endpoint;
pub use graph::Graph;

pub trait Store<S> {
    fn store(&self) -> &S;
}