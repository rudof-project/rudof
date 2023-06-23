use iri_s::IriError;
use prefix_map::PrefixMap;
use srdf::iri::IriS;

#[derive(Debug)]
pub struct Schema<'a> {
    pub(crate) id: Option<IriS>,
    pub(crate) base: Option<IriS>,
    pub(crate) prefixes: Option<PrefixMap<'a>>,
}

impl<'a> Schema<'a> {
    pub fn id(&self) -> Option<IriS> {
        self.id.clone()
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

    pub fn resolve(&self, alias: &str) -> Result<Option<IriS>, IriError> {
        match &self.prefixes {
            Some(pm) => pm.resolve(alias),
            None => Ok(None),
        }
    }
}
