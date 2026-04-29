use crate::IriS;
use crate::registry::{IriRegistry, IriRegistryIdx};
use indexmap::IndexSet;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};

/// A thread-safe IRI registry.
#[derive(Debug)]
pub struct ParallelIriSRegistry {
    registry: RwLock<IndexSet<IriS>>,
}

impl ParallelIriSRegistry {
    pub fn new() -> Self {
        Self {
            registry: RwLock::new(IndexSet::new()),
        }
    }
}

impl IriRegistry for ParallelIriSRegistry {
    type IRI = IriS;
    type RET<'a> = Arc<IriS>;

    fn register(&mut self, iri: Self::IRI) -> IriRegistryIdx {
        let (idx, _) = self.registry.write().unwrap().insert_full(iri);
        idx
    }

    fn get(&self, id: &IriRegistryIdx) -> Option<Self::RET<'_>> {
        self.registry
            .read()
            .unwrap()
            .get_index(*id)
            .map(|iri| Arc::new(iri.clone()))
    }
}

impl Default for ParallelIriSRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ParallelIriSRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParallelIriRegistry {{")?;
        for (idx, value) in self.registry.read().map_err(|_| std::fmt::Error)?.iter().enumerate() {
            write!(f, " {}: {},", idx, value)?;
        }
        write!(f, " }}")
    }
}
