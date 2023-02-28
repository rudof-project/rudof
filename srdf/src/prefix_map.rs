use std::collections::HashMap;
use crate::iri::*;
use crate::alias::*;

pub struct PrefixMap {
    map: HashMap<Alias, Box<dyn IRI>>
} 

impl PrefixMap {
    pub fn new() -> PrefixMap {
        PrefixMap { map: HashMap::new() }
    }

    pub fn insert<I: IRI + 'static>(&mut self, alias: Alias, iri: I) -> &mut Self {
       self.map.insert(alias, Box::new(iri));
       self
    }
}