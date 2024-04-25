use iri_s::IriS;

/// Concrete representation of RDF subjects, which can be IRIs or Blank nodes
pub enum Subject {
    Iri { iri: IriS },
    BlankNode(String),
}
