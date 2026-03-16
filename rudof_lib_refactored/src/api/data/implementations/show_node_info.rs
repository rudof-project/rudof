use crate::{Result, Rudof, formats::NodeInspectionMode};
use std::io;

pub fn show_node_info<W: io::Write>(
    rudof: &Rudof,
    node: &str,
    predicates: Option<&[String]>,
    show_node_mode: Option<&NodeInspectionMode>,
    depth: Option<usize>,
    show_hyperlinks: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
