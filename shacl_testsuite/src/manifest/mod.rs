use crate::TestSuiteError;
use std::marker::PhantomData;
use std::path::Path;

mod entry;
mod graph;
mod node_expr;
mod validate;
mod walker;

pub use entry::ManifestEntry;
pub use graph::ManifestGraph;
pub use node_expr::NodeExprManifest;
pub use validate::{TestInstance, ValidateManifest};
pub use walker::{ManifestWalker, ValidateEntry};

/// A test manifest of kind `T`
pub struct Manifest<T> {
    graph: ManifestGraph,
    _entry: PhantomData<T>,
}

impl<T: ManifestEntry> Manifest<T> {
    pub fn new(path: &Path) -> Result<Self, TestSuiteError> {
        Ok(Self {
            graph: ManifestGraph::new(path)?,
            _entry: PhantomData,
        })
    }

    pub fn graph(&self) -> &ManifestGraph {
        &self.graph
    }

    pub fn collect_tests(&mut self) -> Result<Vec<T>, TestSuiteError> {
        self.graph
            .entries()
            .to_vec()
            .into_iter()
            .map(|entry| T::parse(&mut self.graph, entry.try_into()?))
            .collect()
    }
}
