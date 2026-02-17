#![allow(unused_imports, dead_code)]

use crate::PrefixMap;
use iri_s::IriS;
use proptest::prelude::*;
use std::fmt::Write;
use std::str::FromStr;

const PREFIX_REGEX: &str = r"[a-zA-Z]{1,3}";
const LOCAL_REGEX: &str = r"[a-zA-Z]+";
const URI_REGEX: &str = r"https?://[a-zA-Z0-9]{3,}(\.[a-zA-Z0-9]{3,})+(\/[a-zA-Z0-9/]{1,3})*\/";

mod prefixmap_tests {
    use super::*;

    proptest! {
        #[test]
        fn prefix_map_add(prefix in PREFIX_REGEX, uri in URI_REGEX, local in LOCAL_REGEX) {
            let mut pm = PrefixMap::new();
            let binding = IriS::from_str(&uri)?;
            pm.add_prefix(prefix.clone(), binding)?;
            let expected = IriS::from_str(&format!("{uri}{local}"))?;
            assert_eq!(pm.resolve(&format!("{prefix}:{local}"))?, expected);

        }

        #[test]
        fn prefixmap_display(
            prefixes in proptest::collection::hash_set(PREFIX_REGEX, 5),
            uris in proptest::collection::vec(URI_REGEX, 5)
        ) {
            let mut pm = PrefixMap::new();
            let mut expected = String::new();
            let prefix_vec: Vec<String> = prefixes.iter().cloned().collect();

            for i in 0..5 {
                let iri = IriS::from_str(&uris[i])?;
                pm.add_prefix(&prefix_vec[i], iri)?;
                writeln!(expected, "prefix {}: <{}>", &prefix_vec[i], &uris[i]).unwrap();
            }

            assert_eq!(pm.to_string(), expected);
        }

        #[test]
        fn prefixmap_resolve(prefix in PREFIX_REGEX, uri in URI_REGEX, local in LOCAL_REGEX) {
            let mut pm = PrefixMap::new();
            let ex_iri = IriS::from_str(&uri)?;
            pm.add_prefix(prefix.clone(), ex_iri)?;
            assert_eq!(
                pm.resolve(&format!("{prefix}:{local}"))?,
                IriS::from_str(&format!("{uri}{local}"))?
            );
        }

        #[test]
        fn qualify(
            prefix in proptest::collection::hash_set(PREFIX_REGEX, 5),
            uri in proptest::collection::vec(URI_REGEX, 5),
            local in proptest::collection::vec(LOCAL_REGEX, 5)
        ) {
            let mut pm = PrefixMap::new();
            let prefix_vec: Vec<String> = prefix.iter().cloned().collect();

            for i in 0..5 {
                pm.add_prefix(&prefix_vec[i], IriS::from_str(&uri[i])?)?;
            }
            for i in 0..5 {
                assert_eq!(
                    pm.qualify(&IriS::from_str(&format!("{}{}", uri[i], local[i]))?),
                    format!("{}:{}", prefix_vec[i], local[i])
                )
            }
        }
    }
}
