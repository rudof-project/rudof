use crate::ShapeExprLabel;
use iri_s::IriS;
use prefixmap::{DerefIri, IriRef};
use serde::Serialize;
use std::fmt::Display;

/// A ShapeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select shape expressions to validate
///
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ShapeSelector {
    Label(ShapeExprLabel),
    Start,
}

impl ShapeSelector {
    pub fn label(label: ShapeExprLabel) -> ShapeSelector {
        ShapeSelector::Label(label)
    }

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

impl Display for ShapeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeSelector::Label(label) => write!(f, "{label}"),
            ShapeSelector::Start => write!(f, "START"),
        }
    }
}

impl DerefIri for ShapeSelector {
    fn deref_iri(
        self,
        base: Option<&IriS>,
        prefixmap: Option<&prefixmap::PrefixMap>,
    ) -> Result<Self, prefixmap::DerefError>
    where
        Self: Sized,
    {
        match self {
            ShapeSelector::Label(shape_expr_label) => {
                let resolved = shape_expr_label.deref_iri(base, prefixmap)?;
                Ok(ShapeSelector::Label(resolved))
            },
            ShapeSelector::Start => Ok(ShapeSelector::Start),
        }
    }
}
