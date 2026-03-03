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
        PyCardinalityStrategy, PyCompareSchemaFormat, PyCompareSchemaMode, PyDCTAP, PyDCTapFormat, PyDataGenerator,
        PyDataQuality, PyEntityDistribution, PyGeneratorConfig, PyMie, PyOutputFormat, PyPrefixMap,
        PyQueryResultFormat, PyQueryShapeMap, PyQuerySolution, PyQuerySolutions, PyRDFFormat, PyReaderMode,
        PyResultShapeMap, PyRudof, PyRudofConfig, PyRudofError, PySchemaFormat, PyServiceDescription,
        PyServiceDescriptionFormat, PyShExFormat, PyShExFormatter, PyShExSchema, PyShaclFormat, PyShaclSchema,
        PyShaclValidationMode, PyShapeMapFormat, PyShapeMapFormatter, PyShapesGraphSource, PySortModeResultMap,
        PyUmlGenerationMode, PyValidationReport, PyValidationStatus,
    };

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}

/// Tests that embed Python and run the unittest files from `examples/`.
///
/// These are compiled and executed only when the `extension-module` feature is
/// *not* active (i.e. the normal `cargo test -p pyrudof` invocation).  With
/// that feature disabled pyo3 links against libpython, so we can call
/// `prepare_freethreaded_python` and drive the interpreter from Rust.
#[cfg(test)]
#[cfg(not(feature = "extension-module"))]
mod python_example_tests {
    use super::pyrudof; // brings the #[pymodule] mod into scope as `pyrudof`
    use pyo3::prelude::*;
    use std::ffi::CString;
    use std::path::PathBuf;
    use std::sync::Once;

    static PYTHON_INIT: Once = Once::new();

    /// Register `pyrudof` with the embedded interpreter and initialise Python.
    /// Safe to call from multiple tests; the interpreter is initialised at most once.
    fn init_python() {
        PYTHON_INIT.call_once(|| {
            // append_to_inittab! must be called before prepare_freethreaded_python.
            pyo3::append_to_inittab!(pyrudof);
            Python::initialize();
        });
    }

    /// Discover and run all `unittest.TestCase` classes found in `filename`
    /// (relative to the `examples/` directory next to this crate's Cargo.toml).
    fn run_python_unittest(filename: &str) {
        init_python();
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file_path = manifest.join("examples").join(filename);
        // Build a small Python driver that loads the file as a module and runs
        // its unittest suite, raising AssertionError on failure so that pyo3
        // propagates it back to Rust as a test failure.
        let code = format!(
            r#"
import sys, unittest, importlib.util
spec = importlib.util.spec_from_file_location("_test_module", r"{path}")
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)
suite = unittest.TestLoader().loadTestsFromModule(mod)
result = unittest.TextTestRunner(verbosity=2, stream=sys.stderr).run(suite)
if not result.wasSuccessful():
    raise AssertionError(
        f"{{len(result.failures)}} failure(s), {{len(result.errors)}} error(s)"
    )
"#,
            path = file_path.display()
        );
        Python::attach(|py| {
            let c_code = CString::new(code.as_str()).unwrap();
            py.run(c_code.as_c_str(), None, None).unwrap_or_else(|e| {
                e.print(py);
                panic!("Python tests in '{}' failed", filename);
            });
        });
    }

    #[test]
    fn test_rdf_data_example() {
        run_python_unittest("test_rdf_data.py");
    }
}
