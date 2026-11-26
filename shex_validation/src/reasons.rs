use crate::Reason;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Reasons {
    reasons: Vec<Reason>,
}

impl Reasons {
    pub fn new(reasons: Vec<Reason>) -> Reasons {
        Reasons { reasons }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Reason> {
        self.reasons.iter()
    }
}

impl Display for Reasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for reason in self.reasons.iter() {
            writeln!(f, "  {reason}")?;
        }
        Ok(())
    }
}
