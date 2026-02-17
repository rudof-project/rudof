use iri_s::IriS;
use serde::Serialize;
use std::fmt::Display;

/// Represents a SHACL property path for navigating RDF graphs.
///
/// SHACL paths follow the [SHACL property paths spec](https://www.w3.org/TR/shacl/#property-paths)
/// which are a subset of SPARQL property paths.
/// They enable complex navigation patterns through RDF graphs, extending simple predicate-based traversal with operations
/// like sequences, alternatives, inverses, and quantifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SHACLPath {
    /// A direct predicate path using a single IRI.
    Predicate { pred: IriS },
    /// An alternative path representing a union of multiple paths.
    Alternative { paths: Vec<SHACLPath> },
    /// A sequence path representing composed navigation.
    Sequence { paths: Vec<SHACLPath> },
    /// An inverse path representing reverse property navigation.
    Inverse { path: Box<SHACLPath> },
    /// A zero-or-more quantified path (transitive closure).
    ZeroOrMore { path: Box<SHACLPath> },
    /// A one-or-more quantified path (non-empty transitive closure).
    OneOrMore { path: Box<SHACLPath> },
    /// A zero-or-one quantified path (optional path).
    ZeroOrOne { path: Box<SHACLPath> },
}

impl SHACLPath {
    /// Creates a simple predicate path from an IRI.
    ///
    /// This is a convenience constructor for the most common path type,
    /// equivalent to `SHACLPath::Predicate { pred }`.
    ///
    /// # Arguments
    ///
    /// * `pred` - The IRI representing the predicate to navigate
    pub fn iri(pred: IriS) -> Self {
        SHACLPath::Predicate { pred }
    }

    /// Extracts the predicate IRI from a simple predicate path.
    pub fn pred(&self) -> Option<&IriS> {
        match self {
            SHACLPath::Predicate { pred } => Some(pred),
            _ => None,
        }
    }

    /// Creates a sequence path from multiple paths.
    ///
    /// # Arguments
    ///
    /// * `paths` - A vector of paths to compose in sequence
    pub fn sequence(paths: Vec<SHACLPath>) -> Self {
        SHACLPath::Sequence { paths }
    }

    /// Creates an alternative path from multiple paths.
    ///
    /// # Arguments
    ///
    /// * `paths` - A vector of alternative paths
    pub fn alternative(paths: Vec<SHACLPath>) -> Self {
        SHACLPath::Alternative { paths }
    }

    /// Creates an inverse path that navigates backwards.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to invert
    pub fn inverse(path: SHACLPath) -> Self {
        SHACLPath::Inverse { path: Box::new(path) }
    }

    /// Creates a zero-or-more quantified path (transitive closure).
    ///
    /// # Arguments
    ///
    /// * `path` - The path to repeat
    pub fn zero_or_more(path: SHACLPath) -> Self {
        SHACLPath::ZeroOrMore { path: Box::new(path) }
    }

    /// Creates a one-or-more quantified path (non-empty transitive closure).
    ///
    /// # Arguments
    ///
    /// * `path` - The path to repeat
    pub fn one_or_more(path: SHACLPath) -> Self {
        SHACLPath::OneOrMore { path: Box::new(path) }
    }

    /// Creates a zero-or-one quantified path (optional path).
    ///
    /// # Arguments
    ///
    /// * `path` - The optional path
    pub fn zero_or_one(path: SHACLPath) -> Self {
        SHACLPath::ZeroOrOne { path: Box::new(path) }
    }
}

impl Display for SHACLPath {
    /// Formats the SHACL path as a SPARQL-like property path expression.
    ///
    /// The output follows SPARQL property path syntax conventions:
    /// - Predicates are displayed as IRIs
    /// - Alternatives use `|` separator with parentheses
    /// - Sequences use `/` separator with parentheses
    /// - Inverses use `^` prefix with parentheses
    /// - Quantifiers use postfix operators: `*`, `+`, `?`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SHACLPath::Predicate { pred } => write!(f, "{pred}"),
            SHACLPath::Alternative { paths } => {
                write!(
                    f,
                    "({})",
                    paths
                        .iter()
                        .map(|p| format!("{p}"))
                        .collect::<Vec<String>>()
                        .join(" | ")
                )
            },
            SHACLPath::Sequence { paths } => write!(
                f,
                "({})",
                paths
                    .iter()
                    .map(|p| format!("{p}"))
                    .collect::<Vec<String>>()
                    .join(" / ")
            ),
            SHACLPath::Inverse { path } => {
                write!(f, "^({path})")
            },
            SHACLPath::ZeroOrMore { path } => {
                write!(f, "({path})*")
            },
            SHACLPath::OneOrMore { path } => {
                write!(f, "({path})+")
            },
            SHACLPath::ZeroOrOne { path } => {
                write!(f, "({path})?")
            },
        }
    }
}
