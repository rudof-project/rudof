use std::fmt::{Debug, Display};

use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};


/// Concrete representation of RDF subjects, which can be IRIs or Blank nodes
pub enum Subject {
    Iri { iri: IriS },
    BlankNode(String),
}
