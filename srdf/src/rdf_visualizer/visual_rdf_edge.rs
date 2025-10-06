use crate::iri::Iri;
use crate::rdf_visualizer::REIFIES;
use crate::{Rdf, rdf_visualizer::visual_rdf_graph::EdgeId};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFEdge {
    Iri { label: String, url: String },
    Reifies,
}

impl VisualRDFEdge {
    pub fn from_iri<R: Rdf>(rdf: &R, iri: &R::IRI) -> Self {
        if iri.as_str() == REIFIES {
            return VisualRDFEdge::Reifies;
        }
        let iri_label = R::qualify_iri(rdf, iri);
        let iri_str = (*iri).as_str().to_string();
        VisualRDFEdge::Iri {
            label: iri_label,
            url: iri_str,
        }
    }

    pub fn as_plantuml_link(&self) -> String {
        match self {
            VisualRDFEdge::Iri { label, url } => format!("[[{url} {label}]]"),
            VisualRDFEdge::Reifies => format!("[[{} {}]]", REIFIES, "reifies"),
        }
    }

    pub fn as_plantuml(&self, _edge_id: EdgeId) -> String {
        " ".to_string()
    }

    pub fn label(&self) -> String {
        match self {
            VisualRDFEdge::Iri { label, .. } => label.clone(),
            VisualRDFEdge::Reifies => "reifies".to_string(),
        }
    }
}

impl Display for VisualRDFEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualRDFEdge::Iri { label, url } => write!(f, "{label} ({url})"),
            VisualRDFEdge::Reifies => write!(f, "reifies"),
        }
    }
}

/*fn convert_to_visual_edge<R: Rdf>(rdf: &R, iri: &R::IRI) -> VisualRDFEdge {
    let iri_label = R::qualify_iri(&rdf, iri);
    let iri_str = (*iri).as_str().to_string();
    VisualRDFEdge::Iri {
        label: iri_label,
        url: iri_str,
    }
}*/
