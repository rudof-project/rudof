use crate::{
    Result, Rudof,
    errors::DataError,
    formats::{DataFormat, DataReaderMode, InputSpec},
    types::Data,
    utils::get_base_iri,
};
use iri_s::{IriS, MimeType};
use pgschema::parser::pg_builder::PgBuilder;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::SparqlEndpoint;
use sparql_service::RdfData;
use std::{io, str::FromStr};

pub fn load_data(
    rudof: &mut Rudof,
    data: Option<&[InputSpec]>,
    data_format: Option<&DataFormat>,
    base: Option<&str>,
    endpoint: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    merge: Option<bool>,
) -> Result<()> {
    let (data_format, reader_mode, base, merge) = init_defaults(rudof, data_format, reader_mode, base, merge)?;

    match (data, endpoint) {
        (Some(_data), Some(_endpoint)) => Err(Box::new(DataError::DataSourceSpec {
            message: "Cannot specify both data and endpoint. Please choose one or the other.".to_string(),
        })
        .into()),
        (Some(data), None) => match data_format {
            DataFormat::Pg => load_data_from_specs_pg(rudof, data, merge),
            _ => load_data_from_specs_rdf(rudof, data, data_format, base, reader_mode, merge),
        },
        (None, Some(endpoint)) => load_data_from_endpoint(rudof, endpoint),
        (None, None) => Err(Box::new(DataError::DataSourceSpec {
            message: "No data source specified. Please provide either data or an endpoint.".to_string(),
        })
        .into()),
    }
}

fn init_defaults(
    rudof: &mut Rudof,
    data_format: Option<&DataFormat>,
    reader_mode: Option<&DataReaderMode>,
    base: Option<&str>,
    merge: Option<bool>,
) -> Result<(DataFormat, DataReaderMode, IriS, bool)> {
    let base = get_base_iri(rudof, base)?;
    Ok((
        data_format.copied().unwrap_or_default(),
        reader_mode.copied().unwrap_or_default(),
        base,
        merge.unwrap_or(true),
    ))
}

fn load_data_from_specs_pg(rudof: &mut Rudof, data: &[InputSpec], merge: bool) -> Result<()> {
    for input_spec in data {
        let mut data_reader = input_spec
            .open_read(Some(DataFormat::Pg.mime_type()), "PG data")
            .map_err(|error| {
                Box::new(DataError::DataSourceSpec {
                    message: format!("Failed to open data source '{}': {error}", input_spec.source_name()),
                })
            })?;

        read_pg_data(rudof, &mut data_reader, input_spec.source_name().as_str(), merge)?;
    }

    Ok(())
}

fn load_data_from_specs_rdf(
    rudof: &mut Rudof,
    data: &[InputSpec],
    data_format: DataFormat,
    base: IriS,
    reader_mode: DataReaderMode,
    merge: bool,
) -> Result<()> {
    for input_spec in data {
        let mut data_reader = input_spec
            .open_read(Some(data_format.mime_type()), "RDF data")
            .map_err(|error| {
                Box::new(DataError::DataSourceSpec {
                    message: format!("Failed to open data source '{}': {error}", input_spec.source_name()),
                })
            })?;

        read_rdf_data(
            rudof,
            &mut data_reader,
            input_spec.source_name().as_str(),
            &data_format,
            &base,
            &reader_mode,
            merge,
        )?;
    }

    Ok(())
}

fn read_rdf_data<R: io::Read>(
    rudof: &mut Rudof,
    data_reader: &mut R,
    source_name: &str,
    data_format: &DataFormat,
    base: &IriS,
    reader_mode: &DataReaderMode,
    merge: bool,
) -> Result<()> {
    if !merge || rudof.data.is_none() || matches!(rudof.data, Some(ref data) if data.is_pg()) {
        let rdf_data = init_rdf_data_with_config(rudof)?;
        rudof.data = Some(rdf_data);
    }

    rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .merge_from_reader(
            data_reader,
            source_name,
            &(*data_format).try_into()?,
            Some(base.as_str()),
            &(*reader_mode).into(),
        )
        .map_err(|error| {
            Box::new(DataError::FailedParsingRdfData {
                source_name: source_name.to_string(),
                format: data_format.to_string(),
                base: base.to_string(),
                reader_mode: reader_mode.to_string(),
                error: error.to_string(),
            })
        })?;

    Ok(())
}

