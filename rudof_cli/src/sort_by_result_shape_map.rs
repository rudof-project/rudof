use clap::ValueEnum;
use clientele::envs::windows;
use std::fmt::{Display, Formatter, write};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum SortByResultShapeMap {
    #[default]
    Node,
    Shape,
    Status,
    Details,
}

impl Display for SortByResultShapeMap {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SortByResultShapeMap::Node => write!(dest, "node"),
            SortByResultShapeMap::Shape => write!(dest, "shape"),
            SortByResultShapeMap::Status => write!(dest, "status"),
            SortByResultShapeMap::Details => write!(dest, "details"),
        }
    }
}
