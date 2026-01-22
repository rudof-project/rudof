use crate::{PrefixMap, PrefixMapError};
use std::fmt::Formatter;

pub type QResult = Result<(), PrefixMapError>;

pub trait QDisplay {
    fn show_qualified(&self, shapemap: &PrefixMap, f: &mut Formatter<'_>) -> QResult;
}
