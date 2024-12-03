use oxrdf::BlankNodeRef as OxBlankNodeRef;
use oxrdf::LiteralRef as OxLiteralRef;
use oxrdf::NamedNodeRef as OxIriRef;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::TermRef as OxTermRef;
use oxrdf::TripleRef as OxTripleRef;

use crate::model::conversions::IntoSubject;
use crate::model::Term;
use crate::oxgraph_error::TermConversionError;

impl Term for OxTerm {
    type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;
    type LiteralRef<'x> = OxLiteralRef<'x> where Self: 'x;
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxTerm::BlankNode(blank_node) = self {
            Some(blank_node.as_ref())
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxTerm::NamedNode(named_node) = self {
            Some(named_node.as_ref())
        } else {
            None
        }
    }

    fn into_literal(&self) -> Option<OxLiteralRef<'_>> {
        if let OxTerm::Literal(literal) = self {
            Some(literal.as_ref())
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxTerm::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl IntoSubject for OxTerm {
    type Subject = OxSubject;
    type Error = TermConversionError;

    fn try_into_subject(&self) -> Result<Self::Subject, Self::Error> {
        match self.clone() {
            OxTerm::BlankNode(blank_node) => Ok(blank_node.into()),
            OxTerm::NamedNode(named_node) => Ok(named_node.into()),
            OxTerm::Triple(triple) => Ok(triple.into()),
            term => Err(TermConversionError::FromSubject(term.to_string())),
        }
    }
}

impl<'a> Term for OxTermRef<'a> {
    type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;
    type LiteralRef<'x> = OxLiteralRef<'x> where Self: 'x;
    type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;

    fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
        if let OxTermRef::BlankNode(blank_node) = self {
            Some(*blank_node)
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<OxIriRef<'_>> {
        if let OxTermRef::NamedNode(named_node) = self {
            Some(*named_node)
        } else {
            None
        }
    }

    fn into_literal(&self) -> Option<OxLiteralRef<'_>> {
        if let OxTermRef::Literal(literal) = self {
            Some(*literal)
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<OxTripleRef<'_>> {
        if let OxTermRef::Triple(triple) = self {
            Some(triple.as_ref().into())
        } else {
            None
        }
    }
}

impl<'a> IntoSubject for OxTermRef<'a> {
    type Subject = OxSubjectRef<'a>;
    type Error = TermConversionError;

    fn try_into_subject(&self) -> Result<Self::Subject, Self::Error> {
        match self {
            OxTermRef::BlankNode(blank_node) => Ok((*blank_node).into()),
            OxTermRef::NamedNode(named_node) => Ok((*named_node).into()),
            OxTermRef::Triple(triple) => Ok((*triple).into()),
            term => Err(TermConversionError::FromSubject(term.to_string())),
        }
    }
}
