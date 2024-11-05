use srdf::SRDFBasic;
use srdf::SRDFGraph;

pub struct Store<S: SRDFBasic> {
    store: S,
    subset: Option<SRDFGraph>,
}

impl<S: SRDFBasic> Store<S> {
    pub fn new(srdf: S, slurp: bool) -> Self {
        Self {
            store: srdf,
            subset: match slurp {
                true => Some(Default::default()),
                false => None,
            },
        }
    }

    pub fn inner_store(&self) -> &S {
        &self.store
    }
}
