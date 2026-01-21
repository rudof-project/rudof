use once_cell::sync::Lazy;
use iri_s::IriS;
use prefixmap::IriRef;

/// XSD and RDF datatype IRIs as static lazy references
pub static XSD_STRING: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#string",
    ))
});

pub static RDF_LANG_STRING: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
    ))
});

pub static XSD_BOOLEAN: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#boolean",
    ))
});

pub static XSD_INTEGER: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#integer",
    ))
});

pub static XSD_DATETIME: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#dateTime",
    ))
});

pub static XSD_DOUBLE: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#double",
    ))
});

pub static XSD_DECIMAL: Lazy<IriRef> = Lazy::new(|| {
    IriRef::iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#decimal",
    ))
});
