use crate::{PrefixMap, PrefixMapError};
use std::fmt::Formatter;

pub type QResult = std::result::Result<(), PrefixMapError>;

pub trait QDisplay {
    fn show_qualified(&self, shapemap: &PrefixMap, f: &mut Formatter<'_>) -> QResult;
}
