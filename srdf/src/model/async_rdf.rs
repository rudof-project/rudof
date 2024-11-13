use std::{fmt::Display};

use async_trait::async_trait;
use std::hash::Hash;
use crate::model::rdf::{Object, Objects, Predicate, Predicates, Subject, Subjects, Triples};
use crate::model::Triple;

#[async_trait]
pub trait AsyncRdf {
    type Triple: Triple;
    type Error;

    async fn triples_matching<'a>(
        &'a self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<Triples<'a, Self>, Self::Error>;

    async fn triples(&self) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, None)
    }

    async fn subjects(&self) -> Result<Subjects<Self>, Self::Error> {
        let subjects = self.triples()?.map(Triple::subj);
        Ok(Box::new(subjects))
    }

    async fn predicates(&self) -> Result<Predicates<Self>, Self::Error> {
        let predicates = self.triples()?.map(Triple::pred);
        Ok(Box::new(predicates))
    }

    async fn objects(&self) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples()?.map(Triple::obj);
        Ok(Box::new(objects))
    }

    async fn triples_with_subject<'a>(
        &'a self,
        subject: &'a Subject<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    async fn triples_with_predicate<'a>(
        &'a self,
        predicate: &'a Predicate<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    async fn triples_with_object<'a>(
        &'a self,
        object: &'a Object<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    async fn neighs<'a>(&'a self, node: &'a Subject<Self>) -> Result<Objects<'a, Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(Box::new(objects))
    }
}
