use clap::ValueEnum;
use shex_ast::shapemap::result_shape_map::SortMode;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

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

impl From<&SortByResultShapeMap> for SortMode {
    fn from(format: &SortByResultShapeMap) -> Self {
        match format {
            SortByResultShapeMap::Node => SortMode::Node,
            SortByResultShapeMap::Shape => SortMode::Shape,
            SortByResultShapeMap::Status => SortMode::Status,
            SortByResultShapeMap::Details => SortMode::Details,
        }
    }
}

impl FromStr for SortByResultShapeMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "node" => Ok(SortByResultShapeMap::Node),
            "shape" => Ok(SortByResultShapeMap::Shape),
            "status" => Ok(SortByResultShapeMap::Status),
            "details" => Ok(SortByResultShapeMap::Details),
            _ => Err(format!("Unknown sort type: {s}")),
        }
    }
}
