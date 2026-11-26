use shacl::rdf::ShaclWriter;

use crate::{Result, Rudof, api::data::write_pretty_json, errors::ShaclError, formats::ShaclFormat};
use rudof_rdf::{rdf_core::RDFFormat, rdf_impl::OxigraphInMemory};
use std::io;

pub fn serialize_shacl_schema<W: io::Write>(
    rudof: &Rudof,
    shacl_format: Option<&ShaclFormat>,
    writer: &mut W,
) -> Result<()> {
    let shacl_format = shacl_format.copied().unwrap_or_default();

    let shacl_shapes = rudof.shacl_shapes.as_ref().ok_or(ShaclError::NoShaclShapesLoaded)?;

    match shacl_format {
        ShaclFormat::Internal => {
            write!(writer, "{shacl_shapes}").map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
        },
        _ => {
            let rdf_format: RDFFormat = shacl_format.try_into()?;
            let mut shacl_writer: ShaclWriter<OxigraphInMemory> = ShaclWriter::new();

            shacl_writer
                .register(shacl_shapes)
                .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;

            if rdf_format == RDFFormat::JsonLd && rudof.config.rdf_data().pretty_json() {
                let mut buf = Vec::new();
                shacl_writer
                    .serialize(&rdf_format, &mut buf)
                    .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
                write_pretty_json(writer, &buf, rudof.config.rdf_data().colorize_json())
                    .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
            } else {
                shacl_writer
                    .serialize(&rdf_format, writer)
                    .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
            }
        },
    }

    Ok(())
}
