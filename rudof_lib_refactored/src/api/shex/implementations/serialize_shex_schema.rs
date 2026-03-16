use crate::{Rudof, Result};
use std::io;

pub fn serialize_shex_schema<W: io::Write>(
    rudof: &Rudof,
    shape_label: Option<&str>,
    show_schema: Option<bool>,
    show_statistics: Option<bool>,
    show_dependencies: Option<bool>,
    show_time: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
