use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::ShapeExprLabel;
use shex_ast::{object_value::ObjectValue, Node};
use srdf::shacl_path::SHACLPath;
use srdf::SRDF;
use thiserror::Error;

/// A ShapeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select shape expressions to validate
///
#[derive(Debug, PartialEq)]
pub enum ShapeSelector {
    Label(ShapeExprLabel),
    Start,
}

impl ShapeSelector {
    pub fn iri_unchecked(str: &str) -> ShapeSelector {
        ShapeSelector::Label(ShapeExprLabel::iri_unchecked(str))
    }

    pub fn iri_ref(iri: IriRef) -> ShapeSelector {
        ShapeSelector::Label(ShapeExprLabel::iri_ref(iri))
    }

    pub fn start() -> ShapeSelector {
        ShapeSelector::Start
    }

    pub fn prefixed(alias: &str, local: &str) -> Self {
        ShapeSelector::Label(ShapeExprLabel::prefixed(alias, local))
    }
}
