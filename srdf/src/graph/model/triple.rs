use std::borrow::Cow;

use oxrdf::NamedNode as OxIri;
use oxrdf::NamedNodeRef as OxIriRef;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::TermRef as OxTermRef;
use oxrdf::Triple as OxTriple;
use oxrdf::TripleRef as OxTripleRef;

use crate::model::Triple;

impl Triple for OxTriple {
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

    fn from_spo(
        subject: impl Into<Self::Subject>,
        predicate: impl Into<Self::Iri>,
        object: impl Into<Self::Term>,
    ) -> Self {
        OxTriple::new(subject, predicate, object)
    }

    fn subject(&self) -> &OxSubject {
        &self.subject
    }

    fn predicate(&self) -> &OxIri {
        &self.predicate
    }

    fn object(&self) -> &OxTerm {
        &self.object
    }
}

impl<T: Triple + Clone> Triple for Cow<'_, T> {
    type Subject = T::Subject;
    type Iri = T::Iri;
    type Term = T::Term;

    fn from_spo(
        subject: impl Into<Self::Subject>,
        predicate: impl Into<Self::Iri>,
        object: impl Into<Self::Term>,
    ) -> Self {
        Cow::Owned(T::from_spo(subject, predicate, object))
    }

    fn subject(&self) -> &Self::Subject {
        match self {
            Cow::Borrowed(t) => t.subject(),
            Cow::Owned(t) => t.subject(),
        }
    }

    fn predicate(&self) -> &Self::Iri {
        match self {
            Cow::Borrowed(t) => t.predicate(),
            Cow::Owned(t) => t.predicate(),
        }
    }

    fn object(&self) -> &Self::Term {
        match self {
            Cow::Borrowed(t) => t.object(),
            Cow::Owned(t) => t.object(),
        }
    }
}

// impl Triple for OxTripleRef<'_> {
//     type SubjectRef<'x> = OxSubjectRef<'x> where Self: 'x;
//     type Iri<'x> = OxIriRef<'x> where Self: 'x;
//     type TermRef<'x> = OxTermRef<'x> where Self: 'x;

//     fn subject<'a>(&'a self) -> OxSubjectRef<'a> {
//         self.subject
//     }

//     fn predicate<'a>(&'a self) -> OxIriRef<'a> {
//         self.predicate
//     }

//     fn object<'a>(&'a self) -> OxTermRef<'a> {
//         self.object
//     }
// }

// impl<'a> FromComponents for OxTripleRef<'a> {
//     type Subject = OxSubjectRef<'a>;
//     type Predicate = OxIriRef<'a>;
//     type Object = OxTermRef<'a>;

//     fn from_spo(
//         subject: impl Into<Self::Subject>,
//         predicate: impl Into<Self::Predicate>,
//         object: impl Into<Self::Object>,
//     ) -> Self {
//         OxTripleRef::new(subject, predicate, object)
//     }
// }
