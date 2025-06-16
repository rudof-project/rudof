use std::fmt::Debug;
use std::fmt::Display;

use crate::Iri;
use crate::Rdf;
use crate::Subject;
use crate::Term;

pub trait Triple<S, P, O>: Debug + Clone + Display
where
    S: Subject,
    P: Iri,
    O: Term,
{
    fn new(subj: impl Into<S>, pred: impl Into<P>, obj: impl Into<O>) -> Self;

    fn subj(&self) -> S;
    fn pred(&self) -> P;
    fn obj(&self) -> O;

    fn into_components(self) -> (S, P, O);

    fn into_subject(self) -> S {
        self.into_components().0
    }

    fn into_predicate(self) -> P {
        self.into_components().1
    }

    fn into_object(self) -> O {
        self.into_components().2
    }
}

pub struct STriple<R>
where
    R: Rdf,
{
    subj: R::Subject,
    pred: R::IRI,
    obj: R::Term,
}

impl<R> STriple<R>
where
    R: Rdf,
{
    pub fn new(subj: R::Subject, pred: R::IRI, obj: R::Term) -> Self {
        STriple { subj, pred, obj }
    }

    pub fn subj(&self) -> R::Subject {
        self.subj.clone()
    }

    pub fn pred(&self) -> R::IRI {
        self.pred.clone()
    }

    pub fn obj(&self) -> R::Term {
        self.obj.clone()
    }

    pub fn cnv<T: Rdf>(self) -> STriple<T>
    where
        T::Subject: From<R::Subject>,
        T::Term: From<R::Term>,
        T::IRI: From<R::IRI>,
    {
        STriple {
            subj: T::Subject::from(self.subj),
            pred: T::IRI::from(self.pred),
            obj: T::Term::from(self.obj),
        }
    }
}

impl<R> Display for STriple<R>
where
    R: Rdf,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.subj, self.pred, self.obj)
    }
}