fn read_pg_data<R: io::Read>(rudof: &mut Rudof, data_reader: &mut R, source_name: &str, merge: bool) -> Result<()> {
    if !merge || rudof.data.is_none() || matches!(rudof.data, Some(ref data) if data.is_rdf()) {
        rudof.data = Some(Data::empty_pg());
    }

    let mut data_content = String::new();

    data_reader.read_to_string(&mut data_content).map_err(|error| {
        Box::new(DataError::DataSourceSpec {
            message: format!("Failed to read data from '{}': {error}", source_name),
        })
    })?;

    let pg = match PgBuilder::new().parse_pg(data_content.as_str()) {
        Ok(pg) => pg,
        Err(error) => {
            return Err(Box::new(DataError::FailedParsingPgData {
                source_name: source_name.to_string(),
                error: error.to_string(),
            }))?;
        },
    };

    rudof.data.as_mut().unwrap().unwrap_pg_mut().merge(&pg);

    Ok(())
}

fn load_data_from_endpoint(rudof: &mut Rudof, endpoint_str: &str) -> Result<()> {
    let rdf_data = init_rdf_data_with_config(rudof)?;
    rudof.data = Some(rdf_data);

    let enpoint = get_endpoint_name(rudof, endpoint_str)?;

    use_endpoint(rudof, endpoint_str, enpoint);

    Ok(())
}

fn get_endpoint_name(rudof: &mut Rudof, endpoint_str: &str) -> Result<SparqlEndpoint> {
    let rdf_data = rudof.data.as_mut().unwrap().unwrap_rdf_mut();

    match rdf_data.find_endpoint(endpoint_str) {
        Some(endpoint) => Ok(endpoint),
        None => {
            let normalized_endpoint = normalize_endpoint_id(endpoint_str);
            if let Some(endpoint) = rdf_data.endpoints().iter().find_map(|(name, endpoint)| {
                let endpoint_iri = endpoint.iri().as_str();
                let matches_name = normalize_endpoint_id(name) == normalized_endpoint;
                let matches_iri = normalize_endpoint_id(endpoint_iri) == normalized_endpoint;
                if matches_name || matches_iri {
                    Some(endpoint.clone())
                } else {
                    None
                }
            }) {
                return Ok(endpoint);
            }

            let iri = IriS::from_str(endpoint_str).map_err(|error| {
                Box::new(DataError::InvalidEndpoint {
                    endpoint: (endpoint_str.to_string()),
                    error: (error.to_string()),
                })
            })?;

            let endpoint = SparqlEndpoint::new(&iri, &PrefixMap::new()).map_err(|error| {
                Box::new(DataError::InvalidEndpoint {
                    endpoint: endpoint_str.to_string(),
                    error: error.to_string(),
                })
            })?;

            Ok(endpoint)
        },
    }
}

fn use_endpoint(rudof: &mut Rudof, endpoint_str: &str, endpoint: SparqlEndpoint) {
    rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .use_endpoint(endpoint_str, endpoint);
}

fn init_rdf_data_with_config(rudof: &Rudof) -> Result<Data> {
    let rdf_data = RdfData::new()
        .with_rdf_data_config(&rudof.config.rdf_data_config())
        .map_err(|error| {
            Box::new(DataError::RdfDataConfig {
                error: error.to_string(),
            })
        })?;

    Ok(Data::RDFData(Box::new(rdf_data)))
}

fn normalize_endpoint_id(value: &str) -> String {
    let trimmed = value.trim().trim_end_matches('/');
    let without_scheme = trimmed
        .strip_prefix("http://")
        .or_else(|| trimmed.strip_prefix("https://"))
        .unwrap_or(trimmed);

    without_scheme.to_ascii_lowercase()
}
