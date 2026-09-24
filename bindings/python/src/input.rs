use crate::error::Result;
use pyo3::{Borrowed, prelude::*, types::PyString};
use rudof_lib::{errors::InputSpecError, formats::InputSpec};
use std::{path::PathBuf, str::FromStr};

/// Parses a string as an [`InputSpec`] — inline content, a file path, or a URL.
pub(crate) fn input_spec(s: &str) -> Result<InputSpec> {
    let spec =
        InputSpec::from_str(s).map_err(|e| InputSpecError::InvalidInput { error: e.to_string() })?;
    Ok(spec)
}

/// A "content, path, or URL" parameter.
pub(crate) struct InputArg(pub(crate) InputSpec);

impl<'a, 'py> FromPyObject<'a, 'py> for InputArg {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        let raw = match ob.cast::<PyString>() {
            Ok(s) => s.to_cow()?.into_owned(),
            Err(_) => ob.extract::<PathBuf>()?.to_string_lossy().into_owned(),
        };
        Ok(InputArg(input_spec(&raw)?))
    }
}

#[cfg(feature = "stub-gen")]
impl ::pyo3_stub_gen::PyStubType for InputArg {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::builtin("str")
            | ::pyo3_stub_gen::TypeInfo::with_module("os.PathLike[str]", "os".into())
    }
}
