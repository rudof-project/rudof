use crate::BlankNode;
use crate::Iri;
use crate::Literal;
use crate::SLiteral;
use crate::Subject;
use crate::Term;
use crate::TermKind;
use crate::Triple;
use crate::lang::Lang;
use crate::matcher::Matcher;
use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxNamedNode;
use oxrdf::NamedOrBlankNode as OxSubject;
use oxrdf::NamedOrBlankNodeRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;

impl Subject for OxSubject {
    fn kind(&self) -> TermKind {
        match self {
            OxSubject::NamedNode(_) => TermKind::Iri,
            OxSubject::BlankNode(_) => TermKind::BlankNode,
        }
    }
}

impl Subject for OxSubjectRef<'_> {
    fn kind(&self) -> TermKind {
        match self {
            OxSubjectRef::NamedNode(_) => TermKind::Iri,
            OxSubjectRef::BlankNode(_) => TermKind::BlankNode,
        }
    }
}

impl Matcher<OxSubject> for OxSubject {
    fn value(&self) -> Option<OxSubject> {
        Some(self.clone())
    }
}

impl Iri for OxNamedNode {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl Matcher<OxNamedNode> for OxNamedNode {
    fn value(&self) -> Option<OxNamedNode> {
        Some(self.clone())
    }
}

impl Term for OxTerm {
    fn kind(&self) -> TermKind {
        match self {
            OxTerm::NamedNode(_) => TermKind::Iri,
            OxTerm::BlankNode(_) => TermKind::BlankNode,
            OxTerm::Literal(_) => TermKind::Literal,
            OxTerm::Triple(_) => TermKind::Triple,
        }
    }
    fn lexical_form(&self) -> String {
        match self {
            OxTerm::NamedNode(iri) => iri.as_str().to_string(),
            OxTerm::BlankNode(bnode) => bnode.as_str().to_string(),
            OxTerm::Literal(literal) => literal.value().to_string(),
            OxTerm::Triple(triple) => triple.to_string(),
        }
    }
}

impl Matcher<OxTerm> for OxTerm {
    fn value(&self) -> Option<OxTerm> {
        Some(self.clone())
    }
}

impl Literal for OxLiteral {
    fn lexical_form(&self) -> &str {
        self.value()
    }

    fn lang(&self) -> Option<Lang> {
        self.language().and_then(|lang| Lang::new(lang).ok())
    }

    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }

    fn as_sliteral(&self) -> Option<SLiteral> {
        todo!()
    }
}

impl BlankNode for OxBlankNode {
    fn new(id: impl Into<String>) -> Self {
        OxBlankNode::new_unchecked(id)
    }

    fn id(&self) -> &str {
        self.as_str()
    }
}

impl Triple<OxSubject, OxNamedNode, OxTerm> for OxTriple {
    fn new(
        subj: impl Into<OxSubject>,
        pred: impl Into<OxNamedNode>,
        obj: impl Into<OxTerm>,
    ) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subj(&self) -> &OxSubject {
        &self.subject
    }

    fn pred(&self) -> &OxNamedNode {
        &self.predicate
    }

    fn obj(&self) -> &OxTerm {
        &self.object
    }

    fn into_components(self) -> (OxSubject, OxNamedNode, OxTerm) {
        (self.subject, self.predicate, self.object)
    }
}
