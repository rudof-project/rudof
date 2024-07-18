use prefixmap::IriRef;

#[derive(Debug, PartialEq, Clone)]

pub enum Value {
    Str(String),
    Iri(IriRef),
    // TODO: Add more types of values or decide if we reuse from ShEx values
}
