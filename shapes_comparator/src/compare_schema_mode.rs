use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum CompareSchemaMode {
    #[default]
    ShEx,
    Shacl,
    ServiceDescription,
}

impl CompareSchemaMode {}

impl Display for CompareSchemaMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CompareSchemaMode::ShEx => write!(dest, "shex"),
            CompareSchemaMode::Shacl => write!(dest, "shacl"),
            CompareSchemaMode::ServiceDescription => write!(dest, "service_description"),
        }
    }
}
