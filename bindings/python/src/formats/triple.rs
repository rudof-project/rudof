use pyo3::prelude::*;
use rudof_lib::types::Triple;

/// A triple of the loaded RDF data.
///
/// Unpacks into its three terms, so ``for s, p, o in rudof.triples()`` works as well as
/// reading :attr:`subject`, :attr:`predicate` and :attr:`object`.
///
/// The three terms are written in the syntax :meth:`Rudof.add_triple` and
/// :meth:`Rudof.remove_triple` read (an IRI in angle brackets, a literal with the
/// lexical form it was written with) so a triple read from the graph goes back into
/// them unedited.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, from_py_object, name = "Triple", module = "pyrudof._pyrudof")]
#[derive(Clone)]
pub struct PyTriple {
    pub(crate) inner: Triple,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyTriple {
    /// The subject of the triple, an IRI or a blank node.
    #[getter]
    fn subject(&self) -> String {
        self.inner.subject.to_string()
    }

    /// The predicate IRI.
    #[getter]
    fn predicate(&self) -> String {
        self.inner.predicate.to_string()
    }

    /// The object of the triple, an IRI, a blank node, a literal or a triple term.
    #[getter]
    fn object(&self) -> String {
        self.inner.object.to_string()
    }

    /// The three terms, so that a triple can be unpacked like a tuple.
    #[gen_stub(override_return_type(type_repr = "typing.Iterator[builtins.str]"))]
    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let terms = (self.subject(), self.predicate(), self.object());
        Ok(terms.into_pyobject(py)?.as_any().try_iter()?.into_any())
    }

    fn __len__(&self) -> usize {
        3
    }

    fn __repr__(&self) -> String {
        format!(
            "Triple(subject='{}', predicate='{}', object='{}')",
            self.subject(),
            self.predicate(),
            self.object()
        )
    }
}

/// Iterator over the triples of the loaded RDF data matching a pattern.
///
/// The triples are collected by :meth:`Rudof.triples` before this object exists, so
/// ``__length_hint__`` is exact rather than a hint.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "TripleIterator", module = "pyrudof._pyrudof")]
pub struct PyTriples {
    triples: std::vec::IntoIter<Triple>,
    truncated: bool,
}

impl PyTriples {
    /// Wraps the collected triples, applying `limit` if one was requested.
    ///
    /// `triples` collects one triple past `limit` so that "exactly `limit` triples match"
    /// and "more than `limit` triples match" can be told apart here; the extra triple is
    /// dropped and remembered as [`Self::truncated`].
    pub(crate) fn new(mut triples: Vec<Triple>, limit: Option<usize>) -> Self {
        let truncated = limit.is_some_and(|limit| triples.len() > limit);
        if let Some(limit) = limit {
            triples.truncate(limit);
        }
        Self {
            triples: triples.into_iter(),
            truncated,
        }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyTriples {
    /// ``True`` when a ``limit`` cut the result short, so more triples match the pattern
    /// than this iterator will yield. Always ``False`` when no ``limit`` was given.
    #[getter]
    fn truncated(&self) -> bool {
        self.truncated
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; it is never yielded to Python, so
    // the stub must not widen the element type to `Triple | None`.
    #[gen_stub(override_return_type(type_repr = "Triple"))]
    fn __next__(&mut self) -> Option<PyTriple> {
        self.triples.next().map(|triple| PyTriple { inner: triple })
    }

    /// The number of triples still to be yielded. Exact, not an estimate.
    fn __length_hint__(&self) -> usize {
        self.triples.len()
    }
}
