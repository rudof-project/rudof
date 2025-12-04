//! Common models for schemas and validation
//!
//!
pub mod coremo;
pub mod result_association;

use iri_s::IriS;

pub use crate::coremo::*;
pub use crate::result_association::*;

type ShapeId = usize;
type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShapeLabel {
    Str(String),
    IriS(IriS),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeLabel {
    Str(String),
    IriS(IriS),
}
