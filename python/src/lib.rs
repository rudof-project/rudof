use pyo3::prelude::*;

mod pyrudof_lib;
use crate::pyrudof_lib::*;

// Rudof Python bindings
#[pymodule]
pub mod pyrudof {
    use super::*;

    #[pymodule_export]
    use super::{
        PyDCTAP, PyDCTapFormat, PyRDFFormat, PyReaderMode, PyRudof, PyRudofConfig, PyRudofError,
        PyShExFormat, PyShExFormatter, PyShaclFormat, PyShaclValidationMode, PyShapeMapFormat,
        PyShapeMapFormatter, PyShapesGraphSource, PyUmlGenerationMode, PyValidationReport,
        PyValidationStatus,
    };

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}
