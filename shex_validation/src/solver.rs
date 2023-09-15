use indexmap::IndexSet;
use std::hash::Hash;

#[derive(PartialEq, Clone, Debug)]
pub struct Rule<A>
where
    A: Hash + Eq,
{
    head: Atom<A>,
    body: IndexSet<Atom<A>>,
}

impl<A> Rule<A>
where
    A: Hash + Eq + Clone,
{
    pub fn new(head: Atom<A>, body_atoms: Vec<Atom<A>>) -> Rule<A> {
        let mut body = IndexSet::new();
        for atom in body_atoms {
            body.insert(atom);
        }
        Rule { head, body }
    }

    /// A fact is a rule with an empty body
    fn fact(a: Atom<A>) -> Rule<A> {
        Rule {
            head: a,
            body: IndexSet::new(),
        }
    }

    fn with_solved(&mut self, a: Atom<A>) -> &Self {
        if self.body.contains(&a) {
            self.body.remove(&a);
        };
        self
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Atom<A> {
    Pos { value: A },
    Neg { value: A },
}

impl<A> Atom<A>
where
    A: Clone,
{
    pub fn pos(value: A) -> Atom<A> {
        Atom::Pos { value }
    }

    pub fn neg(value: A) -> Atom<A> {
        Atom::Neg { value }
    }

    pub fn negated(&self) -> Atom<A> {
        match self {
            Atom::Pos { value } => Atom::Neg {
                value: value.clone(),
            },
            Atom::Neg { value } => Atom::Pos {
                value: value.clone(),
            },
        }
    }

    pub fn get_value(&self) -> &A {
        match self {
            Atom::Pos { value } => value,
            Atom::Neg { value } => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Atom, Rule};

    #[test]
    fn test_rule_creation() {
        let mut rule1 = Rule::new(Atom::pos('A'), vec![Atom::pos('B'), Atom::pos('C')]);
        let expected = Rule::new(Atom::pos('A'), vec![Atom::pos('C')]);
        assert_eq!(rule1.with_solved(Atom::pos('B')), &expected);
    }
}
