use colored::*;
use indexmap::IndexMap;
use iri_s::*;
use std::str::FromStr;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
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

    pub fn insert(&mut self, alias: &str, iri: IriS) {
        self.map.insert(alias.to_owned(), iri);
    }

    pub fn find(&self, str: &str) -> Option<&IriS> {
        self.map.get(str)
    }

    pub fn from_hashmap(hm: &HashMap<String, String>) -> Result<PrefixMap, IriSError> {
        let mut pm = PrefixMap::new();
        for (a, s) in hm.iter() {
            let iri = IriS::from_str(s)?;
            pm.insert(a, iri);
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
    /// assert_eq!(a, Some(a_resolved));
    ///
    /// let knows = pm.resolve("schema:knows")?;
    /// let knows_resolved = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(knows, Some(knows_resolved));
    /// # Ok::<(), IriSError>(())
    /// ```
    pub fn resolve(&self, str: &str) -> Result<Option<IriS>, IriSError> {
        match split(str) {
            Some((alias, local_name)) => match self.find(alias) {
                Some(iri) => {
                    let new_iri = iri.extend(local_name)?;
                    Ok(Some(new_iri))
                }
                None => {
                    let iri = IriS::from_str(str)?;
                    Ok(Some(iri))
                }
            },
            None => {
                let iri = IriS::from_str(str)?;
                Ok(Some(iri))
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
        pm.insert("ex", binding);
        let expected = IriS::from_str("http://example.org/name").unwrap();
        assert_eq!(pm.resolve("ex:name").unwrap().unwrap(), expected);
    }

    #[test]
    fn prefixmap_display() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", ex_iri);
        let ex_rdf = IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
        pm.insert("rdf", ex_rdf);
        assert_eq!(
            pm.to_string(),
            "ex <http://example.org/>\nrdf <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n"
        );
    }

    #[test]
    fn prefixmap_resolve() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", ex_iri);
        assert_eq!(
            pm.resolve(&"ex:pepe").unwrap(),
            Some(IriS::from_str("http://example.org/pepe").unwrap())
        );
    }
}
