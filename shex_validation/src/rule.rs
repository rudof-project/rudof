use std::hash::Hash;

use crate::Atom;

#[derive(PartialEq, Clone, Debug)]
pub struct Rule<A>
where
    A: Hash + Eq,
{
    head: Atom<A>,
    body: Vec<Atom<A>>,
}

impl<A> Rule<A>
where
    A: Hash + Eq + Clone,
{
    pub fn new(head: Atom<A>, body: Vec<Atom<A>>) -> Rule<A> {
        Rule { head, body }
    }

    /// A fact is a rule with an empty body
    fn fact(a: Atom<A>) -> Rule<A> {
        Rule {
            head: a,
            body: Vec::new(),
        }
    }

    fn with_solved(&mut self, a: Atom<A>) -> &Self {
        // If the atom is in the body, remove it
        if let Some(index) = self.body.iter().position(|value| *value == a) {
            self.body.swap_remove(index);
        }
        self
    }
}
