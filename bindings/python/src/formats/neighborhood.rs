use crate::{error::Result, guard};
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
    ///
    /// Raises:
    ///     InternalError: If the term has no string rendering yet. `Display for Object` is
    ///         unimplemented for RDF 1.2 triple terms, so an arc that reaches one panics on
    ///         being formatted; the guard turns that into a catchable error. Remove the
    ///         guard once terms are returned typed instead of pre-rendered.
    #[getter]
    fn root(&self) -> Result<String> {
        guard::catch_value(|| self.inner.root.to_string())
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
    ///
    /// Raises:
    ///     InternalError: As :attr:`root`.
    #[getter]
    fn node(&self) -> Result<String> {
        guard::catch_value(|| self.inner.node.to_string())
    }

    /// The predicate IRI of the arc.
    #[getter]
    fn predicate(&self) -> String {
        // A predicate is always an `IriS`, which always formats.
        self.inner.predicate.to_string()
    }

    /// The node on the other end of the arc.
    ///
    /// Raises:
    ///     InternalError: As :attr:`root`. This is the getter RDF 1.2 data actually reaches:
    ///         a reifier's ``rdf:reifies`` arc has a triple term as its neighbor.
    #[getter]
    fn neighbor(&self) -> Result<String> {
        guard::catch_value(|| self.inner.neighbor.to_string())
    }

    /// ``True`` if this is the last arc of the node it expands.
    #[getter]
    fn is_last(&self) -> bool {
        self.inner.is_last
    }

    // `repr()` has to work unconditionally — it is what `print`, logging and debuggers
    // call — so an unformattable term degrades to a placeholder here instead of raising the
    // `InternalError` the getters raise.
    fn __repr__(&self) -> String {
        format!(
            "NeighborArc(node='{}', predicate='{}', neighbor='{}', direction={}, depth={})",
            or_placeholder(self.node()),
            self.predicate(),
            or_placeholder(self.neighbor()),
            self.direction().__str__(),
            self.depth()
        )
    }
}

/// The rendered term, or a marker for one `rudof_rdf` cannot render yet.
fn or_placeholder(term: Result<String>) -> String {
    term.unwrap_or_else(|_| "<unrenderable>".to_string())
}

/// Iterator over the arcs around a node, in depth-first order.
///
/// The arcs are collected by :meth:`Rudof.node_neighborhood` before this object exists, so
/// ``__length_hint__`` is exact rather than a hint.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "NeighborArcIterator", module = "pyrudof._pyrudof")]
pub struct PyNodeNeighborhood {
    pub(crate) arcs: std::vec::IntoIter<NeighborArc>,
    truncated: bool,
}

impl PyNodeNeighborhood {
    /// Wraps the collected arcs, applying `limit` if one was requested.
    ///
    /// `node_neighborhood` collects one arc past `limit` so that "exactly `limit` arcs
    /// exist" and "more than `limit` arcs exist" can be told apart here; the extra arc is
    /// dropped and remembered as [`Self::truncated`].
    pub(crate) fn new(mut arcs: Vec<NeighborArc>, limit: Option<usize>) -> Self {
        let truncated = limit.is_some_and(|limit| arcs.len() > limit);
        if let Some(limit) = limit {
            arcs.truncate(limit);
        }
        Self {
            arcs: arcs.into_iter(),
            truncated,
        }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyNodeNeighborhood {
    /// ``True`` when a ``limit`` cut the neighborhood short, so more arcs exist than this
    /// iterator will yield. Always ``False`` when no ``limit`` was given.
    #[getter]
    fn truncated(&self) -> bool {
        self.truncated
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; it is never yielded to Python, so
    // the stub must not widen the element type to `NeighborArc | None`.
    #[gen_stub(override_return_type(type_repr = "NeighborArc"))]
    fn __next__(&mut self) -> Option<PyNeighborArc> {
        self.arcs.next().map(|arc| PyNeighborArc { inner: arc })
    }

    /// The number of arcs still to be yielded. Exact, not an estimate.
    fn __length_hint__(&self) -> usize {
        self.arcs.len()
    }
}
