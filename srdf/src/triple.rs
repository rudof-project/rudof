use std::fmt::Display;

use crate::SRDFBasic;

pub struct Triple<S>
where
    S: SRDFBasic + ?Sized,
{
    subj: S::Subject,
    pred: S::IRI,
    obj: S::Term,
}

impl<S> Triple<S>
where
    S: SRDFBasic,
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
}

impl<S> Display for Triple<S>
where
    S: SRDFBasic,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.subj, self.pred, self.obj)
    }
}
