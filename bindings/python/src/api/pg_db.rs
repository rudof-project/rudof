use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyDbEngine, PyDdlDialect, PyRDFFormat},
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{BackendSpec, DataFormat, DdlDialect};
use std::path::PathBuf;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Opens (creating if necessary) a property graph database and stores the connection
    /// so :meth:`load_pg_db` / :meth:`query_cypher` can reuse it without repeating the
    /// path.
    ///
    /// Args:
    ///     path (str | os.PathLike, optional): Path to the database directory. Required unless ``in_memory=True``.
    ///     in_memory (bool, optional): Create a transient in-memory database. Its connection cannot be reused,
    ///         since it does not outlive the process. Defaults to ``False``.
    ///     read_only (bool, optional): Open the database in read-only mode. Defaults to ``False``.
    ///     engine (DbEngine, optional): Database engine. Defaults to ``DbEngine.Lbug``, the only one supported today.
    ///
    /// Raises:
    ///     PgDbError: If the database cannot be opened.
    #[pyo3(signature = (path = None, in_memory = false, read_only = false, engine = None))]
    fn connect_pg_db(
        &mut self,
        py: Python<'_>,
        path: Option<PathBuf>,
        in_memory: bool,
        read_only: bool,
        engine: Option<&PyDbEngine>,
    ) -> Result<()> {
        let engine: Option<BackendSpec> = engine.map(Into::into);

        guard::detached(py, move || {
            let mut b = self
                .inner
                .connect_pg_db(path.as_deref())
                .with_in_memory(in_memory)
                .with_read_only(read_only);
            if let Some(e) = &engine {
                b = b.with_engine(e);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Derives a property graph schema from RDF data and emits it as DDL.
    ///
    /// Args:
    ///     data (str | os.PathLike): Inline data, file path or URL to the RDF data.
    ///     dialect (DdlDialect, optional): Target DDL dialect. Defaults to ``DdlDialect.Cypher``.
    ///     graph_type_name (str, optional): Graph type name used by the ``gql`` dialect. Defaults to ``"rudof_graph"``.
    ///     format (RDFFormat, optional): RDF data format. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for the data.
    ///
    /// Returns:
    ///     str: The generated DDL.
    ///
    /// Raises:
    ///     InputError: If the data cannot be resolved.
    ///     PgDbError: If the data cannot be parsed.
    #[pyo3(signature = (data, dialect = None, graph_type_name = None, format = None,
                        base = None))]
    fn pg_db_ddl(
        &self,
        py: Python<'_>,
        data: InputArg,
        dialect: Option<&PyDdlDialect>,
        graph_type_name: Option<&str>,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
    ) -> Result<String> {
        let InputArg(data) = data;
        let data = vec![data];
        let dialect: Option<DdlDialect> = dialect.map(Into::into);
        let format: Option<DataFormat> = format.map(Into::into);
        let graph_type_name = graph_type_name.map(str::to_owned);
        let base = base.map(str::to_owned);

        let ddl = guard::detached(py, move || {
            let mut b = self.inner.pg_db_ddl(&data);
            if let Some(d) = &dialect {
                b = b.with_dialect(d);
            }
            if let Some(n) = &graph_type_name {
                b = b.with_graph_type_name(n);
            }
            if let Some(f) = &format {
                b = b.with_data_format(f);
            }
            if let Some(base) = &base {
                b = b.with_base_data(base);
            }
            b.execute()
        })?;
        Ok(ddl)
    }

    /// Loads RDF data, validates it against SHACL shapes (unless ``skip_validation``),
    /// derives a property graph schema, and copies the data into the connected database.
    ///
    /// Args:
    ///     data (str | os.PathLike): Inline data, file path or URL to the RDF data.
    ///     shapes (str | os.PathLike, optional): Inline shapes, file path or URL to SHACL shapes. If not given,
    ///         shapes embedded in the data itself are used.
    ///     skip_validation (bool, optional): Skip SHACL validation and just copy the data
    ///         — the database DDL enforces conformance. Defaults to ``False``.
    ///     db (str | os.PathLike, optional): Path overriding the database connected via
    ///         :meth:`connect_pg_db`.
    ///     format (RDFFormat, optional): RDF data format. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI for the data.
    ///
    /// Returns:
    ///     str: Progress text describing what was loaded, validated, and inserted.
    ///
    /// Raises:
    ///     PgDbError: If no database is connected or loading fails.
    ///     ValidationError: If SHACL validation fails.
    #[pyo3(signature = (data, shapes = None, skip_validation = false, db = None,
                        format = None, base = None))]
    fn load_pg_db(
        &mut self,
        py: Python<'_>,
        data: InputArg,
        shapes: Option<InputArg>,
        skip_validation: bool,
        db: Option<PathBuf>,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
    ) -> Result<String> {
        let InputArg(data) = data;
        let data = vec![data];
        let shapes = shapes.map(|InputArg(spec)| spec);
        let format: Option<DataFormat> = format.map(Into::into);
        let base = base.map(str::to_owned);

        output::capture_string_detached(py, move |w| {
            let mut b = self.inner.load_pg_db(&data, w).with_skip_validation(skip_validation);
            if let Some(db) = &db {
                b = b.with_db(db, false);
            }
            if let Some(s) = &shapes {
                b = b.with_shapes(s);
            }
            if let Some(f) = &format {
                b = b.with_data_format(f);
            }
            if let Some(base) = &base {
                b = b.with_base_data(base);
            }
            b.execute().map(|_counts| ())
        })
    }

    /// Runs a Cypher query against the connected property graph database.
    ///
    /// Args:
    ///     query (str | os.PathLike): Inline query, file path or URL.
    ///     db (str | os.PathLike, optional): Path overriding the database connected via :meth:`connect_pg_db`.
    ///     read_only (bool, optional): Open the overriding database in read-only mode. Defaults to ``False``.
    ///
    /// Returns:
    ///     dict: ``{"columns": [...], "rows": [...], "compiling_time_ms": float,
    ///     "execution_time_ms": float}``.
    ///
    /// Raises:
    ///     PgDbError: If no database is connected or the query fails.
    #[pyo3(signature = (query, db = None, read_only = false))]
    fn query_cypher(
        &mut self,
        py: Python<'_>,
        query: InputArg,
        db: Option<PathBuf>,
        read_only: bool,
    ) -> Result<Py<PyAny>> {
        let InputArg(query) = query;

        let result = guard::detached(py, move || {
            let mut q = self.inner.query_cypher(&query);
            if let Some(db) = &db {
                q = q.with_db(db, read_only);
            }
            q.execute()
        })?;

        let obj = pythonize::pythonize(py, &result).map_err(|e| CoreError::Generic { error: e.to_string() })?;
        Ok(obj.unbind())
    }
}
