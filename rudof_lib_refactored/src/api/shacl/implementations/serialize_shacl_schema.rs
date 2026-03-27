use shacl_rdf::ShaclWriter;

use crate::{Rudof, Result, formats::ShaclFormat, errors::ShaclError};
use std::io;

pub fn serialize_shacl_schema<W: io::Write>(
    rudof: &Rudof,
    shacl_format: Option<&ShaclFormat>,
    writer: &mut W,
) -> Result<()> {
    let shacl_format = shacl_format.copied().unwrap_or_default();

    let shacl_shapes = rudof.shacl_shapes.as_ref().ok_or_else(|| ShaclError::NoShaclShapesLoaded)?;

    match shacl_format {
        ShaclFormat::Internal => {
            write!(writer, "{shacl_shapes}").map_err(|e| ShaclError::FailedIoOperation {
                error: e.to_string(),
            })?;
        },
        _ => {
            let rdf_format = shacl_format.try_into()?;
            let mut shacl_writer = ShaclWriter::new();

            shacl_writer.write(shacl_shapes).map_err(|e| ShaclError::FailedIoOperation {
                error: e.to_string(),
            })?;

            shacl_writer.serialize(&rdf_format, writer).map_err(|e| ShaclError::FailedIoOperation {
                error: e.to_string(),
            })?;
        }
    }

    Ok(())
}


