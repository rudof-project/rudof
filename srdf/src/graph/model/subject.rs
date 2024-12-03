use oxrdf::BlankNode as OxBlankNode;
use oxrdf::BlankNodeRef as OxBlankNodeRef;
use oxrdf::NamedNode as OxIri;
use oxrdf::NamedNodeRef as OxIriRef;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::TermRef as OxTermRef;
use oxrdf::Triple as OxTriple;
use oxrdf::TripleRef as OxTripleRef;

use crate::model::conversions::IntoTerm;
use crate::model::Subject;
use crate::oxgraph_error::SubjectConversionError;

impl Subject for OxSubject {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Triple = OxTriple;

    fn into_blank_node(&self) -> Option<&OxBlankNode> {
        if let OxSubject::BlankNode(blank_node) = self {
            Some(blank_node)
        } else {
            None
        }
    }

    fn into_iri(&self) -> Option<&OxIri> {
        if let OxSubject::NamedNode(named_node) = self {
            Some(named_node)
        } else {
            None
        }
    }

    fn into_triple(&self) -> Option<&OxTriple> {
        if let OxSubject::Triple(triple) = self {
            Some(triple)
        } else {
            None
        }
    }
}

impl IntoTerm for OxSubject {
    type Term = OxTerm;
    type Error = SubjectConversionError;

    fn try_into_term(&self) -> Result<Self::Term, Self::Error> {
        Ok(self.clone().into())
    }
}

// impl<'a> Subject for OxSubjectRef<'a> {
//     type BlankNodeRef<'x> = OxBlankNodeRef<'x> where Self: 'x;
//     type IriRef<'x> = OxIriRef<'x> where Self: 'x;
//     type TripleRef<'x> = OxTripleRef<'x> where Self: 'x;

//     fn into_blank_node(&self) -> Option<OxBlankNodeRef<'_>> {
//         if let OxSubjectRef::BlankNode(blank_node) = self {
//             Some(*blank_node)
//         } else {
//             None
//         }
//     }

//     fn into_iri(&self) -> Option<OxIriRef<'_>> {
//         if let OxSubjectRef::NamedNode(named_node) = self {
//             Some(*named_node)
//         } else {
//             None
//         }
//     }

//     fn into_triple(&self) -> Option<OxTripleRef<'_>> {
//         if let OxSubjectRef::Triple(triple) = self {
//             Some(triple.as_ref().into())
//         } else {
//             None
//         }
//     }
// }

// impl<'a> IntoTerm for OxSubjectRef<'a> {
//     type Term = OxTermRef<'a>;
//     type Error = SubjectConversionError;

//     fn try_into_term(&self) -> Result<Self::Term, Self::Error> {
//         Ok(self.clone().into())
//     }
// }
