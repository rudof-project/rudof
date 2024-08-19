use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use crate::pyshacl::shacl;

mod pyshacl;

// Rudof Python bindings
#[pymodule]
fn rudof(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__package__", "rudof")?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    module.add_wrapped(wrap_pymodule!(shacl))?;

    Ok(())
}
