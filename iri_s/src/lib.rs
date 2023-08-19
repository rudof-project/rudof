pub mod iri;
pub mod iris;
pub mod iris_error;

pub use iri::*;
pub use iris::*;
pub use iris_error::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn iri_s_test() {
        let iri1: IriS = IriS::from_str("http://example.org/iri").unwrap();
        let iri2 = IriS::from_str("http://example.org/iri").unwrap();
        assert_eq!(iri1, iri2);
    }
}
