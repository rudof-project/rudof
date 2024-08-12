use colored::*;
use indexmap::map::Iter;
use indexmap::IndexMap;
use iri_s::*;
// use serde::{Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

use crate::{IriRef, PrefixMapError};
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

    #[serde(skip)]
    hyperlink: bool,
}

fn split(str: &str) -> Option<(&str, &str)> {
    str.rsplit_once(':')
}

impl PrefixMap {
    /// Creates an empty ("map
    pub fn new() -> PrefixMap {
        PrefixMap::default()
    }

    /// Change ("color when qualifying a IRI
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

    pub fn without_rich_qualifying(self) -> Self {
        self.with_hyperlink(false)
            .with_qualify_localname_color(None)
            .with_qualify_prefix_color(None)
            .with_qualify_semicolon_color(None)
    }

    /// Inserts an alias association to an IRI
    pub fn insert(&mut self, alias: &str, iri: &IriS) {
        self.map.insert(alias.to_owned(), iri.clone());
    }

    pub fn find(&self, str: &str) -> Option<&IriS> {
        self.map.get(str)
    }

    pub fn from_hashmap(hm: &HashMap<&str, &str>) -> Result<PrefixMap, IriSError> {
        let mut pm = PrefixMap::new();
        for (a, s) in hm.iter() {
            let iri = IriS::from_str(s)?;
            pm.insert(a, &iri);
        }
        Ok(pm)
    }

    /// Return an iterator over the key-value pairs of the ("map, in their order
    pub fn iter(&self) -> Iter<String, IriS> {
        self.map.iter()
    }

    /// Resolves a string against a prefix map
    /// Example:
    /// Given a string like "ex:a" and a prefixmap that has alias "ex" with value "http://example.org/", the result will be "http://example.org/a"
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
    ///     ("", "http://example.org/"),
    ///     ("schema", "http://schema.org/")])
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

    /// Resolves an IriRef against a prefix map
    pub fn resolve_iriref(&self, iri_ref: &IriRef) -> Result<IriS, PrefixMapError> {
        match iri_ref {
            IriRef::Prefixed { prefix, local } => {
                let iri = self.resolve_prefix_local(prefix, local)?;
                Ok(iri)
            }
            IriRef::Iri(iri) => Ok(iri.clone()),
        }
    }

    /// Resolves a prefixed alias and a local name in a prefix map to obtain the full IRI
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
    ///     ("", "http://example.org/"),
    ///     ("schema", "http://schema.org/"),
    ///     ("xsd", "http://www.w3.org/2001/XMLSchema#")
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
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("", "http://example.org/"),
    ///     ("schema", "http://schema.org/")])
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
        let str = if let Some((alias, rest)) = founds.first() {
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
        };
        if self.hyperlink {
            format!(
                "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                iri.as_str(),
                str
            )
        } else {
            str
        }
    }

    /// Qualify an IRI against a prefix map and obtains the local name
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   &HashMap::from([
    ///     ("", "http://example.org/"),
    ///     ("schema", "http://schema.org/")])
    /// )?;
    /// let a = IriS::from_str("http://example.org/a")?;
    /// assert_eq!(pm.qualify_local(&a), Some("a".to_string()));
    ///
    /// let knows = IriS::from_str("http://schema.org/knows")?;
    /// assert_eq!(pm.qualify_local(&knows), Some("knows".to_string()));
    ///
    /// let other = IriS::from_str("http://other.org/foo")?;
    /// assert_eq!(pm.qualify_local(&other), None);
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify_local(&self, iri: &IriS) -> Option<String> {
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
        if let Some((_alias, rest)) = founds.first() {
            Some(rest.to_string())
        } else {
            None
        }
    }

    /// Basic prefixmap with common definitions
    pub fn basic() -> PrefixMap {
        PrefixMap::from_hashmap(&HashMap::from([
            ("", "http://example.org/"),
            ("xsd", "http://www.w3.org/2001/XMLSchema#"),
            ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
            ("dc", "http://purl.org/dc/elements/1.1/"),
        ]))
        .unwrap()
        // .without_default_colors()
        // .with_hyperlink(true)
    }

    /// Default Wikidata prefixmap
    /// This source of this list is <https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Full_list_of_prefixes>
    pub fn wikidata() -> PrefixMap {
        PrefixMap::from_hashmap(&HashMap::from([
            ("bd", "http://www.bigdata.com/rdf#"),
            ("cc", "http://creativecommons.org/ns#"),
            ("dct", "http://purl.org/dc/terms/"),
            ("geo", "http://www.opengis.net/ont/geosparql#"),
            ("hint", "http://www.bigdata.com/queryHints#"),
            ("ontolex", "http://www.w3.org/ns/lemon/ontolex#"),
            ("owl", "http://www.w3.org/2002/07/owl#"),
            ("prov", "http://www.w3.org/ns/prov#"),
            ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
            ("schema", "http://schema.org/"),
            ("skos", "http://www.w3.org/2004/02/skos/core#"),
            ("xsd", "http://www.w3.org/2001/XMLSchema#"),
            ("p", "http://www.wikidata.org/prop/"),
            ("pq", "http://www.wikidata.org/prop/qualifier/"),
            (
                "pqn",
                "http://www.wikidata.org/prop/qualifier/value-normalized/",
            ),
            ("pqv", "http://www.wikidata.org/prop/qualifier/value/"),
            ("pr", "http://www.wikidata.org/prop/reference/"),
            (
                "prn",
                "http://www.wikidata.org/prop/reference/value-normalized/",
            ),
            ("prv", "http://www.wikidata.org/prop/reference/value/"),
            ("psv", "http://www.wikidata.org/prop/statement/value/"),
            ("ps", "http://www.wikidata.org/prop/statement/"),
            (
                "psn",
                "http://www.wikidata.org/prop/statement/value-normalized/",
            ),
            ("wd", "http://www.wikidata.org/entity/"),
            ("wdata", "http://www.wikidata.org/wiki/Special:EntityData/"),
            ("wdno", "http://www.wikidata.org/prop/novalue/"),
            ("wdref", "http://www.wikidata.org/reference/"),
            ("wds", "http://www.wikidata.org/entity/statement/"),
            ("wdt", "http://www.wikidata.org/prop/direct/"),
            ("wdtn", "http://www.wikidata.org/prop/direct-normalized/"),
            ("wdv", "http://www.wikidata.org/value/"),
            ("wikibase", "http://wikiba.se/ontology#"),
        ]))
        .unwrap()
        .without_default_colors()
        .with_hyperlink(true)
    }

    pub fn without_colors(mut self) -> Self {
        self.qualify_localname_color = None;
        self.qualify_prefix_color = None;
        self.qualify_semicolon_color = None;
        self
    }

    pub fn without_default_colors(mut self) -> Self {
        self.qualify_localname_color = Some(Color::Black);
        self.qualify_prefix_color = Some(Color::Blue);
        self.qualify_semicolon_color = Some(Color::Red);
        self
    }

    pub fn with_hyperlink(mut self, hyperlink: bool) -> Self {
        self.hyperlink = hyperlink;
        self
    }
}

impl fmt::Display for PrefixMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (alias, iri) in self.map.iter() {
            writeln!(f, "prefix {}: <{}>", &alias, &iri)?
        }
        Ok(())
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
            "prefix ex: <http://example.org/>\nprefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n"
        );
    }

    #[test]
    fn prefixmap_resolve() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("http://example.org/").unwrap();
        pm.insert("ex", &ex_iri);
        assert_eq!(
            pm.resolve("ex:pepe").unwrap(),
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
