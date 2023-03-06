use std::fmt;
use indexmap::IndexMap;
use iri_s::*;

pub struct PrefixMap<'a> {
    map: IndexMap<&'a str, &'a IriS>
} 

fn split(str: &str) -> Option<(&str, &str)> {
    str.rsplit_once(":")
}


impl <'a> PrefixMap<'a> {
    pub fn new() -> PrefixMap<'a> {
        PrefixMap { map: IndexMap::new() }
    }

    pub fn insert(&mut self, alias: &'a str, iri: &'a IriS) -> &mut Self {
       self.map.insert(alias, iri);
       self
    }

    pub fn find(&self, str: &str) -> Option<&IriS> {
       let x = self.map.get(&str);
       match x {
        Some(&b) => { 
            let x = Some(b);
            x
        }
        None => None
       }
    } 
    
    pub fn resolve(&self, str: &str) -> Option<IriS> { 
        match split(str) {
            Some((alias, local_name)) => {
              match self.find(alias) {
                Some(iri) => {
                    Some(iri.extend(local_name))
                },
                None => Some(IriS::from_str(str))
              }
            },
            None => Some(IriS::from_str(str))
        }

    }

}

impl <'a> fmt::Display for PrefixMap<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (&alias, &iri) in self.map.iter().clone() {
        writeln!(f,"{} {}", &alias, &iri)?
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_ex_name()  {
        assert_eq!(split("ex:name"), Some(("ex", "name")))
    }

    #[test]
    fn prefix_map1() {
        let mut pm = PrefixMap::new();
        let binding = IriS::from_str("http://example.org/"); 
        pm.insert("ex", &binding);
        let resolved = IriS::from_str("http://example.org/name");
        assert_eq!(pm.resolve("ex:name").unwrap(), resolved);
    }

    #[test]
    fn prefix_map_display() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/"); 
        pm.insert("ex", &ex_iri);
        let ex_rdf = IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#"); 
        pm.insert("rdf", &ex_rdf);
        assert_eq!(pm.to_string(), 
          "ex <http://example.org/>\nrdf <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n");
    }
}

