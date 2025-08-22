use std::{io::Write, path::PathBuf};

use iri_s::IriS;
use prefixmap::PrefixMap;
use rudof_lib::{RdfData, Rudof, RudofConfig};
use srdf::{QuerySolution, VarName};

use crate::{
    InputSpec, RDFReaderMode, ResultQueryFormat, data::get_data_rudof, data_format::DataFormat,
    writer::get_writer,
};
use anyhow::Result;

#[allow(clippy::too_many_arguments)]
pub fn run_query(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    query: &InputSpec,
    _result_query_format: &ResultQueryFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    _debug: u8,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;
    let mut reader = query.open_read(None, "Query")?;
    let results = rudof.run_query(&mut reader)?;
    let mut results_iter = results.iter().peekable();
    if let Some(first) = results_iter.peek() {
        show_variables(&mut writer, first.variables())?;
        for result in results_iter {
            show_result(&mut writer, result, &rudof.nodes_prefixmap())?
        }
    } else {
        write!(writer, "No results")?;
    }
    Ok(())
}

fn show_variables<'a, W: Write>(
    writer: &mut W,
    vars: impl Iterator<Item = &'a VarName>,
) -> Result<()> {
    for var in vars {
        let str = format!("{var}");
        write!(writer, "{str:15}")?;
    }
    writeln!(writer)?;
    Ok(())
}

fn show_result<W: Write>(
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
}
