use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyRDFFormat, PyReaderMode, PyResultDataFormat},
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{DataFormat, DataReaderMode, ResultDataFormat};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads RDF data from a string, file path or URL. If a SPARQL endpoint is
    /// specified, it loads data from the endpoint instead.
    ///
    /// Args:
    ///     input (str | os.PathLike, optional): Inline data, file path or URL to the RDF
    ///         data. Examples: ``"data.ttl"``, ``"http://example.org/data.rdf"``.
    ///     format (RDFFormat, optional): Serialization format. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///         - ``Lax``: Continue on errors
    ///         - ``Strict``: Fail on first error
    ///     merge (bool, optional): If ``True``, merge with existing data; if ``False``, replace current data.
    ///         Defaults to ``False``.
    ///     endpoint (str, optional): SPARQL endpoint URL to load data from. If provided, it overrides the ``input``
    ///         parameter.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     DataError: If the data is malformed (in Strict mode).
    #[pyo3(signature = (input = None, format = None, base = None, reader_mode = None,
                        merge = None, endpoint = None))]
    fn read_data(
        &mut self,
        py: Python<'_>,
        input: Option<InputArg>,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        merge: Option<bool>,
        endpoint: Option<&str>,
    ) -> Result<()> {
        let format: Option<DataFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let input = input.map(|InputArg(spec)| vec![spec]);
        let base = base.map(str::to_owned);
        let endpoint = endpoint.map(str::to_owned);

        guard::detached(py, move || {
            let mut b = self.inner.load_data();
            if let Some(i) = &input {
                b = b.with_data(i);
            }
            if let Some(f) = &format {
                b = b.with_data_format(f);
            }
            if let Some(base) = &base {
                b = b.with_base(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(merge) = merge {
                b = b.with_merge(merge);
            }
            if let Some(e) = &endpoint {
                b = b.with_endpoint(e);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current RDF data to a string.
    ///
    /// Args:
    ///     format (ResultDataFormat, optional): Output format. Defaults to ``ResultDataFormat.Compact``.
    ///
    /// Returns:
    ///     str: Serialized RDF data.
    ///
    /// Raises:
    ///     DataError: If serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_data(&mut self, py: Python<'_>, format: Option<&PyResultDataFormat>) -> Result<String> {
        let format: Option<ResultDataFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_data(w);
            if let Some(f) = &format {
                s = s.with_result_data_format(f);
            }
            s.execute()
        })
    }

    /// Dereferences an IRI and adds the retrieved triples to the current graph.
    ///
    /// Args:
    ///     uri (str): The IRI to dereference.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///     merge (bool, optional): If ``True``, merge with existing data; if ``False``,
    ///         replace current data. Defaults to ``False``.
    ///
    /// Raises:
    ///     DataError: If the IRI cannot be dereferenced or the response is malformed.
    #[pyo3(signature = (uri, reader_mode = None, merge = None))]
    fn dereference(
        &mut self,
        py: Python<'_>,
        uri: &str,
        reader_mode: Option<&PyReaderMode>,
        merge: Option<bool>,
    ) -> Result<()> {
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let uri = uri.to_owned();

        guard::detached(py, move || {
            let mut b = self.inner.dereference(&uri);
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(merge) = merge {
                b = b.with_merge(merge);
            }
            b.execute()
        })?;
        Ok(())
    }
}
