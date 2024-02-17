use std::fmt::{Debug, Display};

use crate::literal::Literal;
use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};
use crate::Object;

/// Concrete representation of RDF nodes, which are equivalent to objects
pub type RDFNode = Object;

