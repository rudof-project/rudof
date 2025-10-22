use std::path::PathBuf;

use crate::writer::get_writer;
use crate::{RDFReaderMode, ResultServiceFormat};
use anyhow::Result;
use iri_s::mime_type::MimeType;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, data::data_format2rdf_format, data_format::DataFormat,
};
use sparql_service::ServiceDescriptionFormat;

pub fn run_service(
    input: &InputSpec,
    data_format: &DataFormat,
    reader_mode: &RDFReaderMode,
    output: &Option<PathBuf>,
    result_format: &ResultServiceFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let reader = input.open_read(Some(data_format.mime_type()), "Service")?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let rdf_format = data_format2rdf_format(data_format);
    let service_config = config.service_config();
    let base = service_config.base.as_ref().map(|i| i.as_str());
    let mut rudof = Rudof::new(config)?;
    let reader_mode = reader_mode.into();

    rudof.read_service_description(reader, &rdf_format, base, &reader_mode)?;
    match result_format {
        ResultServiceFormat::Internal => {
            rudof
                .serialize_service_description(&ServiceDescriptionFormat::Internal, &mut writer)?;
        }
        ResultServiceFormat::Mie => {
            rudof.serialize_service_description(&ServiceDescriptionFormat::Mie, &mut writer)?;
        }
        ResultServiceFormat::JSON => {
            let json = serde_json::to_string_pretty(&rudof.get_service_description())?;
            writer.write_all(json.as_bytes())?;
        }
    }
    Ok(())
}
