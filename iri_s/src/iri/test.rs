#[cfg(test)]
mod tests {
    use crate::iri::IriS;
    use oxrdf::NamedNode;
    use std::str::FromStr;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("https://example.org/").unwrap();
        assert_eq!(iri.to_string(), "https://example.org/");
    }

    #[test]
    fn obtaining_iri_as_str() {
        let iri = IriS::from_str("https://example.org/p1").unwrap();
        assert_eq!(iri.as_str(), "https://example.org/p1");
    }

    #[test]
    fn extending_iri() {
        let base = NamedNode::new("https://example.org/").unwrap();
        let base_iri = IriS::from_named_node(&base);
        let extended = base_iri.extend("knows").unwrap();
        assert_eq!(extended.as_str(), "https://example.org/knows");
    }

    #[test]
    fn comparing_iris() {
        let iri1 = IriS::from_named_node(&NamedNode::new_unchecked("https://example.org/name"));
        let iri2 = IriS::from_named_node(&NamedNode::new_unchecked("https://example.org/name"));
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn from_str_base() {
        let iri1 = IriS::from_str_base("name", Some("https://example.org/")).unwrap();
        let iri2 = IriS::from_str_base("https://example.org/name", None).unwrap();
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn from_str_base_file() {
        let iri1 = IriS::from_str_base(
            "examples/shex/base.shex",
            Some("file:///home/labra/src/rust/rudof/"),
        )
        .unwrap();
        let iri2 = IriS::from_str_base(
            "file:///home/labra/src/rust/rudof/examples/shex/base.shex",
            None,
        )
        .unwrap();
        assert_eq!(iri1, iri2);
    }

    #[test]
    fn iri_s_test() {
        let iri1: IriS = IriS::from_str("https://example.org/iri").unwrap();
        let iri2 = IriS::from_str("https://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }
}

#[cfg(test)]
mod tests_macros {
    use crate::{IriS, iri, iri_once, static_once};

    #[test]
    fn test_macro_iri() {
        let iri = iri!("https://example.org/");
        assert_eq!(iri.as_str(), "https://example.org/")
    }

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
