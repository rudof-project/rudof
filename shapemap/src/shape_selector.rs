use prefixmap::IriRef;
use serde::Serialize;
use shex_ast::ShapeExprLabel;

/// A ShapeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select shape expressions to validate
///
#[derive(Debug, PartialEq, Clone, Serialize)]
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

    pub fn iter_shape(&self) -> impl Iterator<Item = &ShapeExprLabel> {
        match self {
            ShapeSelector::Label(label) => std::iter::once(label),
            ShapeSelector::Start => std::iter::once(&ShapeExprLabel::Start),
        }
    }
}
