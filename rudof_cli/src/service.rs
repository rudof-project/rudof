use std::path::PathBuf;

use crate::data::data_format2rdf_format;
use crate::mime_type::MimeType;
use crate::writer::get_writer;
use crate::{data_format::DataFormat, InputSpec, RDFReaderMode, ResultServiceFormat};
use anyhow::Result;
use rudof_lib::RudofConfig;
use sparql_service::ServiceDescription;

pub fn run_service(
    input: &InputSpec,
    data_format: &DataFormat,
    reader_mode: &RDFReaderMode,
    output: &Option<PathBuf>,
    result_format: &ResultServiceFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let reader = input.open_read(Some(data_format.mime_type().as_str()), "Service")?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let rdf_format = data_format2rdf_format(data_format);
    let config = config.service_config();
    let base = config.base.as_ref().map(|i| i.as_str());
    let reader_mode = (*reader_mode).into();
    let service_description =
        ServiceDescription::from_reader(reader, &rdf_format, base, &reader_mode)?;
    match result_format {
        ResultServiceFormat::Internal => {
            writeln!(writer, "{service_description}")?;
        }
    }
    Ok(())
}
