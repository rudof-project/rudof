#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
mod endpoint;
mod graph;
mod manager;

#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
pub use endpoint::Endpoint;
pub use graph::Graph;
pub use manager::ShaclDataManager;

pub trait Store<S> {
    fn store(&self) -> &S;
}
