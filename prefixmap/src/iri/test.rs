#![allow(unused_imports, dead_code)]

use crate::PrefixMap;
use crate::iri::IriRef;
use crate::iri::deref::Deref;
use iri_s::IriS;
use proptest::prelude::*;
use std::borrow::Cow;

const PREFIX_REGEX: &str = r"[a-zA-Z]{1,3}";
const LOCAL_REGEX: &str = r"[a-zA-Z]+";
const URI_REGEX: &str = r"https?://[a-zA-Z0-9]{3,}(\.[a-zA-Z0-9]{3,})+(\/[a-zA-Z0-9/]{1,3})*\/";

mod deref_tests {
    use super::*;

    fn iri_iris_ref_gen(prefix: String, local: String, uri: String) -> (IriS, IriRef) {
        let iri = IriS::new(&format!("{uri}{local}")).unwrap();
        let iris_ref = IriRef::Prefixed { prefix, local };
        (iri, iris_ref)
    }

    proptest! {

        #[test]
        fn deref_iri(prefix in PREFIX_REGEX, local in LOCAL_REGEX, uri in URI_REGEX) {
            let pm = PrefixMap::basic();
            let (iri, iris_ref) = iri_iris_ref_gen(prefix.clone(), local, uri);

            let deref_iri = iris_ref.deref(None, Some(&pm));
            if prefix.is_empty() {
                assert_eq!(deref_iri.unwrap(), IriRef::Iri(iri));
            }
        }

        #[test]
        fn deref_option(prefix in PREFIX_REGEX, local in LOCAL_REGEX, uri in URI_REGEX) {
            let pm = PrefixMap::basic();
            let (iri, iris_ref) = iri_iris_ref_gen(prefix.clone(), local, uri);

            let deref_iri = Some(iris_ref).deref(None, Some(&pm));
            if prefix.is_empty() {
                assert_eq!(deref_iri.unwrap(), Some(IriRef::Iri(iri)));
            }
        }

        #[test]
        fn proptest_deref_box(prefix in PREFIX_REGEX, local in LOCAL_REGEX, uri in URI_REGEX) {
            let pm = PrefixMap::basic();
            let (iri, iris_ref) = iri_iris_ref_gen(prefix.clone(), local, uri);

            let deref_iri = Box::new(iris_ref).deref(None, Some(&pm));
            if prefix.is_empty() {
                assert_eq!(deref_iri.unwrap(), Box::new(IriRef::Iri(iri)));
            }
        }

        #[test]
        fn proptest_deref_vec(locals in proptest::collection::vec(LOCAL_REGEX, 1..5)) {
            let pm = PrefixMap::basic();
            let mut iris_ref = Vec::<IriRef>::new();
            let mut result = Vec::<IriRef>::new();

            for local in &locals {
                iris_ref.push(IriRef::Prefixed { prefix: String::from(""), local: local.clone() });
                result.push(IriRef::Iri(IriS::new(&format!("https://example.org/{}", local.clone()))?));
            }

            let deref_iri = iris_ref.deref(None, Some(&pm));
            assert_eq!(deref_iri.unwrap(), result);
        }
    }
}

mod iri_ref_tests {
    use super::*;

    proptest! {
        #[test]
        fn get_iri_returns_iri_for_iri_variant(local in LOCAL_REGEX, uri in URI_REGEX) {
            let iri = IriS::new(&format!("{uri}{local}"))?;
            let iri_ref = IriRef::Iri(iri.clone());
            let result = iri_ref.get_iri()?;
            assert_eq!(result, &iri);
        }

        #[test]
        fn get_iri_returns_error_for_prefixed_variant(prefix in PREFIX_REGEX, local in LOCAL_REGEX) {
            let iri_ref = IriRef::Prefixed { prefix, local };
            let result = iri_ref.get_iri();
            assert!(result.is_err());
        }

        #[test]
        fn get_iri_prefixmap_borrowed_for_iri_variant(local in LOCAL_REGEX, uri in URI_REGEX) {
            let iri = IriS::new(&format!("{uri}{local}"))?;
            let iri_ref = IriRef::Iri(iri.clone());
            let pm = PrefixMap::basic();
            let result = iri_ref.get_iri_prefixmap(&pm)?;

            assert!(matches!(result, Cow::Borrowed(_)));
            assert_eq!(result.as_ref(), &iri);
        }

        #[test]
        fn get_iri_prefixmap_owned_for_resolvable_prefixed(prefix in PREFIX_REGEX, uri in URI_REGEX, local in LOCAL_REGEX) {
            let mut pm = PrefixMap::new();
            pm.add_prefix(prefix.clone(), IriS::new(&uri)?)?;

            let iri_ref = IriRef::Prefixed {
                prefix, local: local.clone(),
            };
            let result = iri_ref.get_iri_prefixmap(&pm).unwrap();
            assert!(matches!(result, Cow::Owned(_)));
            assert_eq!(result.as_ref().as_str(), format!("{uri}{local}"));
        }

        #[test]
        fn get_iri_prefixmap_error_for_unresolvable_prefixed(prefix in PREFIX_REGEX, local in LOCAL_REGEX) {
            let pm = PrefixMap::new();
            let iri_ref = IriRef::Prefixed { prefix, local };
            let err = iri_ref.get_iri_prefixmap(&pm);
            assert!(err.is_err());
        }

        #[test]
        fn prefixed_creates_prefixed_variant(prefix in PREFIX_REGEX, local in LOCAL_REGEX) {
            let iri_ref = IriRef::prefixed(prefix.clone(), local.clone());
            match iri_ref {
                IriRef::Prefixed { prefix: p, local: l } => {
                    assert_eq!(p, prefix);
                    assert_eq!(l, local);
                }
                _ => panic!("Expected Prefixed variant"),
            }
        }

        #[test]
        fn iri_creates_iri_variant(uri in URI_REGEX) {
            let iri = IriS::new(&uri)?;
            let iri_ref = IriRef::iri(iri.clone());
            match iri_ref {
                IriRef::Iri(i) => assert_eq!(i, iri),
                _ => panic!("Expected Iri variant"),
            }
        }
    }
}
