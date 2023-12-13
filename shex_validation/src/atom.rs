use std::hash::Hash;

/// An atom can either be positive or negative
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Atom<A> {
    Pos(PosAtom<A>),
    Neg(NegAtom<A>),
}

impl<A> Atom<A>
where
    A: Clone,
{
    pub fn pos(pa: &PosAtom<A>) -> Atom<A> {
        Atom::Pos(pa.clone())
    }

    pub fn neg(na: &NegAtom<A>) -> Atom<A> {
        Atom::Neg(na.clone())
    }

    pub fn negated(&self) -> Atom<A> {
        match self {
            Atom::Pos(PosAtom { value }) => Atom::Neg(NegAtom {
                value: value.clone(),
            }),
            Atom::Neg(NegAtom { value }) => Atom::Pos(PosAtom {
                value: value.clone(),
            }),
        }
    }

    pub fn get_value(&self) -> &A {
        match self {
            Atom::Pos(PosAtom { value }) => value,
            Atom::Neg(NegAtom { value }) => value,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct PosAtom<A> {
    value: A,
}

impl<A> PosAtom<A> {
    pub fn new(value: A) -> PosAtom<A> {
        PosAtom { value }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct NegAtom<A> {
    value: A,
}

impl<A> NegAtom<A> {
    pub fn new(value: A) -> NegAtom<A> {
        NegAtom { value }
    }
}
