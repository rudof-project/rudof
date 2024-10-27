use dctap::{DCTap, TapConfig};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use shapes_converter::{Tap2ShEx, Tap2ShExConfig};
use shex_compact::ShExFormatter;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[pymodule]
pub fn convert(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(dctap2shex, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (input))]
pub fn dctap2shex(input: &str, py: Python<'_>) -> PyResult<String> {
    py.allow_threads(|| {
        let reader = input.as_bytes();
        let dctap = DCTap::from_reader(reader, &TapConfig::default())
            .map_err(|e| PyValueError::new_err(format!("Error reading DCTAP {e}")))?;
        let converter = Tap2ShEx::new(&Tap2ShExConfig::default());
        let schema = converter
            .convert(&dctap)
            .map_err(|e| PyValueError::new_err(format!("Error converting DCTAP to ShEx: {e}")))?;
        let formatter = ShExFormatter::default().without_colors();
        let str = formatter.format_schema(&schema);
        Ok(str)
    })
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShExFormat {
    ShExC,
    ShExJ,
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
        }
    }
}

impl FromStr for ShExFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shexc" => Ok(ShExFormat::ShExC),
            "shexj" => Ok(ShExFormat::ShExJ),
            _ => Err(format!("Unsupported ShExFormat: {s}")),
        }
    }
}
