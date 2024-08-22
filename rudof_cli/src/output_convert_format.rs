use clap::ValueEnum;

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertFormat {
    Default,
    Internal,
    JSON,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    HTML,
    SVG,
    PNG,
}

impl Display for OutputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertFormat::Internal => write!(dest, "internal"),
            OutputConvertFormat::JSON => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
            OutputConvertFormat::ShExC => write!(dest, "shexc"),
            OutputConvertFormat::ShExJ => write!(dest, "shexj"),
            OutputConvertFormat::Turtle => write!(dest, "turtle"),
            OutputConvertFormat::PlantUML => write!(dest, "uml"),
            OutputConvertFormat::HTML => write!(dest, "html"),
            OutputConvertFormat::PNG => write!(dest, "png"),
            OutputConvertFormat::SVG => write!(dest, "svg"),
        }
    }
}
