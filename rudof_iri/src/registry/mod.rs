mod parallel;
mod simple;

pub use parallel::ParallelIriSRegistry;
pub use simple::SimpleIriSRegistry;

pub type IriRegistryIdx = usize;

pub trait IriRegistry {
    type IRI;
    type RET<'a>
    where
        Self: 'a;

    /// Register an IRI and return its index.
    fn register(&mut self, iri: Self::IRI) -> IriRegistryIdx;
    /// Look up an IRI by index.
    fn get(&self, id: &IriRegistryIdx) -> Option<Self::RET<'_>>;
}
