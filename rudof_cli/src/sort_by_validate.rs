use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
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
