use crate::{
    api::{PyRudof, staging::PyStaging},
    error::Result,
    formats::{PyGraphDiff, PyGraphRelation, PyRDFFormat, PyReaderMode, PyResultDataFormat, PySubgraphMode, PyTriples},
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::formats::{DataFormat, DataReaderMode, IriNormalizationMode, ResultDataFormat};
use rudof_lib::types::SubgraphMode;

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
    ///     min_request_interval_ms (int, optional): Minimum milliseconds between two requests sent to
    ///         ``endpoint``, overriding the configured value (by default 1100 for remote hosts, 0 for
    ///         localhost). ``0`` disables the proactive rate limiter; a ``429 Too Many Requests``
    ///         response still makes rudof back off. Ignored without ``endpoint``.
    ///     max_concurrent_requests (int, optional): Maximum requests in flight at once to ``endpoint``
    ///         when several independent ones are needed, e.g. to fetch the nodes of a shapemap before
    ///         validating them. Defaults to the configured value, else 1.
    ///     max_nodes_per_query (int, optional): Maximum nodes one query to ``endpoint`` fetches when
    ///         fetching the nodes of a shapemap ahead of validation. ``1`` disables batching.
    ///         Defaults to the configured value, else 20.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     DataError: If the data is malformed (in Strict mode).
    #[pyo3(signature = (input = None, format = None, base = None, reader_mode = None,
                        merge = None, endpoint = None, min_request_interval_ms = None,
                        max_concurrent_requests = None, max_nodes_per_query = None))]
    #[allow(clippy::too_many_arguments)]
    fn read_data(
        &mut self,
        py: Python<'_>,
        input: Option<InputArg>,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        merge: Option<bool>,
        endpoint: Option<&str>,
        min_request_interval_ms: Option<u64>,
        max_concurrent_requests: Option<usize>,
        max_nodes_per_query: Option<usize>,
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
                b = b.with_endpoint(e).with_endpoint_limits(
                    rudof_rdf::rdf_core::EndpointLimits::default()
                        .with_min_request_interval_ms(min_request_interval_ms)
                        .with_max_concurrent_requests(max_concurrent_requests)
                        .with_max_nodes_per_query(max_nodes_per_query),
                );
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Compares two RDF graphs up to blank node renaming.
    ///
    /// Both graphs are read from their inputs; the currently loaded data is
    /// neither used nor modified. RDF 1.2 triple terms are supported.
    ///
    /// Args:
    ///     data1 (str | os.PathLike): First graph — inline data, file path or URL.
    ///     data2 (str | os.PathLike): Second graph — inline data, file path or URL.
    ///     format1 (RDFFormat, optional): Format of the first graph. Defaults to ``RDFFormat.Turtle``.
    ///     format2 (RDFFormat, optional): Format of the second graph. Defaults to ``RDFFormat.Turtle``.
    ///     base1 (str, optional): Base IRI for the first graph.
    ///     base2 (str, optional): Base IRI for the second graph.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///     mode (SubgraphMode, optional): How blank nodes may be mapped when checking whether
    ///         one graph is contained in the other. Defaults to ``SubgraphMode.Injective``.
    ///
    /// Returns:
    ///     GraphRelation: ``Isomorphic``, ``Equivalent`` (each contained in the other, not
    ///     isomorphic), ``Subgraph`` (the first contained in the second), ``Supergraph`` or
    ///     ``Different``.
    ///
    /// Raises:
    ///     InputError: If an input string, file or URL cannot be resolved.
    ///     DataError: If an input cannot be parsed or its format is not an RDF format.
    #[pyo3(signature = (data1, data2, format1 = None, format2 = None, base1 = None, base2 = None,
                        reader_mode = None, mode = None))]
    #[allow(clippy::too_many_arguments)]
    fn compare_data(
        &self,
        py: Python<'_>,
        data1: InputArg,
        data2: InputArg,
        format1: Option<&PyRDFFormat>,
        format2: Option<&PyRDFFormat>,
        base1: Option<&str>,
        base2: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        mode: Option<&PySubgraphMode>,
    ) -> Result<PyGraphRelation> {
        let (InputArg(data1), InputArg(data2)) = (data1, data2);
        let mode: Option<SubgraphMode> = mode.map(Into::into);
        let format1: Option<DataFormat> = format1.map(Into::into);
        let format2: Option<DataFormat> = format2.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base1 = base1.map(str::to_owned);
        let base2 = base2.map(str::to_owned);

        let relation = guard::detached(py, move || {
            let mut b = self.inner.compare_data(&data1, &data2);
            if let Some(f) = &format1 {
                b = b.with_format1(f);
            }
            if let Some(f) = &format2 {
                b = b.with_format2(f);
            }
            if let Some(base) = &base1 {
                b = b.with_base1(base);
            }
            if let Some(base) = &base2 {
                b = b.with_base2(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(m) = &mode {
                b = b.with_mode(m);
            }
            b.execute()
        })?;
        Ok(relation.into())
    }

    /// Compares the currently loaded RDF data with another graph, up to blank
    /// node renaming. The loaded data is the first graph and is not modified.
    ///
    /// Args:
    ///     other (str | os.PathLike): The other graph — inline data, file path or URL.
    ///     format (RDFFormat, optional): Format of ``other``. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for ``other``.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///     mode (SubgraphMode, optional): How blank nodes may be mapped when checking
    ///         containment. Defaults to ``SubgraphMode.Injective``.
    ///
    /// Returns:
    ///     GraphRelation: The relation of the loaded data to ``other``, e.g. ``Subgraph``
    ///     if the loaded data is contained in ``other``.
    ///
    /// Raises:
    ///     DataError: If no in-memory RDF data is loaded, or ``other`` cannot be parsed.
    ///     InputError: If ``other`` cannot be resolved.
    #[pyo3(signature = (other, format = None, base = None, reader_mode = None, mode = None))]
    fn compare_loaded_data(
        &self,
        py: Python<'_>,
        other: InputArg,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        mode: Option<&PySubgraphMode>,
    ) -> Result<PyGraphRelation> {
        let InputArg(other) = other;
        let format: Option<DataFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let mode: Option<SubgraphMode> = mode.map(Into::into);
        let base = base.map(str::to_owned);

        let relation = guard::detached(py, move || {
            let mut b = self.inner.compare_loaded_data(&other);
            if let Some(f) = &format {
                b = b.with_format2(f);
            }
            if let Some(base) = &base {
                b = b.with_base2(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(m) = &mode {
                b = b.with_mode(m);
            }
            b.execute()
        })?;
        Ok(relation.into())
    }

    /// Compares two RDF graphs like ``compare_data``, and also returns the triples
    /// of each graph that the other one does not have.
    ///
    /// When one graph is contained in the other the diff is exact. Otherwise the best
    /// blank node mapping is searched for, in at most ``exact_search_limit`` steps;
    /// past it a heuristic mapping is used and ``GraphDiff.exact`` is ``False``.
    ///
    /// Args:
    ///     data1 (str | os.PathLike): First graph — inline data, file path or URL.
    ///     data2 (str | os.PathLike): Second graph — inline data, file path or URL.
    ///     format1 (RDFFormat, optional): Format of the first graph. Defaults to ``RDFFormat.Turtle``.
    ///     format2 (RDFFormat, optional): Format of the second graph. Defaults to ``RDFFormat.Turtle``.
    ///     base1 (str, optional): Base IRI for the first graph.
    ///     base2 (str, optional): Base IRI for the second graph.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///     mode (SubgraphMode, optional): How blank nodes may be mapped. Defaults to
    ///         ``SubgraphMode.Injective``.
    ///     exact_search_limit (int, optional): Maximum steps of the search for the best
    ///         blank node mapping. Defaults to 100000; ``0`` uses the heuristic only.
    ///
    /// Returns:
    ///     GraphDiff: The relation, the triples only in each graph and the blank node mapping.
    ///
    /// Raises:
    ///     InputError: If an input string, file or URL cannot be resolved.
    ///     DataError: If an input cannot be parsed or its format is not an RDF format.
    #[pyo3(signature = (data1, data2, format1 = None, format2 = None, base1 = None, base2 = None,
                        reader_mode = None, mode = None, exact_search_limit = None))]
    #[allow(clippy::too_many_arguments)]
    fn diff_data(
        &self,
        py: Python<'_>,
        data1: InputArg,
        data2: InputArg,
        format1: Option<&PyRDFFormat>,
        format2: Option<&PyRDFFormat>,
        base1: Option<&str>,
        base2: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        mode: Option<&PySubgraphMode>,
        exact_search_limit: Option<usize>,
    ) -> Result<PyGraphDiff> {
        let (InputArg(data1), InputArg(data2)) = (data1, data2);
        let format1: Option<DataFormat> = format1.map(Into::into);
        let format2: Option<DataFormat> = format2.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let mode: Option<SubgraphMode> = mode.map(Into::into);
        let base1 = base1.map(str::to_owned);
        let base2 = base2.map(str::to_owned);

        let inner = guard::detached(py, move || {
            let mut b = self.inner.compare_data(&data1, &data2);
            if let Some(f) = &format1 {
                b = b.with_format1(f);
            }
            if let Some(f) = &format2 {
                b = b.with_format2(f);
            }
            if let Some(base) = &base1 {
                b = b.with_base1(base);
            }
            if let Some(base) = &base2 {
                b = b.with_base2(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(m) = &mode {
                b = b.with_mode(m);
            }
            if let Some(limit) = exact_search_limit {
                b = b.with_exact_search_limit(limit);
            }
            b.execute_diff()
        })?;
        Ok(PyGraphDiff { inner })
    }

    /// Like ``diff_data``, with the currently loaded RDF data as the first graph.
    /// The loaded data is not modified.
    ///
    /// Args:
    ///     other (str | os.PathLike): The other graph — inline data, file path or URL.
    ///     format (RDFFormat, optional): Format of ``other``. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for ``other``.
    ///     reader_mode (ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///     mode (SubgraphMode, optional): How blank nodes may be mapped. Defaults to
    ///         ``SubgraphMode.Injective``.
    ///     exact_search_limit (int, optional): Maximum steps of the search for the best
    ///         blank node mapping. Defaults to 100000.
    ///
    /// Returns:
    ///     GraphDiff: The relation of the loaded data to ``other`` and their differences.
    ///
    /// Raises:
    ///     DataError: If no in-memory RDF data is loaded, or ``other`` cannot be parsed.
    ///     InputError: If ``other`` cannot be resolved.
    #[pyo3(signature = (other, format = None, base = None, reader_mode = None, mode = None,
                        exact_search_limit = None))]
    #[allow(clippy::too_many_arguments)]
    fn diff_loaded_data(
        &self,
        py: Python<'_>,
        other: InputArg,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        mode: Option<&PySubgraphMode>,
        exact_search_limit: Option<usize>,
    ) -> Result<PyGraphDiff> {
        let InputArg(other) = other;
        let format: Option<DataFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let mode: Option<SubgraphMode> = mode.map(Into::into);
        let base = base.map(str::to_owned);

        let inner = guard::detached(py, move || {
            let mut b = self.inner.compare_loaded_data(&other);
            if let Some(f) = &format {
                b = b.with_format2(f);
            }
            if let Some(base) = &base {
                b = b.with_base2(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            if let Some(m) = &mode {
                b = b.with_mode(m);
            }
            if let Some(limit) = exact_search_limit {
                b = b.with_exact_search_limit(limit);
            }
            b.execute_diff()
        })?;
        Ok(PyGraphDiff { inner })
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
    ///     object (str): The object, as an IRI, a blank node, a literal (``'"Alice"'``,
    ///         ``'"Alice"@en'``, ``'"Shalom"@he--rtl'``, ``"23"``, ``'"23"^^xsd:integer'``)
    ///         or an RDF 1.2 triple term (``'<<( ex:alice ex:name "Alice" )>>'``, which may
    ///         nest another one).
    ///     strict_iris (bool, optional): Require angle-bracketed IRIs instead of
    ///         auto-wrapping bare ones. Defaults to ``False``.
    ///
    /// Terms are read in RDF 1.2 Turtle syntax, so an IRI written inside a triple term has
    /// to be bracketed or prefixed whatever ``strict_iris`` says. Prefixed names are
    /// resolved against the prefixes of the loaded data, supplemented by the session's
    /// default prefixes (see :meth:`add_prefix`). Adding a triple the data already
    /// contains changes nothing, an RDF graph being a set of triples.
    ///
    /// The terms a :class:`Triple` from :meth:`triples` carries are written in this same
    /// syntax, so a triple read from the graph is added back unedited, ``strict_iris``
    /// either way.
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

    /// Adds several triples to the loaded RDF data in one operation.
    ///
    /// Args:
    ///     triples (Sequence[tuple]): The triples to add, each one a
    ///         ``(subject, predicate, object)`` tuple whose terms are written as in
    ///         :meth:`add_triple`.
    ///     strict_iris (bool, optional): As in :meth:`add_triple`. Defaults to ``False``.
    ///
    /// Every triple is parsed before any of them is added, so a batch holding a term
    /// that cannot be parsed changes nothing at all. This is the operation to stage a
    /// delta with (see :meth:`staging`).
    ///
    /// Raises:
    ///     DataError: As :meth:`add_triple`, for the first triple that cannot be parsed.
    #[pyo3(signature = (triples, strict_iris = None))]
    fn add_triples(
        &mut self,
        py: Python<'_>,
        triples: Vec<(String, String, String)>,
        strict_iris: Option<bool>,
    ) -> Result<()> {
        guard::detached(py, move || {
            let triples = borrowed(&triples);
            let mut b = self.inner.add_triples(&triples);
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Removes several triples from the loaded RDF data in one operation.
    ///
    /// Args:
    ///     As :meth:`add_triples`.
    ///
    /// Raises:
    ///     DataError: As :meth:`add_triples`.
    #[pyo3(signature = (triples, strict_iris = None))]
    fn remove_triples(
        &mut self,
        py: Python<'_>,
        triples: Vec<(String, String, String)>,
        strict_iris: Option<bool>,
    ) -> Result<()> {
        guard::detached(py, move || {
            let triples = borrowed(&triples);
            let mut b = self.inner.remove_triples(&triples);
            if strict_iris.unwrap_or(false) {
                b = b.with_iri_mode(IriNormalizationMode::Strict);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Opens a staging frame over the loaded RDF data, as a context manager.
    ///
    /// Returns:
    ///     Staging: The frame, which opens on entering the ``with`` block, commits on
    ///     leaving it normally and rolls back on leaving it through an exception.
    ///
    /// Nothing is staged until the block is entered, so the returned object has no
    /// effect of its own. Only the data is staged: see :meth:`begin_staging`.
    fn staging(slf: Py<Self>) -> PyStaging {
        PyStaging::new(slf)
    }

    /// Opens a staging frame over the loaded RDF data by hand.
    ///
    /// Returns:
    ///     int: The new staging depth. Frames nest, and the innermost one is the one
    ///     :meth:`commit_staging` and :meth:`rollback_staging` close.
    ///
    /// Prefer :meth:`staging`, which closes the frame on its own even when the block
    /// is left through an exception.
    ///
    /// What is staged is the loaded **data**, and nothing else. Schemas, shapemaps,
    /// prefixes and configuration are session state: a schema loaded inside a frame
    /// stays loaded after a rollback, including one read out of the data graph itself.
    /// What a rollback clears is the state computed from the data it discards: the
    /// validation results, the query results and the loaded query.
    ///
    /// Raises:
    ///     DataError: If no RDF data is loaded, or if it cannot be modified (only the
    ///         in-memory graph can be) — checked when the frame is opened rather than
    ///         at the first write.
    fn begin_staging(&mut self, py: Python<'_>) -> Result<usize> {
        guard::detached(py, move || self.inner.begin_staging().execute())
    }

    /// Keeps what the innermost open staging frame staged, closing it.
    ///
    /// A frame nested in another hands its changes over to the enclosing frame, so
    /// rolling that one back still undoes them.
    ///
    /// Raises:
    ///     DataError: If no staging frame is open.
    fn commit_staging(&mut self, py: Python<'_>) -> Result<()> {
        guard::detached(py, move || self.inner.commit_staging().execute())
    }

    /// Discards what the innermost open staging frame staged, closing it and putting
    /// the loaded data back as it was when the frame was opened.
    ///
    /// See :meth:`begin_staging` for what is put back and what is cleared.
    ///
    /// Raises:
    ///     DataError: If no staging frame is open, or if the data could not be put
    ///         back, in which case the frame is closed all the same.
    fn rollback_staging(&mut self, py: Python<'_>) -> Result<()> {
        guard::detached(py, move || self.inner.rollback_staging().execute())
    }

    /// How many staging frames are open.
    ///
    /// Returns:
    ///     int: The staging depth, zero when nothing is staged.
    fn staging_depth(&self) -> usize {
        self.inner.staging_depth().execute()
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
    ///     predicate, object)``, written as :meth:`add_triple` reads them: an IRI in
    ///     angle brackets, a literal with the lexical form it was written with, a
    ///     triple term as ``<<( … )>>``.
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

        let triples = guard::detached(py, move || -> Result<Vec<_>> {
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
            Ok(match limit {
                // One triple past `limit`, so the iterator can report `truncated`
                // without needing a second pass over the data.
                Some(limit) => triples.take(limit.saturating_add(1)).collect(),
                None => triples.collect(),
            })
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

/// The triples of a batch as the slice of string slices `rudof_lib` takes.
fn borrowed(triples: &[(String, String, String)]) -> Vec<(&str, &str, &str)> {
    triples
        .iter()
        .map(|(subject, predicate, object)| (subject.as_str(), predicate.as_str(), object.as_str()))
        .collect()
}
