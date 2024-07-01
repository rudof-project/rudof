use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use shacl_ast::{ShaclParser, ShaclWriter};
use srdf::{RDFFormat, SRDFGraph};
use std::path::Path;
use std::str::FromStr;
use std::io::BufWriter;
use std::fs::File;
use std::ffi::OsStr;

#[pyfunction]
#[pyo3(signature = (input, output))]
pub fn parse(
    input: &str, 
    output: &str,
    py: Python<'_>
) -> PyResult<()> {
    py.allow_threads(|| {
        let input = Path::new(input);
        let output = Path::new(output);

        let writer = match File::create(output) {
            Ok(file) => BufWriter::new(file),
            Err(_) => return Err(PyValueError::new_err("Output file could not be created")),
        };

        let mut shacl_writer: ShaclWriter<SRDFGraph> = ShaclWriter::new();

        let input_format = obtain_format(input.extension())?;
        let graph = match SRDFGraph::from_path(input, &input_format, None) {
            Ok(graph) => graph,
            Err(error) => return Err(PyValueError::new_err(error.to_string())),
        };
        let schema = match ShaclParser::new(graph).parse() {
            Ok(schema) => schema,
            Err(error) => return Err(PyValueError::new_err(error.to_string())),
        };
    
        if let Err(error) = shacl_writer.write(&schema) {
            return Err(PyValueError::new_err(error.to_string()))
        }

        let output_format = obtain_format(output.extension())?;

        if let Err(error) = shacl_writer.serialize(output_format, writer) {
            return Err(PyValueError::new_err(error.to_string()))
        }

        Ok(())
    })
}

fn obtain_format(extension: Option<&OsStr>) -> PyResult<RDFFormat> {
    match extension {
        None => Err(PyValueError::new_err("No ouput format is provided")),
        Some(os_str) => match os_str.to_str() {
            Some(str) => match RDFFormat::from_str(str) {
                Ok(format) => Ok(format),
                Err(error) => Err(PyValueError::new_err(error.to_string())),
            },
            None => Err(PyValueError::new_err("{os_str} is not supported")),
        }
    }
}