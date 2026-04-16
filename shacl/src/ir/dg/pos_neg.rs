use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PosNeg {
    #[default]
    Pos,
    Neg,
}

impl PosNeg {
    pub fn value(&self) -> bool {
        match self {
            PosNeg::Pos => true,
            PosNeg::Neg => false,
        }
    }

    pub fn change(&self) -> Self {
        match self {
            PosNeg::Pos => PosNeg::Neg,
            PosNeg::Neg => PosNeg::Pos,
        }
    }
}

impl Display for PosNeg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PosNeg::Pos => write!(f, "[+]"),
            PosNeg::Neg => write!(f, "[-]"),
        }
    }
}