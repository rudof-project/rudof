use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use crate::pyconvert::convert;
use crate::pyshacl::shacl;

mod pyconvert;
mod pyshacl;

// Rudof Python bindings
#[pymodule]
fn pyrudof(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__package__", "pyrudof")?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    module.add_wrapped(wrap_pymodule!(shacl))?;
    module.add_wrapped(wrap_pymodule!(convert))?;

    Ok(())
}
