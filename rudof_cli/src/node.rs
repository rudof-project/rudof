use anyhow::*;
use iri_s::IriS;
use rudof_lib::node_info::{NodeInfoOptions, get_node_info, format_node_info_list, parse_node_selector};
use rudof_lib::{InputSpec, Rudof, RudofConfig, data::get_data_rudof, data_format::DataFormat};
use srdf::ReaderMode;
use std::path::PathBuf;

use crate::{ShowNodeMode, writer::get_writer};

#[allow(clippy::too_many_arguments)]
pub fn run_node(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    node_str: &str,
    predicates: &[String],
    show_node_mode: &ShowNodeMode,
    _show_hyperlinks: &bool,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
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
    let data = rudof.get_rdf_data();

    let node_selector = parse_node_selector(node_str)?;
    tracing::debug!("Node info with node selector: {node_selector:?}");

    let options = match show_node_mode {
        ShowNodeMode::Outgoing => NodeInfoOptions::outgoing(),
        ShowNodeMode::Incoming => NodeInfoOptions::incoming(),
        ShowNodeMode::Both => NodeInfoOptions::both(),
    };

    let node_infos = get_node_info(data, node_selector, predicates, &options)?;
    format_node_info_list(&node_infos, data, &mut writer, &options)?;
    Ok(())
}
