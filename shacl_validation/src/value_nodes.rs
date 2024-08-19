use srdf::SRDFBasic;
use std::cell::RefCell;
use std::rc::Rc;

use crate::targets::Targets;

pub struct ValueNodes<S: SRDFBasic>
where
    S::Term:,
{
    iter: Rc<RefCell<dyn Iterator<Item = (S::Term, Targets<S>)>>>,
}

impl<S: SRDFBasic> ValueNodes<S>
where
    S:,
{
    pub fn new(iter: impl Iterator<Item = (S::Term, Targets<S>)>) -> Self {
        Self {
            iter: Rc::new(RefCell::new(iter)),
        }
    }

    pub fn iter_outer(&self) -> impl Iterator<Item = (S::Term, Targets<S>)> {
        let iter_clone = Rc::clone(&self.iter);
        std::iter::from_fn(move || {
            let mut iter = iter_clone.borrow_mut();
            iter.next()
                .map(|(focus_nodes, value_nodes)| (focus_nodes, value_nodes))
        })
    }

    pub fn iter_full(&self) -> impl Iterator<Item = (S::Term, S::Term)> {
        let iter_clone = Rc::clone(&self.iter);
        std::iter::from_fn(move || {
            let mut iter = iter_clone.borrow_mut();
            iter.next()
                .map(|(outer, inner_iter)| inner_iter.map(move |inner| (outer.clone(), inner)))?
                .next()
        })
    }
}
