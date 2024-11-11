use super::Triple;

type BoxIterator<'a, I> = Box<dyn Iterator<Item = &'a I> + 'a>;
pub type Triples<'a, R> = BoxIterator<'a, <R as Rdf>::Triple>;
pub type RdfSubject<'a, R> = <<R as Rdf>::Triple as Triple>::Subject;
pub type Subjects<'a, R> = BoxIterator<'a, RdfSubject<'a, R>>;
pub type RdfIri<'a, R> = <<R as Rdf>::Triple as Triple>::Iri;
pub type Predicates<'a, R> = BoxIterator<'a, RdfIri<'a, R>>;
pub type RdfTerm<'a, R> = <<R as Rdf>::Triple as Triple>::Term;
pub type Objects<'a, R> = BoxIterator<'a, RdfTerm<'a, R>>;

pub trait Rdf {
    type Triple: Triple;
    type Error;

    fn triples_matching<'a>(
        &'a self,
        subject: Option<&'a RdfSubject<Self>>,
        predicate: Option<&'a RdfIri<Self>>,
        object: Option<&'a RdfTerm<Self>>,
    ) -> Result<Triples<'a, Self>, Self::Error>;

    fn triples(&self) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, None)
    }

    fn subjects(&self) -> Result<Subjects<Self>, Self::Error> {
        let subjects = self.triples()?.map(Triple::subj);
        Ok(Box::new(subjects))
    }

    fn predicates(&self) -> Result<Predicates<Self>, Self::Error> {
        let predicates = self.triples()?.map(Triple::pred);
        Ok(Box::new(predicates))
    }

    fn objects(&self) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples()?.map(Triple::obj);
        Ok(Box::new(objects))
    }

    fn triples_with_subject<'a>(
        &'a self,
        subject: &'a RdfSubject<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate<'a>(
        &'a self,
        predicate: &'a RdfIri<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object<'a>(
        &'a self,
        object: &'a RdfTerm<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs<'a>(&'a self, node: &'a RdfSubject<Self>) -> Result<Objects<'a, Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(Box::new(objects))
    }
}

pub trait MutableRdf: Rdf {
    type MutableError;

    fn add_triple(
        &mut self,
        subject: RdfSubject<Self>,
        predicate: RdfIri<Self>,
        object: RdfTerm<Self>,
    ) -> Result<(), Self::MutableError>;

    fn remove_triple(&mut self, triple: &Self::Triple) -> Result<(), Self::MutableError>;

    fn add_base(&mut self, base: &RdfIri<Self>) -> Result<(), Self::Error>;

    fn add_prefix(&mut self, alias: &str, iri: &RdfIri<Self>) -> Result<(), Self::Error>;
}
