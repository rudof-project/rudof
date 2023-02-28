pub mod iri;
mod alias;
mod prefix_map;

pub use iri::*;
pub use alias::*;
pub use prefix_map::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_2_iris() {
        let iri1: MyIRI = iri::IRI::from(String::from("http://example.org/iri"));
        let iri2 = IRI::from(String::from("http://example.org/iri"));
        assert_eq!(iri1, iri2);
    }
}
