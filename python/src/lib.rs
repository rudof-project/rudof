#![allow(clippy::useless_conversion)]
use pyo3::prelude::*;
mod pyrudof_config;
mod pyrudof_generate;
mod pyrudof_lib;

pub use crate::pyrudof_config::*;
pub use crate::pyrudof_generate::*;
pub use crate::pyrudof_lib::*;

// Rudof Python bindings
#[pymodule]
pub mod pyrudof {
    use super::*;

    #[pymodule_export]
    pub use super::{
        PyCardinalityStrategy, PyDCTapFormat, PyDataGenerator, PyDataQuality, PyEntityDistribution, PyGeneratorConfig,
        PyOutputFormat, PyQueryResultFormat, PyQueryType, PyRDFFormat, PyReaderMode, PyResultDataFormat,
        PyResultShexValidationFormat, PyRudof, PyRudofConfig, PyRudofError, PySchemaFormat,
        PyServiceDescriptionFormat, PyShExFormat, PyShaclFormat, PyShaclValidationMode, PyShapeMapFormat,
        PyShapesGraphSource, PySortModeResultMap,
    };

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}