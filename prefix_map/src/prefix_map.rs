use std::fmt;
use indexmap::IndexMap;
use iri_s::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct PrefixMap<'a> {
    map: IndexMap<&'a str, &'a IriS>
} 

fn split(str: &str) -> Option<(&str, &str)> {
    str.rsplit_once(":")
}


impl <'a> PrefixMap<'a> {
    pub fn new() -> PrefixMap<'a> {
        PrefixMap::default()
    }

    pub fn insert(&mut self, alias: &'a str, iri: &'a IriS) {
       self.map.insert(alias, iri);
    }

    pub fn find(&self, str: &str) -> Option<&IriS> {
       match self.map.get(&str) {
        Some(&b) => { 
            Some(b)
        }
        None => None
       }
    } 
    
    
    pub fn resolve(&self, str: &str) -> Result<Option<IriS>, IriError> { 
        match split(str) {
            Some((alias, local_name)) => {
              match self.find(alias) {
                Some(iri) => {
                    let new_iri = iri.extend(local_name)?;
                    Ok(Some(new_iri))
                },
                None => { 
                    let iri = IriS::from_str(str)?;
                    Ok(Some(iri))
                }
              }
            },
            None => {
                let iri = IriS::from_str(str)?;
                Ok(Some(iri))
            }
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

impl <'a> Default for PrefixMap<'a> {
    fn default() -> PrefixMap<'a> {
        PrefixMap { map: IndexMap::new() }
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
        let binding = IriS::from_str("http://example.org/").unwrap(); 
        pm.insert("ex", &binding);
        let resolved = IriS::from_str("http://example.org/name").unwrap();
        assert_eq!(pm.resolve("ex:name").unwrap().unwrap(), resolved);
    }

    #[test]
    fn prefix_map_display() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap(); 
        pm.insert("ex", &ex_iri);
        let ex_rdf = IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap(); 
        pm.insert("rdf", &ex_rdf);
        assert_eq!(pm.to_string(), 
          "ex <http://example.org/>\nrdf <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n");
    }

    #[test]
    fn prefix_map_resolve() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap(); 
        pm.insert("ex", &ex_iri);
        assert_eq!(pm.resolve(&"ex:pepe").unwrap(), 
          Some(IriS::from_str("http://example.org/pepe").unwrap()));
    }

}

