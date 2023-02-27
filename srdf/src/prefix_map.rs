use std::collections::HashMap;
use crate::iri::*;
use crate::alias::*;

#[derive(Debug, PartialEq)]
pub struct PrefixMap {
    map: HashMap<Alias, IRI>
} 

impl PrefixMap {
    pub fn new() -> PrefixMap {
        PrefixMap { map: HashMap::new() }
    }

    pub fn insert(&mut self, alias: Alias, iri: IRI) -> &mut Self {
       self.map.insert(alias, iri);
       self
    }
}