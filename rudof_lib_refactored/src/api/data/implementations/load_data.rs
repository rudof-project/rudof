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
use std::{io, str::FromStr};

/// Load data into the `Rudof` instance from a string, files or a SPARQL endpoint.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance
/// * `data` - Optional list of `InputSpec` sources
/// * `data_format` - Optional explicit data format (defaults applied)
/// * `base` - Optional base IRI used when parsing RDF content
/// * `endpoint` - Optional SPARQL endpoint URI; mutually exclusive with `data`
/// * `reader_mode` - Optional reader mode controlling parser behavior
/// * `merge` - If true, merge into existing store; otherwise replace it
///
/// # Errors
///
/// Returns an error when:
/// * Both `data` and `endpoint` are provided.
/// * Neither `data` nor `endpoint` is provided.
/// * Any lower-level open/parse operation fails.
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
        (Some(_data), Some(_endpoint)) => Err(DataError::DataSourceSpec {
            message: "Cannot specify both data and endpoint. Please choose one or the other.".to_string(),
        }
        .into()),
        (Some(data), None) => match data_format {
            DataFormat::Pg => load_data_from_specs_pg(rudof, data, merge),
            _ => load_data_from_specs_rdf(rudof, data, data_format, base, reader_mode, merge),
        },
        (None, Some(endpoint)) => load_data_from_endpoint(rudof, endpoint),
        (None, None) => Err(DataError::DataSourceSpec {
            message: "No data source specified. Please provide either data or an endpoint.".to_string(),
        }
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
        merge.unwrap_or(false),
    ))
}

/// Load Postgres-style (`Pg`) data from a list of `InputSpec` sources.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance
/// * `data` - Input specifications to read.
/// * `merge` - Whether to merge into existing PG store.
///
/// # Errors
///
/// Returns an error when opening a source fails or when parsing PG content
/// fails.
fn load_data_from_specs_pg(rudof: &mut Rudof, data: &[InputSpec], merge: bool) -> Result<()> {
    for input_spec in data {
        let mut data_reader = input_spec
            .open_read(Some(DataFormat::Pg.mime_type()), "PG data")
            .map_err(|error| DataError::DataSourceSpec {
                message: format!("Failed to open data source '{}': {error}", input_spec.source_name()),
            })?;

        read_pg_data(rudof, &mut data_reader, input_spec.source_name().as_str(), merge)?;
    }

    Ok(())
}

/// Load RDF data from a list of `InputSpec` sources using the specified
/// `data_format`, `base` IRI and `reader_mode`.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance
/// * `data` - Input specifications to read.
/// * `data_format` - Data format to use for parsing each source.
/// * `base` - Base IRI applied during parsing.
/// * `reader_mode` - Reader mode influencing parser behavior.
/// * `merge` - Whether to merge into existing RDF store.
///
/// # Errors
///
/// Propagates errors from `read_rdf_data` which are mapped to
/// `DataError::FailedParsingRdfData`.
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
            .map_err(|error| DataError::DataSourceSpec {
                message: format!("Failed to open data source '{}': {error}", input_spec.source_name()),
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

/// Read and parse RDF from a reader and merge into the RDF store.
/// 
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance
/// * `data_reader` - Reader to consume the RDF content from.
/// * `source_name` - Human-readable source name for error messages.
/// * `data_format` - Format to parse.
/// * `base` - Base IRI to resolve relative IRIs.
/// * `reader_mode` - Parser reader mode.
/// * `merge` - Whether to merge into existing store or replace it.
///
/// # Errors
///
/// Returns `DataError::FailedParsingRdfData` on parse/merge failures with
/// detailed context (source, format, base, reader mode, and original error).
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
        rudof.data = Some(Data::empty_rdf());
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
        .map_err(|error| DataError::FailedParsingRdfData {
            source_name: source_name.to_string(),
            format: data_format.to_string(),
            base: base.to_string(),
            reader_mode: reader_mode.to_string(),
            error: error.to_string(),
        })?;

    Ok(())
}

/// Read Pg data from the reader and merge into the PG store.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance
/// * `data_reader` - Reader containing PG formatted text.
/// * `source_name` - Name used for error messages.
/// * `merge` - Whether to merge into existing PG store.
///
/// # Errors
///
/// Returns `DataError::DataSourceSpec` if reading fails and
/// `DataError::FailedParsingPgData` if parsing fails.
fn read_pg_data<R: io::Read>(rudof: &mut Rudof, data_reader: &mut R, source_name: &str, merge: bool) -> Result<()> {
    if !merge || rudof.data.is_none() || matches!(rudof.data, Some(ref data) if data.is_rdf()) {
        rudof.data = Some(Data::empty_pg());
    }

    let mut data_content = String::new();

    data_reader
        .read_to_string(&mut data_content)
        .map_err(|error| DataError::DataSourceSpec {
            message: format!("Failed to read data from '{}': {error}", source_name),
        })?;

    let pg = match PgBuilder::new().parse_pg(data_content.as_str()) {
        Ok(pg) => pg,
        Err(error) => {
            return Err(DataError::FailedParsingPgData {
                source_name: source_name.to_string(),
                error: error.to_string(),
            })?;
        },
    };

    rudof.data.as_mut().unwrap().unwrap_pg_mut().merge(&pg);

    Ok(())
}

/// Configure `rudof` to use a SPARQL endpoint specified by `endpoint_str`.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance to modify.
/// * `endpoint_str` - Endpoint URI or name to register.
///
/// # Errors
///
/// Returns `DataError::InvalidEndpoint` if the provided endpoint string
/// cannot be parsed or the endpoint construction fails.
fn load_data_from_endpoint(rudof: &mut Rudof, endpoint_str: &str) -> Result<()> {
    rudof.data = Some(Data::empty_rdf());

    let enpoint = get_endpoint_name(rudof, endpoint_str)?;

    use_endpoint(rudof, endpoint_str, enpoint);

    Ok(())
}

/// Resolve or create a `SparqlEndpoint` for `endpoint_str`.
///
/// Attempts to find an already-registered endpoint in the RDF store. If
/// none is found, parses `endpoint_str` as an IRI and constructs a new
/// `SparqlEndpoint` using an empty `PrefixMap`.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance.
/// * `endpoint_str` - Endpoint URI to resolve or construct.
///
/// # Errors
///
/// Returns `DataError::InvalidEndpoint` if the string is not a valid IRI or
/// if constructing the `SparqlEndpoint` fails.
fn get_endpoint_name(rudof: &mut Rudof, endpoint_str: &str) -> Result<SparqlEndpoint> {
    match rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .find_endpoint(endpoint_str)
    {
        Some(endpoint) => Ok(endpoint),
        None => {
            let iri = IriS::from_str(endpoint_str).map_err(|error| DataError::InvalidEndpoint {
                endpoint: (endpoint_str.to_string()),
                error: (error.to_string()),
            })?;

            let endpoint =
                SparqlEndpoint::new(&iri, &PrefixMap::new()).map_err(|error| DataError::InvalidEndpoint {
                    endpoint: endpoint_str.to_string(),
                    error: error.to_string(),
                })?;

            Ok(endpoint)
        },
    }
}

/// Register an endpoint with the RDF store under `endpoint_str`.
///
/// # Arguments
///
/// * `rudof` - Mutable reference to the Rudof instance.
/// * `endpoint_str` - Key under which the endpoint will be registered.
/// * `endpoint` - The `SparqlEndpoint` instance to register.
fn use_endpoint(rudof: &mut Rudof, endpoint_str: &str, endpoint: SparqlEndpoint) {
    rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .add_endpoint(endpoint_str, endpoint);
}
