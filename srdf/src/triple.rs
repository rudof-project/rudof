use std::fmt::Display;

use crate::Rdf;

pub struct Triple<S>
where
    S: Rdf + ?Sized,
{
    subj: S::Subject,
    pred: S::IRI,
    obj: S::Term,
}

impl<S> Triple<S>
where
    S: Rdf,
{
    pub fn new(subj: S::Subject, pred: S::IRI, obj: S::Term) -> Self {
        Triple { subj, pred, obj }
    }

    pub fn subj(&self) -> S::Subject {
        self.subj.clone()
    }

    pub fn pred(&self) -> S::IRI {
        self.pred.clone()
    }

    pub fn obj(&self) -> S::Term {
        self.obj.clone()
    }

    pub fn cnv<T: Rdf>(self) -> Triple<T>
    where
        T::Subject: From<S::Subject>,
        T::Term: From<S::Term>,
        T::IRI: From<S::IRI>,
    {
        Triple {
            subj: T::Subject::from(self.subj),
            pred: T::IRI::from(self.pred),
            obj: T::Term::from(self.obj),
        }
    }
}

impl<S> Display for Triple<S>
where
    S: Rdf,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.subj, self.pred, self.obj)
    }
}
