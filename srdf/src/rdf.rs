trait RDF {
    fn parse(format: RDFFormat) -> Self;
}



pub enum RDFFormat {
    Turtle,
    NTriples,
    RDFXML,
}

pub enum Subject {
    Iri(Iri),
    BlankNode(BNode),
}

pub struct Triple {
    pub subject: Subject,
    pub predicate: Iri,
    pub object: Object,
}

pub enum Object {
    Iri(Iri),
    BlankNode(BNode),
}

pub struct Iri(String);

pub struct BNode(String);
