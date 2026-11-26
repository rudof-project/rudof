use crate::{api::PyRudof, error::Result, guard};
use pyo3::prelude::*;
use rudof_lib::errors::DataError as CoreDataError;

/// A staging frame over the loaded RDF data, used as a context manager.
///
/// Returned by :meth:`Rudof.staging`. Entering the ``with`` block opens the frame;
/// leaving it normally keeps what was staged, and leaving it through an exception
/// discards it.
///
/// Only the data is staged. Schemas, shapemaps and prefixes are session state and are
/// left alone by both outcomes; what a rollback also clears is the state computed from
/// the discarded data: the validation results, the query results and the loaded query.
/// A report returned by :meth:`Rudof.validate_shacl` is a copy, so one held by the
/// caller survives the rollback.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "Staging", module = "pyrudof._pyrudof")]
pub struct PyStaging {
    rudof: Py<PyRudof>,

    /// The staging depth this object owns, set when the ``with`` block is entered.
    depth: Option<usize>,
}

impl PyStaging {
    pub(crate) fn new(rudof: Py<PyRudof>) -> Self {
        Self { rudof, depth: None }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyStaging {
    /// Opens the staging frame.
    ///
    /// Returns:
    ///     Rudof: The session the frame was opened on, so ``as`` can name it.
    ///
    /// Raises:
    ///     DataError: If no RDF data is loaded, if it cannot be modified, or if this
    ///         object has already been used as a context manager.
    fn __enter__(&mut self, py: Python<'_>) -> Result<Py<PyRudof>> {
        if self.depth.is_some() {
            return Err(Box::new(CoreDataError::NoStagingFrame {
                operation: "re-entering the same Staging object".to_string(),
            })
            .into());
        }

        let rudof = self.rudof.clone_ref(py);
        let depth = guard::catch(|| self.rudof.borrow_mut(py).inner.begin_staging().execute())?;
        self.depth = Some(depth);
        Ok(rudof)
    }

    /// Closes the staging frame: commits when the block is left normally, rolls back
    /// when it is left through an exception.
    ///
    /// Frames opened inside the block and left open are closed the same way. A
    /// :meth:`Rudof.reset_all` inside the block leaves nothing to close, and this
    /// does nothing.
    ///
    /// Raises:
    ///     DataError: If the frame cannot be closed. The exception that caused the
    ///         rollback, if any, is kept as the context of this one.
    #[pyo3(signature = (exc_type = None, _exc_value = None, _traceback = None))]
    fn __exit__(
        &mut self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        _exc_value: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> Result<bool> {
        let Some(depth) = self.depth.take() else {
            return Ok(false);
        };
        let commit = exc_type.is_none();

        guard::catch(|| {
            let mut rudof = self.rudof.borrow_mut(py);
            while rudof.inner.staging_depth().execute() >= depth {
                if commit {
                    rudof.inner.commit_staging().execute()?;
                } else {
                    rudof.inner.rollback_staging().execute()?;
                }
            }
            Ok::<(), rudof_lib::errors::RudofError>(())
        })?;

        // Never swallow the exception that caused the rollback.
        Ok(false)
    }

    fn __repr__(&self) -> String {
        match self.depth {
            Some(depth) => format!("Staging(depth={depth})"),
            None => "Staging()".to_string(),
        }
    }
}
