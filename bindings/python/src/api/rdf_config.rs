use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyRdfConfigFormat, PyResultRdfConfigFormat},
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{RdfConfigFormat, ResultRdfConfigFormat};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads an rdf-config document from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline document, file path or URL.
    ///     format (RdfConfigFormat, optional): Input format. Defaults to ``RdfConfigFormat.Yaml``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     RdfConfigError: If the document is malformed.
    #[pyo3(signature = (input, format = None))]
    fn read_rdf_config(
        &mut self,
        py: Python<'_>,
        input: InputArg,
        format: Option<&PyRdfConfigFormat>,
    ) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<RdfConfigFormat> = format.map(Into::into);

        py.detach(move || {
            let mut b = self.inner.load_rdf_config(&input);
            if let Some(f) = &format {
                b = b.with_rdf_config_format(f);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current rdf-config document to a string.
    ///
    /// Args:
    ///     format (ResultRdfConfigFormat, optional): Output format. Defaults to ``ResultRdfConfigFormat.Internal``.
    ///
    /// Returns:
    ///     str: Serialized rdf-config document.
    ///
    /// Raises:
    ///     RdfConfigError: If no document is loaded or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_rdf_config(
        &self,
        py: Python<'_>,
        format: Option<&PyResultRdfConfigFormat>,
    ) -> Result<String> {
        let format: Option<ResultRdfConfigFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_rdf_config(w);
            if let Some(f) = &format {
                s = s.with_result_rdf_config_format(f);
            }
            s.execute()
        })
    }
}
