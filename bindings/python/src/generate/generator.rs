use crate::{
    error::Result,
    generate::{config::PyGeneratorConfig, enums::PySchemaFormat, runtime::runtime},
};
use pyo3::prelude::*;
use rudof_lib::errors::{GenerationError, RudofError as CoreError};
use std::path::PathBuf;

fn creation_error(e: impl std::fmt::Display) -> CoreError {
    GenerationError::FailedCreatingDataGenerator { error: e.to_string() }.into()
}

fn loading_error(e: impl std::fmt::Display) -> CoreError {
    GenerationError::FailedLoadingSchema { error: e.to_string() }.into()
}

fn generating_error(e: impl std::fmt::Display) -> CoreError {
    GenerationError::FailedGeneratingData { error: e.to_string() }.into()
}

/// Generates synthetic RDF data from a ShEx or SHACL schema.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "DataGenerator", module = "pyrudof._pyrudof")]
pub struct PyDataGenerator {
    inner: rudof_generate::DataGenerator,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyDataGenerator {
    /// Creates a new generator with the given configuration.
    ///
    /// Args:
    ///     config (GeneratorConfig): Configuration object.
    ///
    /// Raises:
    ///     GenerateError: If the generator cannot be initialized.
    #[new]
    fn __init__(config: &PyGeneratorConfig) -> Result<Self> {
        let inner = rudof_generate::DataGenerator::new(config.inner.clone()).map_err(creation_error)?;
        Ok(Self { inner })
    }

    /// Loads and processes a ShEx schema file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the ShEx schema file.
    ///
    /// Raises:
    ///     GenerateError: If the schema cannot be loaded or parsed.
    fn load_shex_schema(&mut self, py: Python<'_>, path: PathBuf) -> Result<()> {
        let rt = runtime()?;
        py.detach(|| rt.block_on(self.inner.load_shex_schema(&path)))
            .map_err(loading_error)?;
        Ok(())
    }

    /// Loads and processes a SHACL schema file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the SHACL schema file.
    ///
    /// Raises:
    ///     GenerateError: If the schema cannot be loaded or parsed.
    fn load_shacl_schema(&mut self, py: Python<'_>, path: PathBuf) -> Result<()> {
        let rt = runtime()?;
        py.detach(|| rt.block_on(self.inner.load_shacl_schema(&path)))
            .map_err(loading_error)?;
        Ok(())
    }

    /// Auto-detects the schema format and loads the file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the schema file.
    ///
    /// Raises:
    ///     GenerateError: If the schema cannot be loaded or parsed.
    fn load_schema_auto(&mut self, py: Python<'_>, path: PathBuf) -> Result<()> {
        let rt = runtime()?;
        py.detach(|| rt.block_on(self.inner.load_schema_auto(&path)))
            .map_err(loading_error)?;
        Ok(())
    }

    /// Generates synthetic data and writes it to the configured output.
    ///
    /// Raises:
    ///     GenerateError: If data generation fails.
    fn generate(&mut self, py: Python<'_>) -> Result<()> {
        let rt = runtime()?;
        py.detach(|| rt.block_on(self.inner.generate()))
            .map_err(generating_error)?;
        Ok(())
    }

    /// Runs the whole pipeline: load the schema, then generate.
    ///
    /// Args:
    ///     schema_path (str | os.PathLike): Path to the schema file.
    ///     format (SchemaFormat, optional): Schema format. Auto-detected when omitted.
    ///
    /// Raises:
    ///     GenerateError: If schema loading or generation fails.
    #[pyo3(signature = (schema_path, format = None))]
    fn run_with_format(&mut self, py: Python<'_>, schema_path: PathBuf, format: Option<PySchemaFormat>) -> Result<()> {
        let format = format.map(Into::into);
        let rt = runtime()?;
        py.detach(|| rt.block_on(self.inner.run_with_format(&schema_path, format)))
            .map_err(generating_error)?;
        Ok(())
    }

    /// Runs the whole pipeline with automatic schema format detection.
    ///
    /// Args:
    ///     schema_path (str | os.PathLike): Path to the schema file.
    ///
    /// Raises:
    ///     GenerateError: If schema loading or generation fails.
    fn run(&mut self, py: Python<'_>, schema_path: PathBuf) -> Result<()> {
        self.run_with_format(py, schema_path, None)
    }

    fn __repr__(&self) -> String {
        "DataGenerator()".to_string()
    }
}
