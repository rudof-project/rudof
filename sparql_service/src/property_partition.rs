use iri_s::IriS;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
pub struct PropertyPartition {
    property: IriS,
    triples: Option<isize>,
}

impl Display for PropertyPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PropertyPartition(property: {}, triples: {:?})",
            self.property, self.triples
        )
    }
}
