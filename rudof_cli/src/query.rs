use crate::{QueryType, ResultQueryFormat as CliResultQueryFormat, writer::get_writer};
use anyhow::{Result, bail};
use iri_s::IriS;
use rudof_lib::{
    InputSpec, RdfData, Rudof, RudofConfig, data_format::DataFormat, data_utils::get_data_rudof,
};
use srdf::{QueryResultFormat, QuerySolutions, ReaderMode};
use std::{io::Write, path::PathBuf};
use tracing::trace;

#[allow(clippy::too_many_arguments)]
pub fn run_query(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    query: &InputSpec,
    query_type: &QueryType,
    result_query_format: &CliResultQueryFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    _debug: u8,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        base,
        endpoint,
        reader_mode,
        config,
        false,
    )?;
    // rudof.serialize_data(&srdf::RDFFormat::Turtle, &mut writer)?;
    let mut reader = query.open_read(None, "Query")?;
    match query_type {
        QueryType::Select => {
            trace!("Running SELECT query");
            let results = rudof.run_query_select(&mut reader)?;
            show_results(&mut writer, &results, result_query_format)?;
        }
        QueryType::Construct => {
            let query_format = cnv_query_format(result_query_format);
            let str = rudof.run_query_construct(&mut reader, &query_format)?;
            writeln!(writer, "{str}")?;
        }
        QueryType::Ask => {
            // let bool = rudof.run_query_ask(&mut reader)?;
            // writeln!(writer, "{bool}")?;
            bail!("Not yet implemented ASK queries");
        }
        QueryType::Describe => {
            bail!("Not yet implemented DESCRIBE queries");
        }
    }
    Ok(())
}

fn show_results(
    writer: &mut dyn Write,
    results: &QuerySolutions<RdfData>,
    result_query_format: &CliResultQueryFormat,
) -> Result<()> {
    match result_query_format {
        CliResultQueryFormat::Internal => {
            results.write_table(writer)?;
        }
        _ => {
            todo!()
        }
    }
    Ok(())
}

/*fn show_variables<'a, W: Write>(
    writer: &mut W,
    vars: impl Iterator<Item = &'a VarName>,
) -> Result<()> {
    for var in vars {
        let str = format!("{var}");
        write!(writer, "{str:15}")?;
    }
    writeln!(writer)?;
    Ok(())
}*/

/*fn show_result<W: Write>(
    writer: &mut W,
    result: &QuerySolution<RdfData>,
    prefixmap: &PrefixMap,
) -> Result<()> {
    for (idx, _variable) in result.variables().enumerate() {
        let str = match result.find_solution(idx) {
            Some(term) => match term {
                oxrdf::Term::NamedNode(named_node) => {
                    let (str, _length) =
                        prefixmap.qualify_and_length(&IriS::from_named_node(named_node));
                    format!("{str}     ")
                }
                oxrdf::Term::BlankNode(blank_node) => format!("  {blank_node}"),
                oxrdf::Term::Literal(literal) => format!("  {literal}"),
                oxrdf::Term::Triple(triple) => format!("  {triple}"),
            },
            None => String::new(),
        };
        write!(writer, "{str:15}")?;
    }
    writeln!(writer)?;
    Ok(())
}*/

fn cnv_query_format(format: &CliResultQueryFormat) -> QueryResultFormat {
    match format {
        CliResultQueryFormat::Internal => QueryResultFormat::Turtle,
        CliResultQueryFormat::NTriples => QueryResultFormat::NTriples,
        CliResultQueryFormat::JsonLd => QueryResultFormat::JsonLd,
        CliResultQueryFormat::RdfXml => QueryResultFormat::RdfXml,
        CliResultQueryFormat::Csv => QueryResultFormat::Csv,
        CliResultQueryFormat::TriG => QueryResultFormat::TriG,
        CliResultQueryFormat::N3 => QueryResultFormat::N3,
        CliResultQueryFormat::NQuads => QueryResultFormat::NQuads,
        CliResultQueryFormat::Turtle => QueryResultFormat::Turtle,
    }
}
