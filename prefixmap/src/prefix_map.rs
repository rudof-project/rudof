use crate::error::PrefixMapError;
use crate::{IriRef, Show};
use colored::*;
use indexmap::IndexMap;
use rudof_iri::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::MapAccess, de::Visitor, ser::SerializeMap};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::{collections::HashMap, fmt};

/// Contains declarations of prefix maps which are used in TURTLE, SPARQL and ShEx
#[derive(Debug, Clone, Eq, Default)]
pub struct PrefixMap {
    /// Proper prefix map associations of an alias [`String`] to an [`IriS`]
    pub map: IndexMap<String, IriS>,

    // TODO - The following properties should be handled by rudof_lib
    /// Color of prefix aliases when qualifying an IRI that has an alias
    qualify_prefix_color: Option<Color>,

    /// Color of local names when qualifying an IRI that has an alias
    qualify_localname_color: Option<Color>,

    /// Color of semicolon when qualifying an IRI that has an alias
    qualify_semicolon_color: Option<Color>,

    /// Whether to generate hyperlink when qualifying an IRI
    hyperlink: bool,

    /// Base IRI used to shorten IRIs that don't match any prefix alias but
    /// share this IRI as a prefix, e.g. if the base is `http://example.org/`
    /// then `http://example.org/foo` qualifies as `<foo>`.
    base: Option<IriS>,
}

impl PartialEq for PrefixMap {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl Hash for PrefixMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 1. Extraemos las referencias de los elementos
        let mut inputs: Vec<(&String, &IriS)> = self.map.iter().collect();

        // 2. Las ordenamos por clave para garantizar que el orden de inserción no afecte al hash
        inputs.sort_by_key(|&(k, _v)| k);

        // 3. Pasamos los elementos ya ordenados al hasher
        for (clave, valor) in inputs {
            clave.hash(state);
            valor.hash(state);
        }
    }
}

impl Serialize for PrefixMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for PrefixMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PrefixMapVisitor;

        impl<'de> Visitor<'de> for PrefixMapVisitor {
            type Value = PrefixMap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of prefix to IRI")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut map = IndexMap::new();
                while let Some((k, v)) = access.next_entry::<String, IriS>()? {
                    map.insert(k, v);
                }
                Ok(PrefixMap {
                    map,
                    ..Default::default()
                })
            }
        }

        deserializer.deserialize_map(PrefixMapVisitor)
    }
}

/// Methods for [`PrefixMap`] manipulation
impl PrefixMap {
    /// Creates an empty map
    pub fn new() -> PrefixMap {
        PrefixMap::default()
    }

    /// Returns the number of prefix associations in the [`PrefixMap`]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the [`PrefixMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Inserts an alias association to an IRI
    ///
    // Returns an [`PrefixMapError`] if the alias already exists.
    pub fn add_prefix<A, I>(&mut self, alias: A, iri: I)
    where
        A: AsRef<str>,
        I: Into<IriS>,
    {
        let key = alias.as_ref();
        // if self.map.contains_key(key) {
        //     return Err(PrefixMapError::AliasAlreadyExists {
        //         prefix: key.to_string(),
        //         value: self.map.get(key).unwrap().to_string(),
        //     });
        // }
        self.map.insert(key.to_string(), iri.into());
    }

    /// Finds an IRI associated with a given alias
    pub fn find(&self, str: &str) -> Option<&IriS> {
        self.map.get(str)
    }

    /// Removes the association for `alias`, returning its [`IriS`] if it existed
    pub fn remove_prefix(&mut self, alias: &str) -> Option<IriS> {
        self.map.shift_remove(alias)
    }

    /// Merges another [`PrefixMap`] into this one.
    ///
    // Returns an error if any of the aliases in the other [`PrefixMap`] already exist in this one.
    pub fn merge(&mut self, other: PrefixMap) {
        for (alias, iri) in other.into_iter() {
            self.add_prefix(alias, iri)
        }
    }

    /// Returns an iterator over the aliases in the [`PrefixMap`]
    pub fn aliases(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &IriS)> {
        self.map.iter()
    }
}

// TODO - Probably should be a good idea to move this to rudof_lib
/// Formatting for [`PrefixMap`] outputs
impl PrefixMap {
    /// Disable all colors when qualifying IRIs
    pub fn without_colors(self) -> Self {
        self.with_qualify_prefix_color(None)
            .with_qualify_localname_color(None)
            .with_qualify_semicolon_color(None)
    }

