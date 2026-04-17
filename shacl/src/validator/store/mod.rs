
mod manager;
mod endpoint;
mod graph;

pub use manager::ShaclDataManager;
pub use endpoint::Endpoint;
pub use graph::Graph;

pub trait Store<S> {
    fn store(&self) -> &S;
}