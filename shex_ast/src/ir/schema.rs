use iri_s::IriS;
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;

// TODO: I think this struct is not used anywhere
// It was in the original design but then we moved to using SchemaIR directly

#[derive(Debug)]
pub struct Schema {
    pub(crate) id: Option<IriS>,
    pub(crate) base: Option<IriS>,
    pub(crate) prefixes: Option<PrefixMap>,
}

impl Schema {
    pub fn id(&self) -> Option<IriS> {
        self.id.clone()
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

    pub fn resolve(&self, alias: &str) -> Result<Option<IriS>, PrefixMapError> {
        self.prefixes.as_ref().map(|pm| pm.resolve(alias)).transpose()
    }
}
