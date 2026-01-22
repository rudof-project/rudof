use crate::{IriRef, PrefixMapError};
use colored::*;
use indexmap::IndexMap;
use iri_s::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{collections::HashMap, fmt};

/// Contains declarations of prefix maps which are used in TURTLE, SPARQL and ShEx
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct PrefixMap {
    /// Proper prefix map associations of an alias `String` to an `IriS`
    pub map: IndexMap<String, IriS>,

    /// Color of prefix aliases when qualifying an IRI that has an alias
    #[serde(skip)]
    qualify_prefix_color: Option<Color>,

    /// Color of local names when qualifying an IRI that has an alias
    #[serde(skip)]
    qualify_localname_color: Option<Color>,

    /// Color of semicolon when qualifying an IRI that has an alias
    #[serde(skip)]
    qualify_semicolon_color: Option<Color>,

    /// Whether to generate hyperlink when qualifying an IRI
    #[serde(skip)]
    hyperlink: bool,
}

impl PrefixMap {
    /// Creates an empty map
    pub fn new() -> PrefixMap {
        PrefixMap::default()
    }

    /// Returns the number of prefix associations in the prefixmap
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the prefixmap is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Change color when qualifying a IRI
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

    /// Disable all rich qualifying (colors and hyperlinks)
    pub fn without_rich_qualifying(self) -> Self {
        self.with_hyperlink(false)
            .without_colors()
    }

    /// Inserts an alias association to an IRI
    pub fn add_prefix<A, I>(&mut self, alias: A, iri: I) -> Result<(), PrefixMapError>
    where
        A: AsRef<str>,
        I: Into<IriS>,
    {
        let key = alias.as_ref();
        if self.map.contains_key(key) {
            return Err(PrefixMapError::AliasAlreadyExists {
                prefix: key.to_string(),
                value: self.map.get(key).unwrap().to_string()
            })
        }
        self.map.insert(key.to_string(), iri.into());
        Ok(())
    }

    /// Finds an IRI associated with an alias
    pub fn find(&self, str: &str) -> Option<&IriS> {
        self.map.get(str)
    }

    /// Creates a prefix map from a hashmap of &str to &str
    pub fn from_hashmap(hm: HashMap<&str, &str>) -> Result<PrefixMap, PrefixMapError> {
        let mut pm = PrefixMap::new();
        for (a, s) in hm.iter() {
            let iri = IriS::from_str(s)?;
            pm.add_prefix(a, iri)?;
        }
        Ok(pm)
    }

    /// Resolves a string against a prefix map
    /// Example:
    /// Given a string like "ex:a" and a prefixmap that has alias "ex" with value "https://example.org/", the result will be "https://example.org/a"
    /// ```
    /// use std::collections::HashMap;
    /// use prefixmap::PrefixMap;
    /// use prefixmap::PrefixMapError;
    /// use iri_s::*;
    /// use std::str::FromStr;
    ///
    ///
    /// let pm: PrefixMap = PrefixMap::from_hashmap(
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// )?;
    /// let a = pm.resolve(":a")?;
    /// let a_resolved = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    /// Ok::<(), PrefixMapError>(());
    ///
    /// let knows = pm.resolve("schema:knows")?;
    /// let knows_resolved = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    /// Ok::<(), PrefixMapError>(())
    /// ```
    pub fn resolve(&self, str: &str) -> Result<IriS, PrefixMapError> {
        match str.rsplit_once(':') {
            Some((prefix, local)) => Ok(self.resolve_prefix_local(prefix, local)?),
            None => Ok(IriS::from_str(str)?),
        }
    }

