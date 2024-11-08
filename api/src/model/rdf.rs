type BoxIterator<'a, I> = Box<dyn Iterator<Item = &'a I> + 'a>;
pub type Triples<'a, R> = BoxIterator<'a, <R as Rdf>::Triple>;
pub type Subjects<'a, R> = BoxIterator<'a, <R as Rdf>::Subject>;
pub type Predicates<'a, R> = BoxIterator<'a, <R as Rdf>::IRI>;
pub type Objects<'a, R> = BoxIterator<'a, <R as Rdf>::Term>;

pub trait Triple<R: Rdf> {
    fn subj(&self) -> &R::Subject;
    fn pred(&self) -> &R::IRI;
    fn obj(&self) -> &R::Term;
}

pub trait Rdf: Sized {
    type Subject; // subject
    type IRI; // predicate
    type Term; // object
    type BNode;
    type Literal;
    type Triple: Triple<Self>;

    type Error;

    fn triples_matching(
        &self,
        subject: Option<&Self::Subject>,
        predicate: Option<&Self::IRI>,
        object: Option<&Self::Term>,
    ) -> Result<Triples<Self>, Self::Error>;

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

    fn triples_with_subject(&self, subject: &Self::Subject) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(Some(subject), None, None)
    }

    fn triples_with_predicate(&self, predicate: &Self::IRI) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, Some(predicate), None)
    }

    fn triples_with_object(&self, object: &Self::Term) -> Result<Triples<Self>, Self::Error> {
        self.triples_matching(None, None, Some(object))
    }

    fn neighs(&self, node: &Self::Subject) -> Result<Objects<Self>, Self::Error> {
        let objects = self.triples_with_subject(node)?.map(Triple::obj);
        Ok(Box::new(objects))
    }
}
