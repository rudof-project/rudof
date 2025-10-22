#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
mod py_prefixmap;
mod pyrudof_config;
mod pyrudof_lib;

pub use crate::py_prefixmap::*;
pub use crate::pyrudof_config::*;
pub use crate::pyrudof_lib::*;

// Rudof Python bindings
#[pymodule]
pub mod pyrudof {
    use super::*;

    #[pymodule_export]
    pub use super::{
        PyCompareSchemaFormat, PyCompareSchemaMode, PyDCTAP, PyDCTapFormat, PyMie, PyPrefixMap,
        PyQueryResultFormat, PyQuerySolution, PyQuerySolutions, PyRDFFormat, PyReaderMode,
        PyResultShapeMap, PyRudof, PyRudofConfig, PyRudofError, PyServiceDescription,
        PyServiceDescriptionFormat, PyShExFormat, PyShExFormatter, PyShaclFormat,
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
