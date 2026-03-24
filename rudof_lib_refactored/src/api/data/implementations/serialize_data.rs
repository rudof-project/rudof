use crate::{Result, Rudof, errors::DataError, formats::ResultDataFormat};
use rudof_rdf::rdf_core::visualizer::{
    VisualRDFGraph,
    uml_converter::{UmlConverter, UmlGenerationMode},
};
use std::io;

pub fn serialize_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: Option<&ResultDataFormat>,
    writer: &mut W,
) -> Result<()> {
    let result_data_format = result_data_format.copied().unwrap_or_default();

    let data = rudof.data.as_ref().ok_or(DataError::NoDataLoaded)?;

    if data.is_rdf() {
        serialize_rdf_data(rudof, result_data_format, writer)
    } else {
         serialize_pg_data(rudof, result_data_format, writer)
    }
}

fn serialize_pg_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: ResultDataFormat,
    writer: &mut W,
) -> Result<()> {
    let data = rudof.data.as_mut().ok_or(DataError::NoDataLoaded)?;

    if !data.is_pg() {
        Err(DataError::NoPgDataLoaded)?
    }

    let graph = data.unwrap_pg_mut();

    write!(writer, "{graph}").map_err(|e| DataError::FailedSerializingData {
        format: result_data_format.to_string(),
        error: e.to_string(),
    })?;

    Ok(())
}

fn serialize_rdf_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: ResultDataFormat,
    writer: &mut W,
) -> Result<()> {
    let data = rudof.data.as_mut().ok_or(DataError::NoRdfDataLoaded)?;

    if !data.is_rdf() {
        Err(DataError::NoRdfDataLoaded)?
    }

    if result_data_format.is_rdf_format() {
        data.unwrap_rdf_mut()
            .serialize(&result_data_format.try_into()?, writer)
            .map_err(|e| DataError::FailedSerializingData {
                format: result_data_format.to_string(),
                error: e.to_string(),
            })?;
    } else {
        let visualization_config = rudof.config.rdf_data_config().rdf_visualization_config();
        let converter = VisualRDFGraph::from_rdf(data.unwrap_rdf_mut(), visualization_config).map_err(|e| {
            DataError::FailedSerializingData {
                format: result_data_format.to_string(),
                error: e.to_string(),
            }
        })?;

        if result_data_format.is_image_visualization_format() {
            converter
                .as_image(
                    writer,
                    result_data_format.try_into()?,
                    &UmlGenerationMode::all(),
                    rudof.config.plantuml_path(),
                )
                .map_err(|e| DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })?;
        } else {
            converter
                .as_plantuml(writer, &UmlGenerationMode::AllNodes)
                .map_err(|e| DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })?;
        }
    }

    Ok(())
}
