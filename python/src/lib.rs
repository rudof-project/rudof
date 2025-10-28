#![allow(clippy::useless_conversion)]
use pyo3::prelude::*;
mod pyprefixmap;
mod pyrudof_config;
mod pyrudof_generate;
mod pyrudof_lib;

pub use crate::pyprefixmap::*;
pub use crate::pyrudof_config::*;
pub use crate::pyrudof_generate::*;
pub use crate::pyrudof_lib::*;

// Rudof Python bindings
#[pymodule]
pub mod pyrudof {
    use super::*;

    #[pymodule_export]
    pub use super::{
        PyCardinalityStrategy, PyCompareSchemaFormat, PyCompareSchemaMode, PyDCTAP, PyDCTapFormat,
        PyDataGenerator, PyDataQuality, PyEntityDistribution, PyGeneratorConfig, PyMie,
        PyOutputFormat, PyPrefixMap, PyQueryResultFormat, PyQueryShapeMap, PyQuerySolution,
        PyQuerySolutions, PyRDFFormat, PyReaderMode, PyResultShapeMap, PyRudof, PyRudofConfig,
        PyRudofError, PySchemaFormat, PyServiceDescription, PyServiceDescriptionFormat,
        PyShExFormat, PyShExFormatter, PyShExSchema, PyShaclFormat, PyShaclSchema,
        PyShaclValidationMode, PyShapeMapFormat, PyShapeMapFormatter, PyShapesGraphSource,
        PySortModeResultMap, PyUmlGenerationMode, PyValidationReport, PyValidationStatus,
    };

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}
