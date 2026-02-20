use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShowNodeMode {
    Outgoing,
    Incoming,
    Both,
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}

impl FromStr for ShowNodeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "outgoing" => Ok(ShowNodeMode::Outgoing),
            "incoming" => Ok(ShowNodeMode::Incoming),
            "both" => Ok(ShowNodeMode::Both),
            _ => Err(format!(
                "Invalid node mode: '{}'. Valid values are: outgoing, incoming, both",
                s
            )),
        }
    }
}
