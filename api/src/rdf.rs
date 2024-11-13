use super::Triple;

type BoxIterator<'a, I> = Box<dyn Iterator<Item = &'a I> + 'a>;

pub type Triples<'a, R> = BoxIterator<'a, <R as Rdf>::Triple>;
pub type Subject<'a, R> = <<R as Rdf>::Triple as Triple>::Subject;
pub type Subjects<'a, R> = BoxIterator<'a, Subject<'a, R>>;
pub type Predicate<'a, R> = <<R as Rdf>::Triple as Triple>::Iri;
pub type Predicates<'a, R> = BoxIterator<'a, Predicate<'a, R>>;
pub type Object<'a, R> = <<R as Rdf>::Triple as Triple>::Term;
pub type Objects<'a, R> = BoxIterator<'a, Object<'a, R>>;

/// This trait provides methods to handle Simple RDF graphs.
///
/// * Finding the triples provided a given pattern
/// * Obtaining the neighborhood of a node
pub trait Rdf {
    type Triple: Triple;
    type Error;

    fn triples_matching<'a>(
        &'a self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
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
        subject: &'a Subject<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate<'a>(
        &'a self,
        predicate: &'a Predicate<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object<'a>(
        &'a self,
        object: &'a Object<Self>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs<'a>(&'a self, node: &'a Subject<Self>) -> Result<Objects<'a, Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(Box::new(objects))
    }
}
