use super::Subject;
use super::Term;

pub trait IntoSubject: Sized {
    type Subject: Subject;
    type Error;
    fn try_into_subject(&self) -> Result<Self::Subject, Self::Error>;
}

pub trait IntoTerm: Sized {
    type Term: Term;
    type Error;
    fn try_into_term(&self) -> Result<Self::Term, Self::Error>;
}
