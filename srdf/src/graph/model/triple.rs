use oxrdf::NamedNode as OxIri;
use oxrdf::NamedNodeRef as OxIriRef;
use oxrdf::Subject as OxSubject;
use oxrdf::SubjectRef as OxSubjectRef;
use oxrdf::Term as OxTerm;
use oxrdf::TermRef as OxTermRef;
use oxrdf::Triple as OxTriple;
use oxrdf::TripleRef as OxTripleRef;

use crate::model::FromComponents;
use crate::model::Triple;

impl Triple for OxTriple {
    type SubjectRef<'a> = OxSubjectRef<'a> where Self: 'a;
    type Iri<'a> = OxIriRef<'a> where Self: 'a;
    type TermRef<'a> = OxTermRef<'a> where Self: 'a;

    fn subject(&self) -> OxSubjectRef<'_> {
        self.subject.as_ref()
    }

    fn predicate(&self) -> OxIriRef<'_> {
        self.predicate.as_ref()
    }

    fn object(&self) -> OxTermRef<'_> {
        self.object.as_ref()
    }
}

impl FromComponents for OxTriple {
    type Subject = OxSubject;
    type Predicate = OxIri;
    type Object = OxTerm;

    fn from_spo(
        subject: impl Into<Self::Subject>,
        predicate: impl Into<Self::Predicate>,
        object: impl Into<Self::Object>,
    ) -> Self {
        OxTriple::new(subject, predicate, object)
    }
}

impl Triple for OxTripleRef<'_> {
    type SubjectRef<'x> = OxSubjectRef<'x> where Self: 'x;
    type Iri<'x> = OxIriRef<'x> where Self: 'x;
    type TermRef<'x> = OxTermRef<'x> where Self: 'x;

    fn subject<'a>(&'a self) -> OxSubjectRef<'a> {
        self.subject
    }

    fn predicate<'a>(&'a self) -> OxIriRef<'a> {
        self.predicate
    }

    fn object<'a>(&'a self) -> OxTermRef<'a> {
        self.object
    }
}

impl<'a> FromComponents for OxTripleRef<'a> {
    type Subject = OxSubjectRef<'a>;
    type Predicate = OxIriRef<'a>;
    type Object = OxTermRef<'a>;

    fn from_spo(
        subject: impl Into<Self::Subject>,
        predicate: impl Into<Self::Predicate>,
        object: impl Into<Self::Object>,
    ) -> Self {
        OxTripleRef::new(subject, predicate, object)
    }
}
