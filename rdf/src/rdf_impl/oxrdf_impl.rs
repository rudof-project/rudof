use crate::rdf_core::{
        Matcher,
        term::{BlankNode, Term, TermKind, Iri, Subject, Triple, 
            literal::{ConcreteLiteral, Lang, Literal}}
};
use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode,
    NamedOrBlankNode as OxSubject, NamedOrBlankNodeRef as OxSubjectRef,
    Term as OxTerm, Triple as OxTriple
};
use prefixmap::IriRef;
use iri_s::IriS;

/// Implements the `Subject` trait for `OxSubject` (owned version).
///
/// This allows owned `NamedOrBlankNode` instances from `oxrdf` to be used
/// as RDF subjects in the core trait system.
impl Subject for OxSubject {
    /// Returns the kind of term this subject represents.
    ///
    /// # Returns
    /// - `TermKind::Iri` for named nodes (IRIs)
    /// - `TermKind::BlankNode` for blank nodes
    fn kind(&self) -> TermKind {
        match self {
            OxSubject::NamedNode(_) => TermKind::Iri,
            OxSubject::BlankNode(_) => TermKind::BlankNode,
        }
    }
}

/// Implements the `Subject` trait for `OxSubjectRef` (borrowed version).
///
/// This allows borrowed `NamedOrBlankNodeRef` instances from `oxrdf` to be
/// used as RDF subjects without requiring ownership.
impl Subject for OxSubjectRef<'_> {
    /// Returns the kind of term this subject reference represents.
    ///
    /// # Returns
    /// - `TermKind::Iri` for named nodes (IRIs)
    /// - `TermKind::BlankNode` for blank nodes
    fn kind(&self) -> TermKind {
        match self {
            OxSubjectRef::NamedNode(_) => TermKind::Iri,
            OxSubjectRef::BlankNode(_) => TermKind::BlankNode,
        }
    }
}

/// Implements the `Iri` trait for `OxNamedNode`.
///
/// Provides access to the string representation of an IRI from an `oxrdf`
/// named node.
impl Iri for OxNamedNode {
    /// Returns the IRI as a string slice.
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

/// Implements the `Term` trait for `OxTerm`.
///
/// This is the main implementation for RDF terms, supporting IRIs, blank nodes,
/// literals, and RDF-star quoted triples.
impl Term for OxTerm {
    /// Returns the kind of RDF term.
    ///
    /// # Returns
    /// - `TermKind::Iri` for named nodes
    /// - `TermKind::BlankNode` for blank nodes
    /// - `TermKind::Literal` for literal values
    /// - `TermKind::Triple` for RDF-star quoted triples
    fn kind(&self) -> TermKind {
        match self {
            OxTerm::NamedNode(_) => TermKind::Iri,
            OxTerm::BlankNode(_) => TermKind::BlankNode,
            OxTerm::Literal(_) => TermKind::Literal,
            OxTerm::Triple(_) => TermKind::Triple,
        }
    }

    // Returns the lexical form of this term as an owned string.
    /// # Returns
    /// - For IRIs: the full IRI string
    /// - For blank nodes: the blank node identifier (e.g., "_:b0")
    /// - For literals: the literal value without datatype or language tag
    /// - For triples: the string representation of the entire triple
    fn lexical_form(&self) -> String {
        match self {
            OxTerm::NamedNode(iri) => iri.as_str().to_string(),
            OxTerm::BlankNode(bnode) => bnode.as_str().to_string(),
            OxTerm::Literal(literal) => literal.value().to_string(),
            OxTerm::Triple(triple) => triple.to_string(),
        }
    }
}

/// Implements the `Matcher` trait for `OxNamedNode`, enabling pattern matching.
///
/// This trivial implementation always matches by cloning the named node.
impl Matcher<OxNamedNode> for OxNamedNode {
    /// Always returns `Some(self)` indicating a successful match.
    fn value(&self) -> Option<&OxNamedNode> {
        Some(self)
    }
}

/// Implements the `Matcher` trait for `OxSubject`, enabling pattern matching.
impl Matcher<OxSubject> for OxSubject {
    /// Always returns `Some(self)` indicating a successful match.
    fn value(&self) -> Option<&OxSubject> {
        Some(self)
    }
}

/// Implements the `Matcher` trait for `OxTerm`, enabling pattern matching.
impl Matcher<OxTerm> for OxTerm {
    /// Always returns `Some(self)` indicating a successful match.
    fn value(&self) -> Option<&OxTerm> {
        Some(self)
    }
}

/// Implements the `Literal` trait for `OxLiteral`.
///
/// Provides access to literal values, language tags, and datatypes from
/// `oxrdf` literals.
impl Literal for OxLiteral {
    /// Returns the lexical form of the literal as a string slice.
    fn lexical_form(&self) -> &str {
        self.value()
    }

    /// Returns the language tag if this is a language-tagged string.
    ///
    /// # Returns
    /// - `Some(Lang)` if this literal has a language tag (e.g., "en", "es")
    /// - `None` if this literal is not language-tagged
    fn lang(&self) -> Option<Lang> {
        self.language().and_then(|lang| Lang::new(lang).ok())
    }

    /// Returns the datatype IRI of this literal.
    fn datatype(&self) -> IriRef {
        IriRef::iri(IriS::new_unchecked(self.datatype().as_str()))
    }

    /// Attempts to convert this literal to a concrete typed value.
    ///
    /// # Returns
    /// `Some(ConcreteLiteral)` if the literal can be parsed into a concrete type,
    /// `None` otherwise.
    fn to_concrete_literal(&self) -> Option<ConcreteLiteral> {
        todo!()
    }
}

/// Implements the `BlankNode` trait for `OxBlankNode`.
///
/// Provides construction and access to blank node identifiers.
impl BlankNode for OxBlankNode {
    /// Creates a new blank node with the given identifier.
    ///
    /// # Parameters
    /// - `id`: The blank node identifier (converted to `String`)
    fn new(id: impl Into<String>) -> Self {
        OxBlankNode::new_unchecked(id)
    }

    /// Returns the identifier of this blank node.
    fn id(&self) -> &str {
        self.as_str()
    }
}

/// Implements the `Triple` trait for `OxTriple`.
///
/// Provides construction, field access, and decomposition for RDF triples
/// consisting of subject, predicate, and object.
impl Triple<OxSubject, OxNamedNode, OxTerm> for OxTriple {
    /// Creates a new RDF triple from subject, predicate, and object.
    ///
    /// # Parameters
    /// - `subj`: The subject (IRI or blank node)
    /// - `pred`: The predicate (IRI/named node)
    /// - `obj`: The object (IRI, blank node, literal, or quoted triple)
    fn new(
        subj: impl Into<OxSubject>,
        pred: impl Into<OxNamedNode>,
        obj: impl Into<OxTerm>,
    ) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    /// Returns a reference to the subject of this triple.
    fn subj(&self) -> &OxSubject {
        &self.subject
    }

    /// Returns a reference to the predicate of this triple.
    fn pred(&self) -> &OxNamedNode {
        &self.predicate
    }

    /// Returns a reference to the object of this triple.
    fn obj(&self) -> &OxTerm {
        &self.object
    }

    /// Consumes the triple and returns its components.
    ///
    /// This method is useful when you need owned values rather than references,
    /// avoiding additional clones.
    fn into_components(self) -> (OxSubject, OxNamedNode, OxTerm) {
        (self.subject, self.predicate, self.object)
    }
}