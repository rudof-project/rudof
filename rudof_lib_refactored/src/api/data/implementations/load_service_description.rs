use crate::{
    Result, Rudof,
    errors::DataError,
    formats::{DataFormat, DataReaderMode, InputSpec},
    utils::get_base_iri,
};
use iri_s::{IriS, MimeType};
pub use sparql_service::ServiceDescription;

pub fn load_service_description(
    rudof: &mut Rudof,
    service: &InputSpec,
    data_format: Option<&DataFormat>,
    reader_mode: Option<&DataReaderMode>,
    base: Option<&str>,
) -> Result<()> {
    let (data_format, reader_mode, base) = init_defaults(rudof, data_format, reader_mode, base)?;

    let mut data_reader = service
        .open_read(Some(data_format.mime_type()), "Service description")
        .map_err(|error| DataError::DataSourceSpec {
            message: format!("Failed to open data source '{}': {error}", service.source_name()),
        })?;

    let service_description = ServiceDescription::from_reader(
        &mut data_reader,
        &service.source_name(),
        &data_format.try_into()?,
        Some(base.as_str()),
        &reader_mode.into(),
    )
    .map_err(|error| DataError::FailedParsingServiceDescriptionData {
        source_name: service.source_name().to_string(),
        format: data_format.to_string(),
        base: base.to_string(),
        reader_mode: reader_mode.to_string(),
        error: error.to_string(),
    })?;

    rudof.service_description = Some(service_description);

    Ok(())
}

fn init_defaults(
    rudof: &mut Rudof,
    data_format: Option<&DataFormat>,
    reader_mode: Option<&DataReaderMode>,
    base: Option<&str>,
) -> Result<(DataFormat, DataReaderMode, IriS)> {
    let base = get_base_iri(rudof, base)?;
    Ok((
        data_format.copied().unwrap_or_default(),
        reader_mode.copied().unwrap_or_default(),
        base,
    ))
}
