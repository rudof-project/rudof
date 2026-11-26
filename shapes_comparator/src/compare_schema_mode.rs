use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum CompareSchemaMode {
    #[default]
    ShEx,
    Shacl,
    ServiceDescription,
}

impl CompareSchemaMode {}

impl FromStr for CompareSchemaMode {
    type Err = crate::ComparatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shex" => Ok(CompareSchemaMode::ShEx),
            "shacl" => Ok(CompareSchemaMode::Shacl),
            "service_description" => Ok(CompareSchemaMode::ServiceDescription),
            _ => Err(crate::ComparatorError::UnknownSchemaMode(s.to_string())),
        }
    }
}

impl Display for CompareSchemaMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CompareSchemaMode::ShEx => write!(dest, "shex"),
            CompareSchemaMode::Shacl => write!(dest, "shacl"),
            CompareSchemaMode::ServiceDescription => write!(dest, "service_description"),
        }
    }
}
