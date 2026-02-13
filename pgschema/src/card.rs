use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Card {
    ZeroOrOne,
    One,
    ZeroOrMore,
    OneOrMore,
    Range(usize, Max),
}

impl Card {
    pub fn range(min: usize, max: Max) -> Self {
        Card::Range(min, max)
    }
    pub fn contains(&self, count: usize) -> bool {
        match self {
            Card::ZeroOrOne => count <= 1,
            Card::One => count == 1,
            Card::ZeroOrMore => true,
            Card::OneOrMore => count >= 1,
            Card::Range(min, max) => match max {
                Max::Unbounded => count >= *min,
                Max::Bounded(max) => count >= *min && count <= *max,
            },
        }
    }

    pub fn intersection(&self, other: &Card) -> Card {
        match (self, other) {
            (Card::ZeroOrOne, _) | (_, Card::ZeroOrOne) => Card::ZeroOrOne,
            (Card::One, _) | (_, Card::One) => Card::One,
            (Card::ZeroOrMore, _) | (_, Card::ZeroOrMore) => Card::ZeroOrMore,
            (Card::OneOrMore, _) | (_, Card::OneOrMore) => Card::OneOrMore,
            (Card::Range(min1, max1), Card::Range(min2, max2)) => Card::Range(*min1.max(min2), max1.min(max2)),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::ZeroOrOne => write!(f, "0..1"),
            Card::One => write!(f, "1"),
            Card::ZeroOrMore => write!(f, "0..*"),
            Card::OneOrMore => write!(f, "1..*"),
            Card::Range(min, max) => write!(f, "{}..{}", min, max),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Max {
    Unbounded,
    Bounded(usize),
}

impl Max {
    pub fn min(&self, other: &Max) -> Max {
        match (self, other) {
            (Max::Unbounded, _) => Max::Unbounded,
            (_, Max::Unbounded) => Max::Unbounded,
            (Max::Bounded(a), Max::Bounded(b)) => Max::Bounded(*a.min(b)),
        }
    }
}

impl Display for Max {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Max::Unbounded => write!(f, "∞"),
            Max::Bounded(n) => write!(f, "{}", n),
        }
    }
}
