#![allow(clippy::useless_conversion)]

#[cfg(not(target_family = "wasm"))]
use pyo3::prelude::*;
#[cfg(not(target_family = "wasm"))]
mod pyrudof_config;
#[cfg(not(target_family = "wasm"))]
mod pyrudof_generate;
#[cfg(not(target_family = "wasm"))]
mod pyrudof_lib;

#[cfg(not(target_family = "wasm"))]
pub use crate::{pyrudof_config::*, pyrudof_generate::*, pyrudof_lib::*};

// Rudof Python bindings
#[cfg(not(target_family = "wasm"))]
#[pymodule]
pub mod pyrudof {
    use super::*;

    #[pymodule_export]
    pub use super::{
        PyCardinalityStrategy, PyConversionFormat, PyConversionMode, PyDCTapFormat, PyDataGenerator, PyDataQuality,
        PyEntityDistribution, PyGeneratorConfig, PyOutputFormat, PyQueryResultFormat, PyQueryType, PyRDFFormat,
        PyReaderMode, PyResultConversionFormat, PyResultConversionMode, PyResultDCTapFormat, PyResultDataFormat,
        PyResultShaclValidationFormat, PyResultShexValidationFormat, PyRudof, PyRudofConfig, PyRudofError,
        PySchemaFormat, PyServiceDescriptionFormat, PyShExFormat, PyShaclFormat, PyShaclValidationMode,
        PyShaclValidationSortMode, PyShapeMapFormat, PyShapesGraphSource, PyShexValidationSortMode,
    };

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}