    /// Resolves an IriRef against a prefix map
    pub fn resolve_iriref(&self, iri_ref: IriRef) -> Result<IriS, PrefixMapError> {
        match iri_ref {
            IriRef::Prefixed { prefix, local } => {
                Ok(self.resolve_prefix_local(prefix, local)?)
            },
            IriRef::Iri(iri) => Ok(iri),
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
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/"),
    ///     ("xsd", "https://www.w3.org/2001/XMLSchema#")
    /// ]))?;
    ///
    /// let a = pm.resolve_prefix_local("", "a")?;
    /// let a_resolved = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    ///
    /// let knows = pm.resolve_prefix_local("schema","knows")?;
    /// let knows_resolved = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    ///
    /// let xsd_string = pm.resolve_prefix_local("xsd","string")?;
    /// let xsd_string_resolved = IriS::from_str("https://www.w3.org/2001/XMLSchema#string")?;
    /// assert_eq!(xsd_string, xsd_string_resolved);
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn resolve_prefix_local<S: Into<String>>(&self, prefix: S, local: S) -> Result<IriS, PrefixMapError> {
        let prefix = prefix.into();
        let local = local.into();

        match self.find(prefix.as_str()) {
            Some(iri) => {
                let new_iri = iri.extend(local.as_str())?;
                Ok(new_iri)
            }
            None => Err(PrefixMapError::PrefixNotFound {
                prefix,
                prefixmap: self.clone(),
            }),
        }
    }

    /// Qualifies an IRI against a prefix map
    ///
    /// If it can't qualify the IRI, it returns the iri between `<` and `>`
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// )?;
    /// let a = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(pm.qualify(&a), ":a");
    ///
    /// let knows = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(pm.qualify(&knows), "schema:knows");
    ///
    /// let other = IriS::from_str("https://other.org/foo")?;
    /// assert_eq!(pm.qualify(&other), "<https://other.org/foo>");
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify(&self, iri: &IriS) -> String {
        self.qualify_optional(iri).unwrap_or_else(|| format!("<{iri}>"))
    }

    /// Qualifies an IRI against a prefix map
    ///
    /// If it can't qualify the IRI, returns None
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// )?;
    /// let a = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(pm.qualify_optional(&a), Some(":a".to_string()));
    ///
    /// let knows = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(pm.qualify_optional(&knows), Some("schema:knows".to_string()));
    ///
    /// let other = IriS::from_str("https://other.org/foo")?;
    /// assert_eq!(pm.qualify_optional(&other), None);
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify_optional(&self, iri: &IriS) -> Option<String> {
        let (alias, rest) = self.longest_prefix_match(iri)?;
        let s = self.format_colored(alias, rest);

        if self.hyperlink {
            Some(format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", iri.as_str(), s))
        } else {
            Some(s)
        }
    }

    /// Qualifies an IRI against a prefix map returning the length of the qualified string
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::PrefixMapError;
    /// # use iri_s::*;
    /// # use std::str::FromStr;
    /// let pm = PrefixMap::from_hashmap(
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// )?;
    /// let a = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(pm.qualify_and_length(&a), (":a".to_string(), 2));
    ///
    /// let knows = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(pm.qualify_and_length(&knows), ("schema:knows".to_string(),12));
    ///
    /// let other = IriS::from_str("https://other.org/foo")?;
    /// assert_eq!(pm.qualify_and_length(&other), ("<https://other.org/foo>".to_string(), 23));
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify_and_length(&self, iri: &IriS) -> (String, usize) {
        let (s, length) = if let Some((alias, rest)) = self.longest_prefix_match(iri) {
            let s = self.format_colored(alias, rest);
            let length = alias.len() + 1 + rest.len();
            (s, length)
        } else {
            let s = format!("<{iri}>");
            let length = iri.as_str().len() + 2;
            (s, length)
        };

        if self.hyperlink {
            let s_hyperlink =
                format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", iri.as_str(), s);
            (s_hyperlink, length)
        } else {
            (s, length)
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
    ///   HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// )?;
    /// let a = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(pm.qualify_local(&a), Some("a".to_string()));
    ///
    /// let knows = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(pm.qualify_local(&knows), Some("knows".to_string()));
    ///
    /// let other = IriS::from_str("https://other.org/foo")?;
    /// assert_eq!(pm.qualify_local(&other), None);
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn qualify_local(&self, iri: &IriS) -> Option<String> {
        self.longest_prefix_match(iri)
            .map(|(_, rest)| rest.to_string())
    }

    fn longest_prefix_match<'a>(&'a self, iri: &'a IriS) -> Option<(&'a str, &'a str)> {
        self.map.iter()
            .filter_map(|(alias, pm_iri)| {
                iri
                    .as_str()
                    .strip_prefix(pm_iri.as_str())
                    .map(|rest| (alias.as_str(), rest))
            })
            .max_by_key(|(_, rest)| iri.as_str().len() - rest.len())
    }

    fn alias_color(&self, alias: &str) -> ColoredString {
        match self.qualify_prefix_color {
            Some(color) => alias.color(color),
            None => ColoredString::from(alias)
        }
    }

    fn local_color(&self, rest: &str) -> ColoredString {
        match self.qualify_localname_color {
            Some(color) => rest.color(color),
            None => ColoredString::from(rest)
        }
    }

    fn semicolon_color(&self) -> ColoredString {
        match self.qualify_semicolon_color {
            Some(color) => ":".color(color),
            None => ColoredString::from(":")
        }
    }

    fn format_colored(&self, alias: &str, rest: &str) -> String {
        let prefix_colored = self.alias_color(alias);
        let rest_colored = self.local_color(rest);
        let semicolon_colored = self.semicolon_color();

        format!("{prefix_colored}{semicolon_colored}{rest_colored}")
    }

    /// Basic prefixmap with common definitions
    pub fn basic() -> PrefixMap {
        PrefixMap::from_hashmap(HashMap::from([
            ("", "https://example.org/"),
            ("dc", "https://purl.org/dc/elements/1.1/"),
            ("rdf", "https://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "https://www.w3.org/2000/01/rdf-schema#"),
            ("sh", "https://www.w3.org/ns/shacl#"),
            ("xsd", "https://www.w3.org/2001/XMLSchema#"),
        ]))
        .unwrap()
    }

    /// Default Wikidata prefixmap
    /// This source of this list is <https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Full_list_of_prefixes>
    pub fn wikidata() -> PrefixMap {
        PrefixMap::from_hashmap(HashMap::from([
            ("bd", "https://www.bigdata.com/rdf#"),
            ("cc", "https://creativecommons.org/ns#"),
            ("dct", "https://purl.org/dc/terms/"),
            ("geo", "https://www.opengis.net/ont/geosparql#"),
            ("hint", "https://www.bigdata.com/queryHints#"),
            ("ontolex", "https://www.w3.org/ns/lemon/ontolex#"),
            ("owl", "https://www.w3.org/2002/07/owl#"),
            ("prov", "https://www.w3.org/ns/prov#"),
            ("rdf", "https://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "https://www.w3.org/2000/01/rdf-schema#"),
            ("schema", "https://schema.org/"),
            ("skos", "https://www.w3.org/2004/02/skos/core#"),
            ("xsd", "https://www.w3.org/2001/XMLSchema#"),
            ("p", "https://www.wikidata.org/prop/"),
            ("pq", "https://www.wikidata.org/prop/qualifier/"),
            (
                "pqn",
                "https://www.wikidata.org/prop/qualifier/value-normalized/",
            ),
            ("pqv", "https://www.wikidata.org/prop/qualifier/value/"),
            ("pr", "https://www.wikidata.org/prop/reference/"),
            (
                "prn",
                "https://www.wikidata.org/prop/reference/value-normalized/",
            ),
            ("prv", "https://www.wikidata.org/prop/reference/value/"),
            ("psv", "https://www.wikidata.org/prop/statement/value/"),
            ("ps", "https://www.wikidata.org/prop/statement/"),
            (
                "psn",
                "https://www.wikidata.org/prop/statement/value-normalized/",
            ),
            ("wd", "https://www.wikidata.org/entity/"),
            ("wdata", "https://www.wikidata.org/wiki/Special:EntityData/"),
            ("wdno", "https://www.wikidata.org/prop/novalue/"),
            ("wdref", "https://www.wikidata.org/reference/"),
            ("wds", "https://www.wikidata.org/entity/statement/"),
            ("wdt", "https://www.wikidata.org/prop/direct/"),
            ("wdtn", "https://www.wikidata.org/prop/direct-normalized/"),
            ("wdv", "https://www.wikidata.org/value/"),
            ("wikibase", "https://wikiba.se/ontology#"),
        ]))
        .unwrap()
        .without_default_colors()
        .with_hyperlink(true)
    }

    pub fn without_colors(self) -> Self {
        self.with_qualify_prefix_color(None)
            .with_qualify_localname_color(None)
            .with_qualify_semicolon_color(None)
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

    pub fn merge(&mut self, other: PrefixMap) -> Result<(), PrefixMapError> {
        for (alias, iri) in other.into_iter() {
            self.add_prefix(alias, iri)?
        }
        Ok(())
    }

    pub fn aliases(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
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

impl Iterator for PrefixMap {
    type Item = (String, IriS);

    fn next(&mut self) -> Option<Self::Item> {
        match self.map.is_empty() {
            true => None,
            false => {
                let (k, v) = self.map.shift_remove_index(0).unwrap();
                Some((k, v))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_ex_name() {
        assert_eq!("ex:name".rsplit_once(':'), Some(("ex", "name")))
    }

    #[test]
    fn prefix_map1() {
        let mut pm = PrefixMap::new();
        let binding = IriS::from_str("https://example.org/").unwrap();
        pm.add_prefix("ex", binding).unwrap();
        let expected = IriS::from_str("https://example.org/name").unwrap();
        assert_eq!(pm.resolve("ex:name").unwrap(), expected);
    }

    #[test]
    fn prefixmap_display() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("https://example.org/").unwrap();
        pm.add_prefix("ex", ex_iri).unwrap();
        let ex_rdf = IriS::from_str("https://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
        pm.add_prefix("rdf", ex_rdf).unwrap();
        assert_eq!(
            pm.to_string(),
            "prefix ex: <https://example.org/>\nprefix rdf: <https://www.w3.org/1999/02/22-rdf-syntax-ns#>\n"
        );
    }

    #[test]
    fn prefixmap_resolve() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("https://example.org/").unwrap();
        pm.add_prefix("ex", ex_iri).unwrap();
        assert_eq!(
            pm.resolve("ex:pepe").unwrap(),
            IriS::from_str("https://example.org/pepe").unwrap()
        );
    }

    #[test]
    fn prefixmap_resolve_xsd() {
        let mut pm = PrefixMap::new();
        let ex_iri = IriS::from_str("https://www.w3.org/2001/XMLSchema#").unwrap();
        pm.add_prefix("xsd", ex_iri).unwrap();
        assert_eq!(
            pm.resolve_prefix_local("xsd", "string").unwrap(),
            IriS::from_str("https://www.w3.org/2001/XMLSchema#string").unwrap()
        );
    }

    #[test]
    fn qualify() {
        let mut pm = PrefixMap::new();
        pm.add_prefix("", IriS::from_str("https://example.org/").unwrap())
            .unwrap();
        pm.add_prefix(
            "shapes",
            IriS::from_str("https://example.org/shapes/").unwrap(),
        )
        .unwrap();
        assert_eq!(
            pm.qualify(&IriS::from_str("https://example.org/alice").unwrap()),
            ":alice"
        );
        assert_eq!(
            pm.qualify(&IriS::from_str("https://example.org/shapes/User").unwrap()),
            "shapes:User"
        );
    }
}
