pub mod iri;

pub use iri::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iri_s_test() {
        let iri1: IriS = IriS::from_str("http://example.org/iri");
        let iri2 = IriS::from_str("http://example.org/iri");
        assert_eq!(iri1, iri2);
    }
}
