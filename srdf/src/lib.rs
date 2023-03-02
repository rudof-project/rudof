pub mod iri;
pub mod bnode;
mod alias;
pub mod prefix_map;

pub use iri::*;
pub use bnode::*;
pub use alias::*;
pub use prefix_map::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_2_iris() {
        let iri1: SIRI = SIRI::from("http://example.org/iri");
        let iri2 = SIRI::from("http://example.org/iri");
        assert_eq!(iri1, iri2);
    }
}
