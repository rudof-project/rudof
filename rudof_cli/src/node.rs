extern crate anyhow;
use anyhow::*;
use std::path::PathBuf;

use rudof_lib::{Rudof, RudofConfig};
use rudof_lib::node_info::{get_node_info, NodeInfoOptions};

use crate::data_format::DataFormat;
use crate::{
    RDFReaderMode, ShowNodeMode, 
    data::get_data_rudof,
    input_spec::InputSpec,
    node_selector::parse_node_selector,
    writer::get_writer,
};
use crate::node_formatter::format_node_info_list;

// Run the node info command
#[allow(clippy::too_many_arguments)]
pub fn run_node(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    node_str: &str,
    predicates: &Vec<String>,
    show_node_mode: &ShowNodeMode,
    _show_hyperlinks: &bool,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
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
    
    let predicates_slice: Vec<String> = predicates.clone();
    let node_infos = get_node_info(
        data,
        node_selector,
        &predicates_slice,
        options,
    )?;
    
    format_node_info_list(&node_infos, data, &mut writer)?;
    Ok(())
}