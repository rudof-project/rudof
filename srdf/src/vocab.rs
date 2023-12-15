use const_format::concatcp;
use iri_s::IriS;

pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
pub const RDF_TYPE: &str = concatcp!(RDF, "type");
pub const RDF_FIRST: &str = concatcp!(RDF, "first");
pub const RDF_REST: &str = concatcp!(RDF, "rest");
pub const RDF_NIL: &str = concatcp!(RDF, "nil");

pub struct Vocab {}

impl Vocab {
    #[inline]
    pub fn rdf_type() -> IriS {
        IriS::new_unchecked(RDF_TYPE)
    }

    #[inline]
    pub fn rdf_first() -> IriS {
        IriS::new_unchecked(RDF_FIRST)
    }

    #[inline]
    pub fn rdf_rest() -> IriS {
        IriS::new_unchecked(RDF_REST)
    }

    #[inline]
    pub fn rdf_nil() -> IriS {
        IriS::new_unchecked(RDF_NIL)
    }
}