    /// Use default colors when qualifying IRIs
    pub fn without_default_colors(mut self) -> Self {
        self.qualify_localname_color = Some(Color::Black);
        self.qualify_prefix_color = Some(Color::Blue);
        self.qualify_semicolon_color = Some(Color::Red);
        self
    }

    /// Enable or disable hyperlinking when qualifying IRIs
    pub fn with_hyperlink(mut self, hyperlink: bool) -> Self {
        self.hyperlink = hyperlink;
        self
    }

    /// Sets the base IRI used to shorten IRIs that don't match any prefix alias
    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    /// Sets the base IRI used to shorten IRIs that don't match any prefix alias
    pub fn set_base(&mut self, base: Option<IriS>) {
        self.base = base;
    }

    /// Returns the base IRI used to shorten IRIs that don't match any prefix alias, if any
    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }

    /// Color the alias when qualifying an IRI
    fn alias_color(&self, alias: &str) -> ColoredString {
        match self.qualify_prefix_color {
            Some(color) => alias.color(color),
            None => ColoredString::from(alias),
        }
    }

    /// Color the local name when qualifying an IRI
    fn local_color(&self, rest: &str) -> ColoredString {
        match self.qualify_localname_color {
            Some(color) => rest.color(color),
            None => ColoredString::from(rest),
        }
    }

    /// Color the semicolon when qualifying an IRI
    fn semicolon_color(&self) -> ColoredString {
        match self.qualify_semicolon_color {
            Some(color) => ":".color(color),
            None => ColoredString::from(":"),
        }
    }

    /// Format a qualified IRI with colors
    fn format_colored(&self, alias: &str, rest: &str) -> String {
        let prefix_colored = self.alias_color(alias);
        let rest_colored = self.local_color(rest);
        let semicolon_colored = self.semicolon_color();

        format!("{prefix_colored}{semicolon_colored}{rest_colored}")
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
        self.with_hyperlink(false).without_colors()
    }

    pub fn show<S: Show>(&self, item: &S) -> String {
        item.show(self)
    }
}

/// Common predefined prefix maps
impl PrefixMap {
    /// Basic prefixmap with common definitions.
    /// This includes:
    /// - `default`
    /// - `dc`
    /// - `rdf`
    /// - `rdfs`
    /// - `sh`
    /// - `xsd`
    pub fn basic() -> PrefixMap {
        HashMap::from([
            ("", "http://example.org/"),
            ("dc", "http://purl.org/dc/elements/1.1/"),
            ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
            ("sh", "http://www.w3.org/ns/shacl#"),
            ("xsd", "http://www.w3.org/2001/XMLSchema#"),
        ])
        .try_into()
        .unwrap()
    }

    /// Default Wikidata prefix map
    /// This source of this list is <https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Full_list_of_prefixes>
    pub fn wikidata() -> PrefixMap {
        let pm: PrefixMap = HashMap::from([
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
            ("pqn", "http://www.wikidata.org/prop/qualifier/value-normalized/"),
            ("pqv", "http://www.wikidata.org/prop/qualifier/value/"),
            ("pr", "http://www.wikidata.org/prop/reference/"),
            ("prn", "http://www.wikidata.org/prop/reference/value-normalized/"),
            ("prv", "http://www.wikidata.org/prop/reference/value/"),
            ("psv", "http://www.wikidata.org/prop/statement/value/"),
            ("ps", "http://www.wikidata.org/prop/statement/"),
            ("psn", "http://www.wikidata.org/prop/statement/value-normalized/"),
            ("wd", "http://www.wikidata.org/entity/"),
            ("wdata", "http://www.wikidata.org/wiki/Special:EntityData/"),
            ("wdno", "http://www.wikidata.org/prop/novalue/"),
            ("wdref", "http://www.wikidata.org/reference/"),
            ("wds", "http://www.wikidata.org/entity/statement/"),
            ("wdt", "http://www.wikidata.org/prop/direct/"),
            ("wdtn", "http://www.wikidata.org/prop/direct-normalized/"),
            ("wdv", "http://www.wikidata.org/value/"),
            ("wikibase", "http://wikiba.se/ontology#"),
            ("es", "https://www.wikidata.org/wiki/Special:EntitySchemaText/"),
        ])
        .try_into()
        .unwrap();
        pm.without_default_colors().with_hyperlink(true)
    }

