use crate::lang::Lang;
use crate::iri::IRI;
pub trait Literal<'a> {
    fn lexicalForm(&self) -> &'a str ;
    fn datatype(&self) -> &dyn IRI ;
    fn lang(&self) -> &dyn Lang ;
}

