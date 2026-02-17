use crate::writer::get_writer;
use anyhow::Result;
use iri_s::IriS;
use rdf::rdf_impl::ReaderMode;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, data::get_data_rudof, data_format::DataFormat, query::execute_query,
    query_result_format::ResultQueryFormat as CliResultQueryFormat, query_type::QueryType,
};
use std::path::PathBuf;

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

    execute_query(&mut rudof, query, query_type, result_query_format, &mut writer)?;
    Ok(())
}
