use std::marker::PhantomData;
use std::path::Path;
use crate::common::TestSuiteError;

mod entry;
mod graph;
mod node_expr;
mod validate;

pub(crate) use entry::ManifestEntry;
pub(crate) use graph::ManifestGraph;
pub(crate) use node_expr::NodeExprManifest;
pub(crate) use validate::ValidateManifest;

/// A test manifest of kind `T`
pub(crate) struct Manifest<T> {
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