    /// Default DBpedia prefix map
    pub fn dbpedia() -> PrefixMap {
        let pm: PrefixMap = HashMap::from([
            ("dbc", "http://dbpedia.org/class/"),
            ("dbo", "http://dbpedia.org/ontology/"),
            ("dbp", "http://dbpedia.org/property/"),
            ("dbr", "http://dbpedia.org/resource/"),
            ("foaf", "http://xmlns.com/foaf/0.1/"),
            ("geo", "http://www.w3.org/2003/01/geo/wgs84_pos#"),
            ("xsd", "http://www.w3.org/2001/XMLSchema#"),
        ])
        .try_into()
        .unwrap();
        pm.without_default_colors().with_hyperlink(true)
    }

    /// Default Uniprot prefix map
    pub fn uniprot() -> PrefixMap {
        let pm: PrefixMap = HashMap::from([
            ("formats", "http://www.w3.org/ns/formats/"),
            ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
            ("sd", "http://www.w3.org/ns/sparql-service-description#"),
            ("taxon", "http://purl.uniprot.org/taxonomy/"),
            ("up", "http://purl.uniprot.org/core/"),
            ("void", "https://sparql.uniprot.org/.well-known/void#"),
        ])
        .try_into()
        .unwrap();
        pm.without_default_colors().with_hyperlink(true)
    }
}

/// Qualifying IRIs against a [`PrefixMap`]
impl PrefixMap {
    /// Qualifies an IRI against a [`PrefixMap`]
    ///
    /// If it can't qualify the IRI, it returns the iri between `<` and `>`
    /// (relative to the [`PrefixMap`]'s base IRI, if one is set and matches)
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// .try_into()?;
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
        self.qualify_optional(iri).unwrap_or_else(|| format!("<{}>", iri))
    }

    /// Qualifies an IRI against a [`PrefixMap`]
    ///
    /// If it can't qualify the IRI against a prefix alias, but the [`PrefixMap`]
    /// has a base IRI that is a prefix of `iri`, it returns the base-relative
    /// form between `<` and `>`, e.g. `<foo>` if the base is `http://example.org/`
    /// and `iri` is `http://example.org/foo`.
    ///
    /// Otherwise, returns [`None`]
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// .try_into()?;
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
        let s = if let Some((alias, rest)) = self.longest_prefix_match(iri) {
            self.format_colored(alias, rest)
        } else {
            format!("<{}>", self.base_relative_match(iri)?)
        };

        if self.hyperlink {
            Some(format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", iri.as_str(), s))
        } else {
            Some(s)
        }
    }

    /// Returns the IRI relative to the [`PrefixMap`]'s base IRI, if a base is
    /// set and it is a prefix of `iri`
    fn base_relative_match(&self, iri: &IriS) -> Option<String> {
        let base = self.base.as_ref()?;
        if iri.as_str().starts_with(base.as_str()) {
            Some(iri.relative_from(base))
        } else {
            None
        }
    }

    /// Qualifies an IRI against a [`PrefixMap`], returning the length of the qualified string
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// .try_into()?;
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
        } else if let Some(relative) = self.base_relative_match(iri) {
            let s = format!("<{relative}>");
            let length = relative.len() + 2;
            (s, length)
        } else {
            let s = format!("<{iri}>");
            let length = iri.as_str().len() + 2;
            (s, length)
        };

        if self.hyperlink {
            let s_hyperlink = format!("\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", iri.as_str(), s);
            (s_hyperlink, length)
        } else {
            (s, length)
        }
    }

    /// Qualify an IRI against a [`PrefixMap`] and obtains the local name.
    ///
    /// Returns [`None`] if it can't qualify the IRI.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// .try_into()?;
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
        self.longest_prefix_match(iri).map(|(_, rest)| rest.to_string())
    }

    /// Finds the longest prefix match for a given IRI in the [`PrefixMap`]
    fn longest_prefix_match<'a>(&'a self, iri: &'a IriS) -> Option<(&'a str, &'a str)> {
        self.map
            .iter()
            .filter_map(|(alias, pm_iri)| {
                iri.as_str()
                    .strip_prefix(pm_iri.as_str())
                    .map(|rest| (alias.as_str(), rest))
            })
            .max_by_key(|(_, rest)| iri.as_str().len() - rest.len())
    }
}

