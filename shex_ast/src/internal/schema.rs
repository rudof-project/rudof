use iri_s::IriS;
use iri_s::IriSError;
use prefixmap::PrefixMap;

#[derive(Debug)]
pub struct Schema {
    pub(crate) id: Option<IriS>,
    pub(crate) base: Option<IriS>,
    pub(crate) prefixes: Option<PrefixMap>,
}

impl<'a> Schema {
    pub fn id(&self) -> Option<IriS> {
        self.id.clone()
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

    pub fn resolve(&self, alias: &str) -> Result<Option<IriS>, IriSError> {
        match &self.prefixes {
            Some(pm) => pm.resolve(alias),
            None => Ok(None),
        }
    }
}
