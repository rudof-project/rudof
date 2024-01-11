use prefixmap::PrefixMap;

use crate::{SRDF, RDFNode};

pub trait SRDFBuilder: SRDF {

    fn add_base(&mut self, base: &Self::IRI) -> Result<(), Self::Err>;
    fn add_prefix(&mut self, alias: &str, iri: &Self::IRI) -> Result<(), Self::Err>;
    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err>;
    fn add_triple(&mut self, subj: &Self::Subject, pred: &Self::IRI, obj: &Self::Term) -> Result<(), Self::Err>;
    fn remove_triple(&mut self, subj: &Self::Subject, pred: &Self::IRI, obj: &Self::Term) -> Result<(), Self::Err>;
    fn add_type(&mut self, node: &RDFNode, type_: Self::Term) -> Result<(), Self::Err>;
}