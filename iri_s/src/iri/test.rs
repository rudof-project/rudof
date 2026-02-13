#![allow(unused_imports, dead_code)]

use crate::iri;
use crate::{IriS, iri_once, static_once};
use proptest::prelude::*;
use std::str::FromStr;

const URI_REGEX: &str = r"https?://[a-zA-Z0-9]{3,}(\.[a-zA-Z0-9]{3,})+(\/[a-zA-Z0-9/]{1,3})*\/";
const PATH_REGEX: &str = r"([a-zA-Z0-9_\-]{3,5}/){1,10}";
const FILE_REGEX: &str = concat!("file:///", r"([a-zA-Z0-9_\-]{3,5}/){1,10}");

mod tests {
    use super::*;
    use oxrdf::NamedNode;

    proptest! {
        #[test]
        fn create_iris(uri in URI_REGEX) {
            let iri = IriS::from_str(&uri)?;
            assert_eq!(iri.to_string(), uri)
        }

        #[test]
        fn obtain_iri_as_str(uri in URI_REGEX) {
            let iri = IriS::from_str(&uri)?;
            assert_eq!(iri.as_str(), uri)
        }

        #[test]
        fn extend_iri(base in URI_REGEX, extension in PATH_REGEX) {
            let base = NamedNode::new(base)?;
            let base_iri: IriS = base.into();
            let extended = base_iri.extend(&extension)?;
            assert_eq!(extended.as_str(), format!("{}{}", base_iri.as_str(), extension))
        }

        #[test]
        fn compare_iris(uri in URI_REGEX) {
            let iri1: IriS = NamedNode::new_unchecked(uri.clone()).into();
            let iri2: IriS = NamedNode::new_unchecked(uri).into();
            assert_eq!(iri1, iri2)
        }

        #[test]
        fn from_str_base(base in URI_REGEX, extension in PATH_REGEX) {
            let iri1 = IriS::from_str_base(&extension, Some(&base))?;
            let iri2 = IriS::from_str_base(&format!("{}{}", base, extension), None)?;

            assert_eq!(iri1, iri2)
        }

        #[test]
        fn from_str_base_file(base in FILE_REGEX, path in PATH_REGEX) {
            let iri1 = IriS::from_str_base(&path, Some(&base))?;
            let iri2 = IriS::from_str_base(&format!("{}{}", base, path), None)?;

            assert_eq!(iri1, iri2)
        }

        #[test]
        fn iri_s(uri in URI_REGEX) {
            let iri1 = IriS::from_str(&uri)?;
            let iri2 = IriS::from_str(&uri)?;

            assert_eq!(iri1, iri2)
        }
    }
}

#[cfg(test)]
mod tests_macros_static {
    use super::*;

    #[test]
    fn test_macro_static_once() {
        static_once!(example, IriS, IriS::new_unchecked("https://example.org/"));
        let iri = example();
        assert_eq!(iri.as_str(), "https://example.org/")
    }

    #[test]
    fn test_macro_iri_lazy() {
        iri_once!(example, "https://example.org/");
        let iri = example();
        assert_eq!(iri.as_str(), "https://example.org/")
    }
}

mod tests_macros {
    use super::*;

    proptest! {
        #[test]
        fn test_macro_iri(uri in URI_REGEX) {
            let uri = &uri;
            let iri = iri!(uri);
            assert_eq!(iri.as_str(), uri)
        }
    }
}