/// Resolving strings and IRI references against a [`PrefixMap`]
impl PrefixMap {
    /// Resolves a string against a prefix map
    ///
    /// Returns an error if the prefix is not found in the prefix map or if the `string` is not a valid IRI.
    ///
    /// Example:
    /// Given a string like "ex:a" and a prefixmap that has alias "ex" with value "https://example.org/", the result will be "https://example.org/a"
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    ///
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/")])
    /// .try_into()?;
    ///
    /// let a = pm.resolve(":a")?;
    /// let a_resolved = IriS::from_str("https://example.org/a")?;
    /// assert_eq!(a, a_resolved);
    ///
    /// let knows = pm.resolve("schema:knows")?;
    /// let knows_resolved = IriS::from_str("https://schema.org/knows")?;
    /// assert_eq!(knows, knows_resolved);
    /// # Ok::<(), PrefixMapError>(())
    /// ```
    pub fn resolve(&self, str: &str) -> Result<IriS, PrefixMapError> {
        match str.rsplit_once(':') {
            Some((prefix, local)) => Ok(self.resolve_prefix_local(prefix, local)?),
            None => Ok(IriS::from_str(str)?),
        }
    }

    /// Resolves an [`IriRef`] against a [`PrefixMap`]
    pub fn resolve_iriref(&self, iri_ref: IriRef) -> Result<IriS, PrefixMapError> {
        match iri_ref {
            IriRef::Prefixed { prefix, local } => Ok(self.resolve_prefix_local(prefix, local)?),
            IriRef::Iri(iri) => Ok(iri),
        }
    }

    /// Resolves a prefixed alias and a local name in a prefix map to obtain the full IRI
    ///
    /// Returns an error if:
    /// - the prefix is not found in the prefix map.
    /// - the resulting IRI is invalid.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use prefixmap::PrefixMap;
    /// # use prefixmap::error::PrefixMapError;
    /// # use rudof_iri::*;
    /// # use std::str::FromStr;
    ///
    /// let pm: PrefixMap = HashMap::from([
    ///     ("", "https://example.org/"),
    ///     ("schema", "https://schema.org/"),
    ///     ("xsd", "https://www.w3.org/2001/XMLSchema#")])
    /// .try_into()?;
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
                if local.is_empty() {
                    return Ok(iri.clone());
                }
                let new_iri = iri.extend(local.as_str())?;
                Ok(new_iri)
            },
            None => Err(PrefixMapError::PrefixNotFound {
                prefix,
                prefixmap: Box::new(self.clone()),
            }),
        }
    }
}

impl Display for PrefixMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (alias, iri) in self.map.iter() {
            writeln!(f, "prefix {}: <{}>", alias, iri)?
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
            },
        }
    }
}

impl TryFrom<HashMap<&str, &str>> for PrefixMap {
    type Error = PrefixMapError;

    fn try_from(value: HashMap<&str, &str>) -> Result<Self, Self::Error> {
        let mut pm = PrefixMap::new();
        for (a, s) in value {
            let iri = IriS::from_str(s)?;
            pm.add_prefix(a, iri);
        }
        Ok(pm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualify_with_base() {
        let pm: PrefixMap = HashMap::from([("", "https://example.org/"), ("schema", "https://schema.org/")])
            .try_into()
            .unwrap();

        let a = IriS::new_unchecked("https://example.org/a");
        assert_eq!(pm.qualify(&a), ":a");

        let knows = IriS::new_unchecked("https://schema.org/knows");
        assert_eq!(pm.qualify(&knows), "schema:knows");

        let other = IriS::new_unchecked("https://other.org/foo");
        assert_eq!(pm.qualify(&other), "<https://other.org/foo>");

        let relative = IriS::new_unchecked("relative");
        assert_eq!(pm.qualify(&relative), "<relative>");
    }

    #[test]
    fn test_qualify_relative_to_base() {
        let pm: PrefixMap = HashMap::from([("", "https://example.org/"), ("schema", "https://schema.org/")])
            .try_into()
            .unwrap();
        let pm = pm.with_base(Some(IriS::new_unchecked("http://a.example/")));

        // A prefix alias match still wins over the base
        let a = IriS::new_unchecked("https://example.org/a");
        assert_eq!(pm.qualify(&a), ":a");

        // No alias matches, but the base does: relativize against it
        let observation = IriS::new_unchecked("http://a.example/#Observation");
        assert_eq!(pm.qualify(&observation), "<#Observation>");

        // Neither an alias nor the base matches: full IRI
        let other = IriS::new_unchecked("https://other.org/foo");
        assert_eq!(pm.qualify(&other), "<https://other.org/foo>");
    }
}
