use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Deactivated {
    is_deactivated: bool, // TODO - Change with node expr
}

impl Deactivated {
    pub fn new(is_deactivated: bool) -> Self {
        Self { is_deactivated }
    }

    pub fn is_deactivated(&self) -> bool {
        self.is_deactivated
    }
}

impl Display for Deactivated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deactivated: {}", self.is_deactivated)
    }
}
