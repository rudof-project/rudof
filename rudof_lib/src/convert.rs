use crate::{dctap_format::DCTapFormat, shacl_format::ShaclFormat, shex_format::ShExFormat};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InputConvertMode {
    Shacl,
    ShEx,
    Dctap,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::Shacl => write!(dest, "shacl"),
            InputConvertMode::ShEx => write!(dest, "shex"),
            InputConvertMode::Dctap => write!(dest, "dctap"),
        }
    }
}

impl FromStr for InputConvertMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shacl" => Ok(InputConvertMode::Shacl),
            "shex" => Ok(InputConvertMode::ShEx),
            "dctap" => Ok(InputConvertMode::Dctap),
            _ => Err(format!(
                "Invalid conversion mode: '{}'. Supported modes: shacl, shex, dctap",
                s
            )),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum InputConvertFormat {
    Csv,
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    Xlsx,
}

impl InputConvertFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, String> {
        match self {
            InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            InputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => Err(format!("Converting ShEx, format {self} not supported")),
        }
    }
    pub fn to_shacl_format(self) -> Result<ShaclFormat, String> {
        match self {
            InputConvertFormat::Turtle => Ok(ShaclFormat::Turtle),
            _ => Err(format!("Converting to SHACL, format {self} not supported")),
        }
    }

    pub fn to_dctap_format(self) -> Result<DCTapFormat, String> {
        match self {
            InputConvertFormat::Csv => Ok(DCTapFormat::Csv),
            InputConvertFormat::Xlsx => Ok(DCTapFormat::Xlsx),
            _ => Err(format!("Converting to DCTAP, format {self} not supported")),
        }
    }
}

impl FromStr for InputConvertFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(InputConvertFormat::Csv),
            "xlsx" => Ok(InputConvertFormat::Xlsx),
            "shexc" => Ok(InputConvertFormat::ShExC),
            "shexj" => Ok(InputConvertFormat::ShExJ),
            "turtle" => Ok(InputConvertFormat::Turtle),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertFormat::Csv => write!(dest, "csv"),
            InputConvertFormat::Xlsx => write!(dest, "xlsx"),
            InputConvertFormat::ShExC => write!(dest, "shexc"),
            InputConvertFormat::ShExJ => write!(dest, "shexj"),
            InputConvertFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OutputConvertFormat {
    Default,
    Internal,
    Json,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    Html,
    Svg,
    Png,
}

impl OutputConvertFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, String> {
        match self {
            OutputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => Err(format!("Converting ShEx, format {self} not supported")),
        }
    }

    pub fn to_shacl_format(self) -> Result<ShaclFormat, String> {
        match self {
            OutputConvertFormat::Default => Ok(ShaclFormat::Internal),
            OutputConvertFormat::Turtle => Ok(ShaclFormat::Turtle),
            _ => Err(format!("Converting to SHACL, format {self} not supported")),
        }
    }
}

impl Display for OutputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertFormat::Internal => write!(dest, "internal"),
            OutputConvertFormat::Json => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
            OutputConvertFormat::ShExC => write!(dest, "shexc"),
            OutputConvertFormat::ShExJ => write!(dest, "shexj"),
            OutputConvertFormat::Turtle => write!(dest, "turtle"),
            OutputConvertFormat::PlantUML => write!(dest, "uml"),
            OutputConvertFormat::Html => write!(dest, "html"),
            OutputConvertFormat::Png => write!(dest, "png"),
            OutputConvertFormat::Svg => write!(dest, "svg"),
        }
    }
}

impl FromStr for OutputConvertFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(OutputConvertFormat::Default),
            "internal" => Ok(OutputConvertFormat::Internal),
            "json" => Ok(OutputConvertFormat::Json),
            "shexc" => Ok(OutputConvertFormat::ShExC),
            "shexj" => Ok(OutputConvertFormat::ShExJ),
            "turtle" => Ok(OutputConvertFormat::Turtle),
            "uml" => Ok(OutputConvertFormat::PlantUML),
            "html" => Ok(OutputConvertFormat::Html),
            "svg" => Ok(OutputConvertFormat::Svg),
            "png" => Ok(OutputConvertFormat::Png),
            _ => Err(format!("Unsupported output conversion format: '{}'", s)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OutputConvertMode {
    Sparql,
    ShEx,
    Uml,
    Html,
    Shacl,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::Sparql => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::Uml => write!(dest, "uml"),
            OutputConvertMode::Html => write!(dest, "html"),
            OutputConvertMode::Shacl => write!(dest, "shacl"),
        }
    }
}

impl FromStr for OutputConvertMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sparql" => Ok(OutputConvertMode::Sparql),
            "shex" => Ok(OutputConvertMode::ShEx),
            "uml" => Ok(OutputConvertMode::Uml),
            "html" => Ok(OutputConvertMode::Html),
            "shacl" => Ok(OutputConvertMode::Shacl),
            _ => Err(format!(
                "Unknown output conversion mode: '{}'. Supported modes: sparql, shex, uml, html, shacl",
                s
            )),
        }
    }
}
