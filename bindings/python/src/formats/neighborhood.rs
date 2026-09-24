use pyo3::prelude::*;
use rudof_lib::types::{ArcDirection, NeighborArc};

pyenum! {
    /// Direction in which an arc is followed from the node being expanded.
    "ArcDirection": PyArcDirection => ArcDirection, default = Outgoing {
        /// The node is the subject of the arc.
        Outgoing => Outgoing,
        /// The node is the object of the arc.
        Incoming => Incoming,
    }
}

/// A single arc in a node's neighborhood.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, from_py_object, name = "NeighborArc", module = "pyrudof._pyrudof")]
#[derive(Clone)]
pub struct PyNeighborArc {
    pub(crate) inner: NeighborArc,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyNeighborArc {
    /// The node the neighborhood expansion started from.
    #[getter]
    fn root(&self) -> String {
        self.inner.root.to_string()
    }

    /// Whether the arc leaves or enters `node`.
    #[getter]
    fn direction(&self) -> PyArcDirection {
        self.inner.direction.into()
    }

    /// How many arcs away from `root` this arc is.
    #[getter]
    fn depth(&self) -> usize {
        self.inner.depth
    }

    /// The node this arc starts from.
    #[getter]
    fn node(&self) -> String {
        self.inner.node.to_string()
    }

    /// The predicate IRI of the arc.
    #[getter]
    fn predicate(&self) -> String {
        self.inner.predicate.to_string()
    }

    /// The node on the other end of the arc.
    #[getter]
    fn neighbor(&self) -> String {
        self.inner.neighbor.to_string()
    }

    /// ``True`` if this is the last arc of the node it expands.
    #[getter]
    fn is_last(&self) -> bool {
        self.inner.is_last
    }

    fn __repr__(&self) -> String {
        format!(
            "NeighborArc(node='{}', predicate='{}', neighbor='{}', direction={}, depth={})",
            self.node(),
            self.predicate(),
            self.neighbor(),
            self.direction().__str__(),
            self.depth()
        )
    }
}

/// Iterator over the arcs around a node, in depth-first order.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "NeighborArcIterator", module = "pyrudof._pyrudof")]
pub struct PyNodeNeighborhood {
    pub(crate) arcs: std::vec::IntoIter<NeighborArc>,
}

impl PyNodeNeighborhood {
    pub(crate) fn new(arcs: Vec<NeighborArc>) -> Self {
        Self {
            arcs: arcs.into_iter(),
        }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyNodeNeighborhood {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; it is never yielded to Python, so
    // the stub must not widen the element type to `NeighborArc | None`.
    #[gen_stub(override_return_type(type_repr = "NeighborArc"))]
    fn __next__(&mut self) -> Option<PyNeighborArc> {
        self.arcs.next().map(|arc| PyNeighborArc { inner: arc })
    }

    fn __length_hint__(&self) -> usize {
        self.arcs.len()
    }
}
