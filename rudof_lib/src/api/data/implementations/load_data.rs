use crate::errors::{InputSpecError, RudofError};
use crate::{
    Result, Rudof,
    errors::DataError,
    formats::{DataFormat, DataReaderMode, InputSpec},
    types::Data,
    utils::get_base_iri,
};
use pgschema::parser::pg_builder::PgBuilder;
use prefixmap::PrefixMap;
use regex::Regex;
use rudof_iri::{IriS, MimeType};
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::OxigraphEndpoint;
use sparql_service::RdfData;
use std::io::Read;
use std::{io, str::FromStr};

pub fn load_data(
    rudof: &mut Rudof,
    data: Option<&[InputSpec]>,
    data_format: Option<&DataFormat>,
    base: Option<&str>,
    endpoint: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    merge: Option<bool>,
    prefixes: Option<&[InputSpec]>,
) -> Result<()> {
    let (data_format, reader_mode, base, merge) = init_defaults(rudof, data_format, reader_mode, base, merge)?;
    match (data, endpoint) {
        (Some(_data), Some(_endpoint)) => Err(Box::new(DataError::DataSourceSpec {
            message: "Cannot specify both data and endpoint. Please choose one or the other.".to_string(),
        })
        .into()),
        (Some(data), None) => match data_format {
            DataFormat::Pg => load_data_from_specs_pg(rudof, data, merge),
            _ => load_data_from_specs_rdf(rudof, data, data_format, base, reader_mode, merge, prefixes),
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
    prefixes: Option<&[InputSpec]>,
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

    if let Some(prefixes) = prefixes {
        let mut pm = PrefixMap::new();

        for prefix in prefixes {
            pm.merge(load_user_prefixes(prefix)?)
        }

        rudof.data.as_mut().unwrap().unwrap_rdf_mut().merge_prefixes(pm);
    }
    Ok(())
}

fn load_user_prefixes(input_spec: &InputSpec) -> Result<PrefixMap> {
    if matches!(input_spec, InputSpec::Stdin) {
        return Err(RudofError::InputSpec(InputSpecError::InvalidInput {
            error: "Stdin is not supported".to_string(),
        }));
    }
    let mut pm = PrefixMap::new();

    let mut reader = input_spec.open_read(Some("text/plain"), "Prefix error")?;
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .map_err(|e| RudofError::InputSpec(InputSpecError::InvalidInput { error: e.to_string() }))?;

    let content = String::from_utf8(buffer)
        .map_err(|e| RudofError::InputSpec(InputSpecError::InvalidInput { error: e.to_string() }))?;

    // Maybe replace with https://www.w3.org/TR/turtle/#grammar-production-PN_PREFIX
    let prefix_regex = Regex::new(r"^(.*) *: *<(.*)>$").unwrap();

    let mut add_prefix = |content: &str| {
        if let Some(captures) = prefix_regex.captures(content) {
            let prefix = captures.get(1).unwrap().as_str().trim();
            let iri = captures.get(2).unwrap().as_str().trim();
            pm.add_prefix(
                prefix,
                IriS::from_str(iri)
                    .map_err(|e| RudofError::InputSpec(InputSpecError::InvalidInput { error: e.to_string() }))?,
            );
            Ok(())
        } else {
            Err(RudofError::InputSpec(InputSpecError::InvalidInput {
                error: format!("Invalid prefix declaration: '{}'", content),
            }))
        }
    };

    match input_spec {
        InputSpec::Path(_) | InputSpec::Url(_) => {
            let lines = content.split("\n");
            for line in lines {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                add_prefix(line)?
            }
        },
        InputSpec::Stdin => unreachable!(),
        InputSpec::Str(_) => {
            let content = content.trim();
            add_prefix(content)?
        },
    }

    Ok(pm)
}

fn read_rdf_data<R: Read>(
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

fn get_endpoint_name(rudof: &mut Rudof, endpoint_str: &str) -> Result<OxigraphEndpoint> {
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

            let endpoint = OxigraphEndpoint::new(&iri, &PrefixMap::new()).map_err(|error| {
                Box::new(DataError::InvalidEndpoint {
                    endpoint: endpoint_str.to_string(),
                    error: error.to_string(),
                })
            })?;

            Ok(endpoint)
        },
    }
}

fn use_endpoint(rudof: &mut Rudof, endpoint_str: &str, endpoint: OxigraphEndpoint) {
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

/// Load data into the QLever Docker backend instead of the in-memory graph.
///
/// Available with `--features qlever`.
#[cfg(feature = "qlever")]
pub fn load_data_via_qlever(
    rudof: &mut Rudof,
    data: Option<&[InputSpec]>,
    data_format: Option<&DataFormat>,
    base: Option<&str>,
    prefixes: Option<&[InputSpec]>,
) -> Result<()> {
    use crate::types::Data;
    use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{QleverGraphContainer, qlever_runtime};
    use sparql_service::RdfData;

    // Pg is the only non-RDF DataFormat; reject it before we try anything.
    if matches!(data_format, Some(DataFormat::Pg)) {
        return Err(Box::new(DataError::NonRdfFormat {
            format: "pg".to_string(),
        })
        .into());
    }

    let inputs = data.ok_or_else(|| {
        Box::new(DataError::DataSourceSpec {
            message: "--backend qlever requires at least one --data input".to_string(),
        })
    })?;
    if inputs.is_empty() {
        return Err(Box::new(DataError::DataSourceSpec {
            message: "--backend qlever requires at least one --data input".to_string(),
        })
        .into());
    }

    // Collect every file-system path; reject other InputSpec variants so the
    // caller gets a clear error instead of a silently-partial index.
    let mut paths: Vec<std::path::PathBuf> = Vec::with_capacity(inputs.len());
    for spec in inputs {
        match spec {
            InputSpec::Path(p) => paths.push(p.clone()),
            other => {
                let kind = match other {
                    InputSpec::Stdin => "stdin",
                    InputSpec::Str(_) => "string",
                    InputSpec::Url(_) => "URL",
                    InputSpec::Path(_) => unreachable!(),
                };
                return Err(Box::new(DataError::DataSourceSpec {
                    message: format!(
                        "--backend qlever only supports file-system inputs; got {kind} for '{}'",
                        other.source_name()
                    ),
                })
                .into());
            },
        }
    }

    let rdf_format: Option<RDFFormat> = match data_format {
        Some(f) => Some(RDFFormat::try_from(f).map_err(|e| {
            Box::new(DataError::DataSourceSpec {
                message: format!("could not map --data-format to an RDF format: {e}"),
            })
        })?),
        None => None,
    };

    // Honour any `[qlever]` section in the loaded config.
    let qlever_config = rudof.config.rdf_data_config().qlever.clone().unwrap_or_default();

    // Drive the async loader on the process-wide QLever runtime so the
    // reactor outlives the resulting `QleverServer`. If we created a local
    // `Runtime` here it would be dropped at end of scope, but the
    // `QleverServer` (stored in `rudof.data`) outlives this function and its
    // `Drop` impl needs a live reactor to talk to the Docker socket.
    let graph = qlever_runtime()
        .block_on(QleverGraphContainer::from_paths(
            &paths,
            rdf_format.as_ref(),
            qlever_config,
        ))
        .map_err(|e| {
            Box::new(DataError::DataSourceSpec {
                message: format!("QLever backend failed: {e}"),
            })
        })?;

    let mut rdf_data = RdfData::new()
        .with_rdf_data_config(&rudof.config.rdf_data_config())
        .map_err(|error| {
            Box::new(DataError::RdfDataConfig {
                error: error.to_string(),
            })
        })?;
    rdf_data.set_qlever(graph);

    if let Some(base) = base {
        let base_iri = IriS::from_str(base).map_err(|e| {
            Box::new(DataError::DataSourceSpec {
                message: format!("invalid --base IRI '{base}': {e}"),
            })
        })?;
        rdf_data.add_base(&Some(base_iri));
    }
    if let Some(prefixes) = prefixes {
        let mut pm = PrefixMap::new();
        for prefix in prefixes {
            pm.merge(load_user_prefixes(prefix)?);
        }
        rdf_data.merge_prefixes(pm);
    }

    rudof.data = Some(Data::RDFData(Box::new(rdf_data)));
    Ok(())
}

fn normalize_endpoint_id(value: &str) -> String {
    let trimmed = value.trim().trim_end_matches('/');
    let without_scheme = trimmed
        .strip_prefix("http://")
        .or_else(|| trimmed.strip_prefix("https://"))
        .unwrap_or(trimmed);

    without_scheme.to_ascii_lowercase()
}
