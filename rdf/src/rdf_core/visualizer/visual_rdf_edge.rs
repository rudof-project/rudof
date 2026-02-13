
use crate::rdf_core::{
    Rdf, term::Iri, vocab::rdf_reifies
};
use std::fmt::Display;

/// Represents an edge in a visual RDF graph.
///
/// Edges connect nodes and can represent RDF predicates or special relationships
/// like reification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualRDFEdge {
    /// An edge representing an IRI predicate with a label and URL.
    Iri {
        /// The display label for the IRI
        label: String,
        /// The full IRI URL
        url: String
    },
    /// A special edge representing reification (rdf:reifies).
    Reifies,
}

impl VisualRDFEdge {
    /// Creates a `VisualRDFEdge` from an RDF IRI.
    ///
    /// If the IRI is the special `rdf:reifies` predicate, returns `Reifies`.
    /// Otherwise, creates an `Iri` variant with qualified label and URL.
    ///
    /// # Arguments
    /// * `rdf` - The RDF context for IRI qualification
    /// * `iri` - The IRI to convert
    ///
    /// # Returns
    /// * `VisualRDFEdge` - The corresponding visual edge
    pub fn from_iri<R: Rdf>(rdf: &R, iri: &R::IRI) -> Self {
        if iri.as_str() == rdf_reifies().as_str() {
            return VisualRDFEdge::Reifies;
        }
        let iri_label = R::qualify_iri(rdf, iri);
        let iri_str = (*iri).as_str().to_string();
        VisualRDFEdge::Iri {
            label: iri_label,
            url: iri_str,
        }
    }

    /// Converts the edge to a PlantUML link format.
    ///
    /// # Returns
    /// * `String` - The PlantUML link representation
    pub fn as_plantuml_link(&self) -> String {
        match self {
            VisualRDFEdge::Iri { label, url } => format!("[[{url} {label}]]"),
            VisualRDFEdge::Reifies => format!("[[{} {}]]", rdf_reifies().as_str(), "reifies"),
        }
    }

    /// Gets the display label for the edge.
    ///
    /// # Returns
    /// * `String` - The label string
    pub fn label(&self) -> String {
        match self {
            VisualRDFEdge::Iri { label, .. } => label.clone(),
            VisualRDFEdge::Reifies => "reifies".to_string(),
        }
    }
}

impl Display for VisualRDFEdge {
    /// Formats the edge for display purposes.
    ///
    /// Shows the label and URL for IRI edges, or just "reifies" for reification edges.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualRDFEdge::Iri { label, url } => write!(f, "{label} ({url})"),
            VisualRDFEdge::Reifies => write!(f, "reifies"),
        }
    }
}
