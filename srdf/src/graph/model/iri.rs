use iri_s::IriS;
use oxrdf::NamedNode as OxIri;
use oxrdf::NamedNodeRef as OxIriRef;

use crate::model::Iri;

impl Iri for OxIri {
    type IriRef<'x> = OxIriRef<'x> where Self: 'x;

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}

impl Iri for OxIriRef<'_> {
    type IriRef<'x> = Self where Self: 'x;

    fn into_iri_s(&self) -> IriS {
        IriS::new_unchecked(self.as_str().to_string())
    }
}
