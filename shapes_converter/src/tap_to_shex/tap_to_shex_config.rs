use iri_s::{iri, IriS};

#[derive(Debug, PartialEq)]
pub struct Tap2ShExConfig {
    pub base_iri: Option<IriS>,
}

impl Default for Tap2ShExConfig {
    fn default() -> Self {
        Self {
            base_iri: Some(iri!("http://example.org/")),
        }
    }
}
