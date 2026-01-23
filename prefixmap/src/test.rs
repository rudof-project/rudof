#[cfg(test)]
mod tests1 {
    use crate::PrefixMap;
    use iri_s::IriS;
    use std::str::FromStr;

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

#[cfg(test)]
mod tests2 {
    use crate::{PrefixMap, PrefixMapError};
    use iri_s::IriS;
    use std::str::FromStr;

    #[test]
    fn it_works() -> Result<(), PrefixMapError> {
        let mut pm = PrefixMap::new();
        let schema_iri = IriS::from_str("http://schema.org/")?;
        pm.add_prefix("schema", schema_iri)?;
        let resolved = pm.resolve("schema:knows")?;
        let schema_knows = IriS::from_str("http://schema.org/knows")?;
        assert_eq!(resolved, schema_knows);
        Ok(())
    }
}
