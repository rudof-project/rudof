use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyDCTapFormat, PyResultDCTapFormat},
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{DCTapFormat, ResultDCTapFormat};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a DCTAP profile from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline profile, file path or URL.
    ///     format (DCTapFormat, optional): Input format. Defaults to ``DCTapFormat.Csv``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     DCTapError: If the profile is malformed.
    #[pyo3(signature = (input, format = None))]
    fn read_dctap(&mut self, py: Python<'_>, input: InputArg, format: Option<&PyDCTapFormat>) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<DCTapFormat> = format.map(Into::into);

        py.detach(move || {
            let mut b = self.inner.load_dctap(&input);
            if let Some(f) = &format {
                b = b.with_dctap_format(f);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current DCTAP profile to a string.
    ///
    /// Args:
    ///     format (ResultDCTapFormat, optional): Output format. Defaults to ``ResultDCTapFormat.Internal``.
    ///
    /// Returns:
    ///     str: Serialized DCTAP profile.
    ///
    /// Raises:
    ///     DCTapError: If no profile is loaded or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_dctap(&self, py: Python<'_>, format: Option<&PyResultDCTapFormat>) -> Result<String> {
        let format: Option<ResultDCTapFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_dctap(w);
            if let Some(f) = &format {
                s = s.with_result_dctap_format(f);
            }
            s.execute()
        })
    }
}
