use srdf::model::rdf::Rdf;
use srdf::oxgraph::OxGraph;

pub struct Store<R: Rdf> {
    store: R,
    subset: Option<OxGraph>,
}

impl<R: Rdf> Store<R> {
    pub fn new(srdf: R, is_subsetting: bool) -> Self {
        Self {
            store: srdf,
            subset: match is_subsetting {
                true => Some(Default::default()),
                false => None,
            },
        }
    }

    pub fn inner_store(&self) -> &R {
        &self.store
    }
}
