use crate::error::{Error, Result};
use pyo3::Python;
use rudof_lib::errors::RudofError as CoreError;
use std::io::BufWriter;

/// Runs `f` with a byte writer and returns the captured UTF-8 output.
pub(crate) fn capture_string_detached<F>(py: Python<'_>, f: F) -> Result<String>
where
    F: FnOnce(&mut BufWriter<Vec<u8>>) -> std::result::Result<(), CoreError> + Send,
{
    py.detach(move || {
        let mut writer = BufWriter::new(Vec::new());
        f(&mut writer)?;
        finish(writer)
    })
}

fn finish(writer: BufWriter<Vec<u8>>) -> Result<String> {
    let bytes = writer
        .into_inner()
        .map_err(|e| CoreError::Generic { error: e.to_string() })?;
    String::from_utf8(bytes).map_err(|e| Error(CoreError::Generic { error: e.to_string() }))
}
