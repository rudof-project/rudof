#[cfg(feature = "sparql")]
mod cached;
#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
mod endpoint;
mod graph;
mod manager;

#[cfg(feature = "sparql")]
pub use cached::{CachedRdfData, DEFAULT_PREFETCH_BATCH_SIZE};
#[cfg(all(feature = "sparql", not(target_family = "wasm")))]
pub use endpoint::Endpoint;
pub use graph::Graph;
pub use manager::ShaclDataManager;

pub trait Store<S> {
    fn store(&self) -> &S;
}
