use colored::*;
use indexmap::map::Iter;
use indexmap::IndexMap;
use iri_s::*;
// use serde::{Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

use crate::PrefixMapError;
use std::str::FromStr;
use std::{collections::HashMap, fmt};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(transparent)]
pub struct PrefixMap {
    pub map: IndexMap<String, IriS>,

    #[serde(skip)]
    qualify_prefix_color: Option<Color>,

    #[serde(skip)]
    qualify_localname_color: Option<Color>,

    #[serde(skip)]
    qualify_semicolon_color: Option<Color>,
}

fn split(str: &str) -> Option<(&str, &str)> {
    str.rsplit_once(":")
}

impl PrefixMap {
    /// Creates an empty prefix map
    pub fn new() -> PrefixMap {
        PrefixMap::default()
    }

    /// Change prefix color when qualifying a IRI
    pub fn with_qualify_prefix_color(mut self, color: Option<Color>) -> Self {
        self.qualify_prefix_color = color;
        self
    }

    /// Change color of localname when qualifying a IRI
    pub fn with_qualify_localname_color(mut self, color: Option<Color>) -> Self {
        self.qualify_localname_color = color;
        self
    }

    /// Change color of semicolon when qualifying a IRI
    pub fn with_qualify_semicolon_color(mut self, color: Option<Color>) -> Self {
        self.qualify_semicolon_color = color;
        self
    }

    /// Inserts an alias association to an IRI
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

    /// Return an iterator over the key-value pairs of the prefix map, in their order
    pub fn iter(&self) -> Iter<String, IriS> {
        self.map.iter()
    }

    /// Resolves a string against a prefix map
    /// Example:
    /// Given a prefix map `pm`
    /// ```
    /// use std::collections::HashMap;
    /// use prefixmap::PrefixMap;
    /// use prefixmap::PrefixMapError;
    /// use iri_s::*;
    /// use std::str::FromStr;
    ///
    ///
    /// let pm: PrefixMap = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("".to_string(), "http://example.org/".to_string()),
    ///     ("schema".to_string(), "http://schema.org/".to_string())])
    /// )?;
    /// let a = pm.resolve(":a")?;
    /// let a_resolved = IriS::from_str("http://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    /// Ok::<(), PrefixMapError>(());
    ///
    /// let knows = pm.resolve("schema:knows")?;
    /// let knows_resolved = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    /// Ok::<(), PrefixMapError>(())
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
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    ///
    ///
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("".to_string(), "http://example.org/".to_string()),
    ///     ("schema".to_string(), "http://schema.org/".to_string()),
    ///     ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string())
    /// ]))?;
    ///
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
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        match self.find(prefix) {
            Some(iri) => {
                let new_iri = iri.extend(local)?;
                Ok(new_iri)
            }
            None => Err(PrefixMapError::PrefixNotFound {
                prefix: prefix.to_string(),
                prefixmap: self.clone(),
            }),
        }
    }

    /// Qualifies an IRI against a prefix map
    /// Example:
    /// Given a prefix map `pm`
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
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
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify(&self, iri: &IriS) -> String {
        let mut founds: Vec<_> = self
            .map
            .iter()
            .filter_map(|(alias, pm_iri)| {
                iri.as_str()
                    .strip_prefix(pm_iri.as_str())
                    .map(|rest| (alias, rest))
            })
            .collect();
        founds.sort_by_key(|(_, iri)| iri.len());
        if let Some((alias, rest)) = founds.first() {
            let prefix_colored = match self.qualify_prefix_color {
                Some(color) => alias.color(color),
                None => ColoredString::from(alias.as_str()),
            };
            let rest_colored = match self.qualify_localname_color {
                Some(color) => rest.color(color),
                None => ColoredString::from(*rest),
            };
            let semicolon_colored = match self.qualify_semicolon_color {
                Some(color) => ":".color(color),
                None => ColoredString::from(":"),
            };
            format!("{}{}{}", prefix_colored, semicolon_colored, rest_colored)
        } else {
            format!("<{iri}>")
        }
    }
}

impl fmt::Display for PrefixMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (alias, iri) in self.map.iter() {
            writeln!(f, "{} <{}>", &alias, &iri)?
        }
        Ok(())
    }
}

/*impl Default for PrefixMap {
    fn default() -> PrefixMap {
        PrefixMap {
            map: IndexMap::new(),
        }
    }
}*/

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

    #[test]
    fn qualify() {
        let mut pm = PrefixMap::new();
        pm.insert("", &IriS::from_str("http://example.org/").unwrap());
        pm.insert(
            "shapes",
            &IriS::from_str("http://example.org/shapes/").unwrap(),
        );
        assert_eq!(
            pm.qualify(&IriS::from_str("http://example.org/alice").unwrap()),
            ":alice"
        );
        assert_eq!(
            pm.qualify(&IriS::from_str("http://example.org/shapes/User").unwrap()),
            "shapes:User"
        );
    }
}
