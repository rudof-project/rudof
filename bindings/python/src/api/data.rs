use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyRDFFormat, PyReaderMode, PyResultDataFormat, PyTriples},
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{DataFormat, DataReaderMode, IriNormalizationMode, ResultDataFormat};

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

    /// Adds a triple to the loaded RDF data.
    ///
    /// Args:
    ///     subject (str): The subject, as an IRI (``"<http://example.org/alice>"``,
    ///         ``"ex:alice"``) or a blank node (``"_:b1"``).
    ///     predicate (str): The predicate, as an angle-bracketed IRI or a prefixed name.
    ///     object (str): The object, as an IRI, a blank node or a literal (``'"Alice"'``,
    ///         ``'"Alice"@en'``, ``"23"``, ``'"23"^^xsd:integer'``).
    ///     strict_iris (bool, optional): Require angle-bracketed IRIs instead of
    ///         auto-wrapping bare ones. Defaults to ``False``.
    ///
    /// Prefixed names are resolved against the prefixes of the loaded data, supplemented
    /// by the session's default prefixes (see :meth:`add_prefix`). Adding a triple the
    /// data already contains changes nothing, an RDF graph being a set of triples.
    ///
    /// Raises:
    ///     DataError: If no RDF data is loaded, if a term cannot be parsed or its prefix
    ///         resolved, or if the data cannot be modified (only the in-memory graph
    ///         can be, not a SPARQL endpoint, the QLever backend, or data federated with
    ///         SPARQL endpoints).
    #[pyo3(signature = (subject, predicate, object, strict_iris = None))]
    fn add_triple(
        &mut self,
        py: Python<'_>,
        subject: &str,
        predicate: &str,
        object: &str,
        strict_iris: Option<bool>,
    ) -> Result<()> {
        let (subject, predicate, object) = (subject.to_owned(), predicate.to_owned(), object.to_owned());

        guard::detached(py, move || {
            let mut b = self.inner.add_triple(&subject, &predicate, &object);
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Removes a triple from the loaded RDF data.
    ///
    /// Args:
    ///     As :meth:`add_triple`.
    ///
    /// Removing a triple the data does not contain changes nothing.
    ///
    /// Raises:
    ///     DataError: As :meth:`add_triple`.
    #[pyo3(signature = (subject, predicate, object, strict_iris = None))]
    fn remove_triple(
        &mut self,
        py: Python<'_>,
        subject: &str,
        predicate: &str,
        object: &str,
        strict_iris: Option<bool>,
    ) -> Result<()> {
        let (subject, predicate, object) = (subject.to_owned(), predicate.to_owned(), object.to_owned());

        guard::detached(py, move || {
            let mut b = self.inner.remove_triple(&subject, &predicate, &object);
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Returns an iterator over the triples of the loaded RDF data matching a pattern.
    ///
    /// Args:
    ///     subject (str, optional): The subject to match, as in :meth:`add_triple`.
    ///         Omitted or ``None`` matches any subject.
    ///     predicate (str, optional): The predicate to match; ``None`` matches any.
    ///     object (str, optional): The object to match; ``None`` matches any.
    ///     strict_iris (bool, optional): As in :meth:`add_triple`. Defaults to ``False``.
    ///     limit (int, optional): Stop after this many triples. Unbounded when omitted.
    ///
    /// Omitting all three positions walks the whole graph.
    ///
    /// Returns:
    ///     TripleIterator: The matching triples, each unpacking into ``(subject,
    ///     predicate, object)``.
    ///
    /// Raises:
    ///     DataError: If no RDF data is loaded, or if a term of the pattern cannot be
    ///         parsed or its prefix resolved.
    ///
    /// Note:
    ///     The triples are materialized before this returns, so an unconstrained pattern
    ///     allocates the whole graph and breaking out of the loop early saves nothing.
    ///     Pass ``limit`` to bound the work and check
    ///     :attr:`TripleIterator.truncated` to learn whether it cut the result short.
    ///     With a SPARQL endpoint or the QLever backend as the data, an unconstrained
    ///     pattern pulls the whole remote graph.
    #[pyo3(signature = (subject = None, predicate = None, object = None, strict_iris = None,
                        limit = None))]
    fn triples(
        &self,
        py: Python<'_>,
        subject: Option<&str>,
        predicate: Option<&str>,
        object: Option<&str>,
        strict_iris: Option<bool>,
        limit: Option<usize>,
    ) -> Result<PyTriples> {
        let subject = subject.map(str::to_owned);
        let predicate = predicate.map(str::to_owned);
        let object = object.map(str::to_owned);

        let triples = guard::detached(py, move || {
            let mut b = self.inner.triples();
            if let Some(s) = &subject {
                b = b.with_subject(s);
            }
            if let Some(p) = &predicate {
                b = b.with_predicate(p);
            }
            if let Some(o) = &object {
                b = b.with_object(o);
            }
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            let triples = b.execute()?;
            match limit {
                // One triple past `limit`, so the iterator can report `truncated`
                // without needing a second pass over the data.
                Some(limit) => triples
                    .take(limit.saturating_add(1))
                    .collect::<std::result::Result<Vec<_>, _>>(),
                None => triples.collect(),
            }
        })?;

        Ok(PyTriples::new(triples, limit))
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
