use std::path::PathBuf;
use std::str::FromStr;

use iri_s::IriS;
use prefixmap::PrefixMap;
use rudof_lib::{Rudof, RudofConfig};
use srdf::RDFFormat;

use crate::anyhow::{bail, Result};
use crate::cli::MimeType;
use crate::writer::get_writer;
use crate::{
    cli::{DataFormat, RDFReaderMode},
    InputSpec,
};

pub fn get_data_rudof(
    rudof: &mut Rudof,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    match (data.is_empty(), endpoint) {
        (true, None) => {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
        (false, None) => {
            let rdf_format = data_format2rdf_format(data_format);
            let reader_mode = match &reader_mode {
                RDFReaderMode::Lax => srdf::ReaderMode::Lax,
                RDFReaderMode::Strict => srdf::ReaderMode::Strict,
            };
            for d in data {
                let data_reader = d.open_read(Some(&data_format.mime_type()), "RDF data")?;
                let base = get_base(d, config)?;
                rudof.read_data(data_reader, &rdf_format, base.as_deref(), &reader_mode)?;
            }
            Ok(())
        }
        (true, Some(endpoint)) => {
            let (endpoint_iri, prefixmap) =
                if let Some(endpoint_descr) = config.rdf_data_config().find_endpoint(endpoint) {
                    (
                        endpoint_descr.query_url().clone(),
                        endpoint_descr.prefixmap().clone(),
                    )
                } else {
                    let iri = IriS::from_str(endpoint.as_str())?;
                    (iri, PrefixMap::basic())
                };
            rudof.add_endpoint(&endpoint_iri, &prefixmap)?;
            Ok(())
        }
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time at this moment")
        }
    }
}

pub fn data_format2rdf_format(data_format: &DataFormat) -> RDFFormat {
    match data_format {
        DataFormat::N3 => RDFFormat::N3,
        DataFormat::NQuads => RDFFormat::NQuads,
        DataFormat::NTriples => RDFFormat::NTriples,
        DataFormat::RDFXML => RDFFormat::RDFXML,
        DataFormat::TriG => RDFFormat::TriG,
        DataFormat::Turtle => RDFFormat::Turtle,
    }
}

/*
fn parse_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    reader_mode: &RDFReaderMode,
    config: &RdfDataConfig,
) -> Result<SRDFGraph> {
    let mut graph = SRDFGraph::new();
    let rdf_format = data_format2rdf_format(data_format);
    for d in data {
        let reader = d.open_read(Some(data_format.mime_type().as_str()))?;
        let base = config.base.as_ref().map(|iri_s| iri_s.as_str());
        let reader_mode = reader_mode_convert(*reader_mode);
        graph.merge_from_reader(reader, &rdf_format, base, &reader_mode)?;
    }
    Ok(graph)
}*/

pub fn get_base(input: &InputSpec, config: &RudofConfig) -> Result<Option<String>> {
    let base = match config.rdf_data_base() {
        Some(base) => Some(base.to_string()),
        None => {
            if config.automatic_base() {
                let base = input.guess_base()?;
                Some(base)
            } else {
                None
            }
        }
    };
    Ok(base)
}

#[allow(clippy::too_many_arguments)]
pub fn run_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    debug: u8,
    output: &Option<PathBuf>,
    result_format: &DataFormat,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    if debug > 0 {
        println!("Config: {config:?}")
    }
    get_data_rudof(&mut rudof, data, data_format, &None, reader_mode, config)?;
    let format: RDFFormat = RDFFormat::from(*result_format);
    rudof.get_rdf_data().serialize(&format, &mut writer)?;
    Ok(())
}
