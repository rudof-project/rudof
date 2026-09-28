use crate::error::{Error, Result};
use pyo3::Python;
use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Runs `f`, converting a panic inside it into [`Error::Panicked`].
pub(crate) fn catch<T, E, F>(f: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<T, E>,
    E: Into<Error>,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(e)) => Err(e.into()),
        Err(payload) => Err(Error::Panicked(message(&*payload))),
    }
}

/// [`catch`] for a closure that cannot fail on its own — a getter that formats a term, say.
pub(crate) fn catch_value<T, F>(f: F) -> Result<T>
where
    F: FnOnce() -> T,
{
    catch(move || Ok::<T, Error>(f()))
}

/// Releases the GIL for the duration of `f` and guards it against panics.
///
/// This is the only way the bindings should call a long `rudof_lib` operation: it keeps the
/// GIL-release protocol and the panic boundary in one place, so neither can be forgotten at
/// a call site.
pub(crate) fn detached<T, E, F>(py: Python<'_>, f: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<T, E> + Send,
    T: Send,
    E: Into<Error> + Send,
{
    py.detach(move || catch(f))
}

/// The panic message, recovered from the payload `catch_unwind` hands back.
fn message(payload: &(dyn Any + Send)) -> String {
    let detail = payload
        .downcast_ref::<&'static str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned());

    match detail {
        Some(detail) => format!("rudof panicked: {detail}"),
        None => "rudof panicked with a non-string payload".to_string(),
    }
}
