use srdf::SRDFBasic;

pub struct Targets<S: SRDFBasic> {
    iter: Box<dyn Iterator<Item = S::Term>>,
}

impl<S: SRDFBasic> Targets<S> {
    pub fn new(iter: impl Iterator<Item = S::Term>) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<S: SRDFBasic> Iterator for Targets<S> {
    type Item = S::Term;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
