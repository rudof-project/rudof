use std::collections::hash_map::IntoIter;
use crate::registry::{IriRegistry, IriRegistryIdx};
use crate::IriS;
use std::fmt::{Display, Formatter};
use indexmap::IndexSet;

/// A single-threaded IRI registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleIriSRegistry {
    registry: IndexSet<IriS>,
}

impl SimpleIriSRegistry {
    pub fn new() -> Self {
        Self { registry: IndexSet::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item = (IriRegistryIdx, &IriS)> {
        self
            .registry
            .iter()
            .enumerate()
            .map(|(idx, iri)| (idx, iri))
    }
}

impl IriRegistry for SimpleIriSRegistry {
    type IRI = IriS;
    type RET<'a> = &'a IriS;

    fn register(&mut self, iri: Self::IRI) -> IriRegistryIdx {
        let (idx, _) = self.registry.insert_full(iri);
        idx
    }

    fn get(&self, id: &IriRegistryIdx) -> Option<Self::RET<'_>> {
        self.registry.get_index(*id)
    }
}

impl Display for SimpleIriSRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleIriRegistry {{")?;
        for (idx, iri) in self.registry.iter().enumerate() {
            write!(f, " {}: {},", idx, iri)?;
        }
        write!(f, " }}")
    }
}

impl IntoIterator for SimpleIriSRegistry {
    type Item = (IriRegistryIdx, IriS);
    type IntoIter = IntoIter<IriRegistryIdx, IriS>;

    fn into_iter(self) -> Self::IntoIter {
        // self.registry.into_iter()
        todo!()
    }
}

impl Default for SimpleIriSRegistry {
    fn default() -> Self {
        Self::new()
    }
}