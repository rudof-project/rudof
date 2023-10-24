use colored::*;
use indexmap::IndexMap;
use iri_s::*;
use serde::{Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

use std::result;
use std::str::FromStr;
use std::{collections::HashMap, fmt};
use crate::PrefixMapError;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct PrefixMap {
    map: IndexMap<String, IriS>,
}

fn split(str: &str) -> Option<(&str, &str)> {
    str.rsplit_once(":")
}

impl PrefixMap {
    pub fn new() -> PrefixMap {
        PrefixMap::default()
    }

    pub fn insert(&mut self, alias: &str, iri: &IriS) {
        self.map.insert(alias.to_owned(), iri.clone());
    }

    pub fn find(&self, str: &str) -> Option<&IriS> {
        self.map.get(str)
    }

    pub fn from_hashmap(hm: &HashMap<String, String>) -> Result<PrefixMap, IriSError> {
        let mut pm = PrefixMap::new();
        for (a, s) in hm.iter() {
            let iri = IriS::from_str(s)?;
            pm.insert(a, &iri);
        }
        Ok(pm)
    }

    /// Resolves a string against a prefix map
    /// Example:
    /// Given a prefix map `pm`
    /// ```
    /// use std::collections::HashMap;
    /// use prefixmap::PrefixMap;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    ///
    ///
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("".to_string(), "http://example.org/".to_string()),
    ///     ("schema".to_string(), "http://schema.org/".to_string())])
    /// )?;
    /// let a = pm.resolve(":a")?;
    /// let a_resolved = IriS::from_str("http://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    ///
    /// let knows = pm.resolve("schema:knows")?;
    /// let knows_resolved = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    /// # Ok::<(), IriSError>(())
    /// ```
    pub fn resolve(&self, str: &str) -> Result<IriS, PrefixMapError> {
        match split(str) {
            Some((prefix, local)) => {
                let iri = self.resolve_prefix_local(prefix, local)?;
                Ok(iri)
            }
            None => {
                let iri = IriS::from_str(str)?;
                Ok(iri)
            }
        }
    }

    /// Resolves a prefix and a local name against a prefix map
    /// Example:
    /// Given a prefix map `pm`
    /// ```
    /// use std::collections::HashMap;
    /// use prefixmap::PrefixMap;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    ///
    ///
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("".to_string(), "http://example.org/".to_string()),
    ///     ("schema".to_string(), "http://schema.org/".to_string())
    ///     ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string())
    /// ])
    /// 
    /// )?;
    /// let a = pm.resolve_prefix_local("", "a")?;
    /// let a_resolved = IriS::from_str("http://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    ///
    /// let knows = pm.resolve_prefix_local("schema","knows")?;
    /// let knows_resolved = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    /// 
    /// let xsd_string = pm.resolve_prefix_local("xsd","string")?;
    /// let xsd_string_resolved = IriS::from_str("http://www.w3.org/2001/XMLSchema#string")?;
    /// assert_eq!(xsd_string, xsd_string_resolved);
    /// # Ok::<(), IriSError>(())
    /// ```
    pub fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        match self.find(prefix) {
                Some(iri) => {
                    let new_iri = iri.extend(local)?;
                    Ok(new_iri)
                }
                None => {
                    Err(PrefixMapError::PrefixNotFound {
                        prefix: prefix.to_string(),
                        prefixmap: self.clone()
                    })
                }
      }
    }

    /// Qualifies an IRI against a prefix map
    /// Example:
    /// Given a prefix map `pm`
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("".to_string(), "http://example.org/".to_string()),
    ///     ("schema".to_string(), "http://schema.org/".to_string())])
    /// )?;
    /// let a = IriS::from_str("http://example.org/a")?;
    /// assert_eq!(pm.qualify(&a), ":a");
    ///
    /// let knows = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(pm.qualify(&knows), "schema:knows");
    ///
    /// let other = IriS::from_str("http://other.org/foo")?;
    /// assert_eq!(pm.qualify(&other), "<http://other.org/foo>");
    /// # Ok::<(), IriSError>(())
    /// ```
    pub fn qualify(&self, iri: &IriS) -> String {
        for (alias, pm_iri) in &self.map {
            if let Some(rest) = iri.as_str().strip_prefix(pm_iri.as_str()) {
                let result = format!("{}:{}", alias.blue(), rest);
                return result;
            }
        }
        format!("{iri}")
    }
}

impl fmt::Display for PrefixMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (alias, iri) in self.map.iter() {
            writeln!(f, "{} {}", &alias, &iri)?
        }
        Ok(())
    }
}

impl Default for PrefixMap {
    fn default() -> PrefixMap {
        PrefixMap {
            map: IndexMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_ex_name() {
        assert_eq!(split("ex:name"), Some(("ex", "name")))
    }

    #[test]
    fn prefix_map1() {
        let mut pm = PrefixMap::new();
        let binding = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", &binding);
        let expected = IriS::from_str("http://example.org/name").unwrap();
        assert_eq!(pm.resolve("ex:name").unwrap(), expected);
    }

    #[test]
    fn prefixmap_display() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", &ex_iri);
        let ex_rdf = IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
        pm.insert("rdf", &ex_rdf);
        assert_eq!(
            pm.to_string(),
            "ex <http://example.org/>\nrdf <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n"
        );
    }

    #[test]
    fn prefixmap_resolve() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", &ex_iri);
        assert_eq!(
            pm.resolve(&"ex:pepe").unwrap(),
            IriS::from_str("http://example.org/pepe").unwrap()
        );
    }

    #[test]
    fn prefixmap_resolve_xsd() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://www.w3.org/2001/XMLSchema#").unwrap();
        pm.insert("xsd", &ex_iri);
        assert_eq!(
            pm.resolve_prefix_local("xsd", "string").unwrap(),
            IriS::from_str("http://www.w3.org/2001/XMLSchema#string").unwrap()
        );
    }

}
