use dctap::{DCTap, TapConfig};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use shapes_converter::{Tap2ShEx, Tap2ShExConfig};
use shex_compact::ShExFormatter;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

#[pymodule]
pub fn convert(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(dctap2shex, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (input, output))]
pub fn dctap2shex(input: &str, output: &str, py: Python<'_>) -> PyResult<()> {
    py.allow_threads(|| {
        let input = Path::new(input);
        let dctap = DCTap::from_path(input, &TapConfig::default())
            .map_err(|e| PyValueError::new_err(format!("Error reading DCTAP {e}")))?;
        let converter = Tap2ShEx::new(&Tap2ShExConfig::default());
        let schema = converter
            .convert(&dctap)
            .map_err(|e| PyValueError::new_err(format!("Error converting DCTAP to ShEx: {e}")))?;
        let formatter = ShExFormatter::default().without_colors();
        let str = formatter.format_schema(&schema);

        let output = Path::new(output);

        let mut writer = match File::create(output) {
            Ok(file) => BufWriter::new(file),
            Err(_) => return Err(PyValueError::new_err("Output file could not be created")),
        };

        if let Err(error) = writeln!(writer, "{str}") {
            return Err(PyValueError::new_err(error.to_string()));
        }

        Ok(())
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
