use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SortByValidate {
    #[default]
    Node,
    Details,
}

impl Display for SortByValidate {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SortByValidate::Node => write!(dest, "node"),
            SortByValidate::Details => write!(dest, "details"),
        }
    }
}

impl FromStr for SortByValidate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "node" => Ok(SortByValidate::Node),
            "details" => Ok(SortByValidate::Details),
            other => Err(format!("Invalid sort mode '{}'. Valid options: node, details", other)),
        }
    }
}
