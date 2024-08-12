mod shacl;

use pyo3::prelude::*;

#[pymodule]
pub mod pyshapes {
    use super::*;

    #[pymodule_export]
    use super::shacl::parse;

    #[pymodule_init]
    fn pymodule_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        module.add("__package__", "rudof")?;
        module.add("__version__", env!("CARGO_PKG_VERSION"))?;
        module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))
    }
}
