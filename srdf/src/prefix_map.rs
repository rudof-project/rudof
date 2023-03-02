use std::collections::HashMap;
use crate::iri::*;
use crate::alias::*;

pub struct PrefixMap<'a> {
    map: HashMap<Alias, Box<dyn IRI<'a>>>
} 

impl <'a> PrefixMap<'a> {
    pub fn new() -> PrefixMap<'a> {
        PrefixMap { map: HashMap::new() }
    }

    pub fn insert<I: IRI<'a> + 'static>(&mut self, alias: Alias, iri: I) -> &mut Self {
       self.map.insert(alias, Box::new(iri));
       self
    }
}