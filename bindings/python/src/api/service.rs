use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyRDFFormat, PyReaderMode, PyServiceDescriptionFormat},
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{DataFormat, DataReaderMode, ResultServiceFormat};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a service description from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline description, file path or URL, in RDF.
    ///     format (RDFFormat, optional): RDF format. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     ServiceError: If the description is malformed.
    #[pyo3(signature = (input, format = None, base = None, reader_mode = None))]
    fn read_service_description(
        &mut self,
        py: Python<'_>,
        input: InputArg,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<DataFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base = base.map(str::to_owned);

        py.detach(move || {
            let mut b = self.inner.load_service_description(&input);
            if let Some(f) = &format {
                b = b.with_data_format(f);
            }
            if let Some(base) = &base {
                b = b.with_base(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current service description to a string.
    ///
    /// Args:
    ///     format (ServiceDescriptionFormat, optional): Output format. Defaults to ``ServiceDescriptionFormat.Internal``.
    ///
    /// Returns:
    ///     str: Serialized service description.
    ///
    /// Raises:
    ///     ServiceError: If no description is loaded or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_service_description(
        &self,
        py: Python<'_>,
        format: Option<&PyServiceDescriptionFormat>,
    ) -> Result<String> {
        let format: Option<ResultServiceFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_service_description(w);
            if let Some(f) = &format {
                s = s.with_result_service_format(f);
            }
            s.execute()
        })
    }
}
