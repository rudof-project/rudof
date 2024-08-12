use pyo3::prelude::*;

use crate::shacl::parse;
use crate::shacl::validate;

mod shacl;

// Rudof Python bindings
#[pymodule]
fn rudof(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__package__", "rudof")?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    module.add_function(wrap_pyfunction!(parse, module)?)?;
    module.add_function(wrap_pyfunction!(validate, module)?)?;

    Ok(())
}
