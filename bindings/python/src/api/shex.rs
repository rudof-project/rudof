use crate::{
    api::PyRudof,
    error::Result,
    formats::{
        PyReaderMode, PyResultShexValidationFormat, PyShExFormat, PyShExValidationReport, PyShexValidationSortMode,
    },
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{DataReaderMode, ResultShExValidationFormat, ShExFormat, ShExValidationSortByMode};
use std::{io::BufWriter, path::PathBuf};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Checks whether a ShEx schema is well-formed, without loading it into the session.
    ///
    /// Parses the schema, compiles it to IR, and checks for negative dependency cycles,
    /// without affecting any currently loaded schema.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline schema, file path or URL.
    ///     format (ShExFormat, optional): Schema format. Defaults to ``ShExFormat.ShExC``.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///
    /// Returns:
    ///     tuple[bool, str]: Whether the schema is well-formed, and a human-readable
    ///     message describing the result (or the error found).
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    #[pyo3(signature = (input, format = None, base = None))]
    fn check_shex(
        &self,
        py: Python<'_>,
        input: InputArg,
        format: Option<&PyShExFormat>,
        base: Option<&str>,
    ) -> Result<(bool, String)> {
        let InputArg(input) = input;
        let format: Option<ShExFormat> = format.map(Into::into);
        let base = base.map(str::to_owned);

        let out = py.detach(move || {
            let mut writer = BufWriter::new(Vec::new());
            let is_valid = {
                let mut b = self.inner.check_shex_schema(&input, &mut writer);
                if let Some(f) = &format {
                    b = b.with_shex_schema_format(f);
                }
                if let Some(base) = &base {
                    b = b.with_base(base);
                }
                b.execute()?
            };
            let bytes = writer
                .into_inner()
                .map_err(|e| CoreError::Generic { error: e.to_string() })?;
            let message = String::from_utf8(bytes).map_err(|e| CoreError::Generic { error: e.to_string() })?;
            Ok::<_, CoreError>((is_valid, message))
        })?;
        Ok(out)
    }

    /// Loads a ShEx schema from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline schema, file path or URL.
    ///     format (ShExFormat, optional): Schema format. Defaults to ``ShExFormat.ShExC``.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     ShExError: If the schema is malformed.
    #[pyo3(signature = (input, format = None, base = None, reader_mode = None))]
    fn read_shex(
        &mut self,
        py: Python<'_>,
        input: InputArg,
        format: Option<&PyShExFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<ShExFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base = base.map(str::to_owned);

        py.detach(move || {
            let mut b = self.inner.load_shex_schema(&input);
            if let Some(f) = &format {
                b = b.with_shex_schema_format(f);
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

    /// Serializes the current ShEx schema to a string.
    ///
    /// Args:
    ///     shape_label (str, optional): Restrict the output to a single shape.
    ///     show_dependencies (bool, optional): Include the shape dependency graph.
    ///     show_statistics (bool, optional): Include schema statistics.
    ///     show_schema (bool, optional): Include the schema itself.
    ///     show_time (bool, optional): Include elapsed time.
    ///     format (ShExFormat, optional): Output format. Defaults to ``ShExFormat.ShExC``.
    ///
    /// Returns:
    ///     str: Serialized ShEx schema.
    ///
    /// Raises:
    ///     ShExError: If no schema is loaded or serialization fails.
    #[pyo3(signature = (shape_label = None, show_dependencies = None, show_statistics = None,
                        show_schema = None, show_time = None, format = None))]
    fn serialize_current_shex(
        &self,
        py: Python<'_>,
        shape_label: Option<&str>,
        show_dependencies: Option<bool>,
        show_statistics: Option<bool>,
        show_schema: Option<bool>,
        show_time: Option<bool>,
        format: Option<&PyShExFormat>,
    ) -> Result<String> {
        let format: Option<ShExFormat> = format.map(Into::into);
        let shape_label = shape_label.map(str::to_owned);

        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_shex_schema(w);
            if let Some(f) = &format {
                s = s.with_result_shex_format(f);
            }
            if let Some(l) = &shape_label {
                s = s.with_shape(l);
            }
            if let Some(d) = show_dependencies {
                s = s.with_show_dependencies(d);
            }
            if let Some(st) = show_statistics {
                s = s.with_show_statistics(st);
            }
            if let Some(sc) = show_schema {
                s = s.with_show_schema(sc);
            }
            if let Some(t) = show_time {
                s = s.with_show_time(t);
            }
            s.execute()
        })
    }

    /// Writes the currently loaded ShEx schema's compiled IR to a cache file.
    ///
    /// The cache can later be loaded quickly with :meth:`read_shex_precompiled`,
    /// skipping parsing, imports, and AST-to-IR compilation.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path of the cache file to write.
    ///
    /// Raises:
    ///     ShExError: If no ShEx schema is loaded or the file cannot be written.
    fn compile_shex_to_file(&self, py: Python<'_>, path: PathBuf) -> Result<()> {
        py.detach(move || {
            let file = std::fs::File::create(&path).map_err(|e| CoreError::Generic { error: e.to_string() })?;
            let mut writer = BufWriter::new(file);
            self.inner.compile_shex_schema_to_file(&mut writer).execute()
        })?;
        Ok(())
    }

    /// Loads a precompiled ShEx schema cache produced by :meth:`compile_shex_to_file`.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the cache file.
    ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     InputError: If the path cannot be resolved.
    ///     ShExError: If the cache file is invalid.
    #[pyo3(signature = (path, reader_mode = None))]
    fn read_shex_precompiled(
        &mut self,
        py: Python<'_>,
        path: InputArg,
        reader_mode: Option<&PyReaderMode>,
    ) -> Result<()> {
        let InputArg(path) = path;
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);

        py.detach(move || {
            let mut b = self.inner.load_shex_schema_precompiled(&path);
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Validates the current RDF data against the loaded ShEx schema using the current ShapeMap.
    ///
    /// Returns:
    ///     ShExValidationReport: A snapshot of the result — ``.conforms``,
    ///     ``.violations``, ``len()`` and iteration. Unaffected by later calls on the
    ///     session.
    ///
    /// Raises:
    ///     ValidationError: If no schema, data, or ShapeMap is loaded.
    fn validate_shex(&mut self, py: Python<'_>) -> Result<PyShExValidationReport> {
        py.detach(|| self.inner.validate_shex().execute())?;
        let results = self
            .inner
            .shex_validation_results()
            .cloned()
            .ok_or(CoreError::Generic {
                error: "validate_shex produced no results".into(),
            })?;
        Ok(PyShExValidationReport::new(results))
    }

    /// Serializes the results of the most recent :meth:`validate_shex` call.
    ///
    /// Args:
    ///     format (ResultShexValidationFormat, optional): Output format. Defaults to
    ///         ``ResultShexValidationFormat.Details``.
    ///     sort_mode (ShexValidationSortMode, optional): Sorting mode. Defaults to
    ///         ``ShexValidationSortMode.Node``.
    ///
    /// Returns:
    ///     str: Serialized validation results.
    ///
    /// Raises:
    ///     ValidationError: If there are no results to serialize.
    #[pyo3(signature = (format = None, sort_mode = None))]
    fn serialize_shex_validation_results(
        &self,
        py: Python<'_>,
        format: Option<&PyResultShexValidationFormat>,
        sort_mode: Option<&PyShexValidationSortMode>,
    ) -> Result<String> {
        let format: Option<ResultShExValidationFormat> = format.map(Into::into);
        let sort_mode: Option<ShExValidationSortByMode> = sort_mode.map(Into::into);

        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_shex_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_shex_validation_format(f);
            }
            if let Some(m) = &sort_mode {
                s = s.with_shex_validation_sort_order_mode(m);
            }
            s.execute()
        })
    }

    /// Registers an external-shape resolver from a spec string.
    ///
    /// The spec follows the grammar ``<kind>[:<arg>]``. Built-in kinds (see :meth:`list_external_resolvers`):
    /// * ``reject-all`` — reject any unhandled EXTERNAL shape.
    /// * ``schema:<path>`` — substitute EXTERNAL declarations using a ShEx file.
    ///
    /// Resolvers are prepended to the chain, so the most recently added is consulted
    /// first. Call this **before** :meth:`read_shex`, because the compiler reads the
    /// chain during AST-to-IR.
    ///
    /// Args:
    ///     spec (str): Resolver spec string.
    ///
    /// Raises:
    ///     ShExError: If the spec is malformed or the resolver cannot be built.
    fn add_external_resolver(&mut self, spec: &str) -> Result<()> {
        self.inner.add_external_resolver(spec)?;
        Ok(())
    }

    /// Resets the external-shape resolver chain to the default (``reject-all`` only).
    fn clear_external_resolvers(&mut self) {
        self.inner.clear_external_resolvers();
    }

    /// Returns the built-in external-shape resolver kinds.
    ///
    /// Returns:
    ///     list[tuple[str, str, str]]: ``(name, description, spec_syntax)`` triples
    ///     documenting the kinds accepted by :meth:`add_external_resolver`.
    #[staticmethod]
    fn list_external_resolvers() -> Vec<(String, String, String)> {
        rudof_lib::Rudof::list_external_resolvers()
            .into_iter()
            .map(|info| {
                (
                    info.name.to_string(),
                    info.description.to_string(),
                    info.spec_syntax.to_string(),
                )
            })
            .collect()
    }
}
