use iri_s::IriS;
use iri_s::IriSError;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;

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

    pub fn resolve(&self, alias: &str) -> Result<Option<IriS>, PrefixMapError> {
        match &self.prefixes {
            Some(pm) => { 
                let iri = pm.resolve(alias)?;
                Ok(Some(iri))
            },
            None => Ok(None),
        }
    }
}
