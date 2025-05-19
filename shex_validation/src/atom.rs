use std::{fmt::Display, hash::Hash};

/// An atom can either be positive or negative
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Atom<A> {
    Pos(A),
    Neg(A),
}

impl<A> Atom<A>
where
    A: Clone,
{
    pub fn pos(pa: &A) -> Atom<A> {
        Atom::Pos(pa.clone())
    }

    pub fn neg(na: &A) -> Atom<A> {
        Atom::Neg(na.clone())
    }

    pub fn negated(&self) -> Atom<A> {
        match self {
            Atom::Pos(value) => Atom::Neg(value.clone()),
            Atom::Neg(value) => Atom::Pos(value.clone()),
        }
    }

    pub fn get_value(&self) -> &A {
        match self {
            Atom::Pos(value) => value,
            Atom::Neg(value) => value,
        }
    }
}

impl<A> Atom<A>
where
    A: Display,
{
    pub fn to_string(&self) -> String {
        match self {
            Atom::Pos(value) => format!("+({})", value),
            Atom::Neg(value) => format!("!({})", value),
        }
    }
}
