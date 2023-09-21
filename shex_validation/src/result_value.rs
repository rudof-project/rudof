use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum ResultValue {
    Pending,
    Ok,
    Failed,
    Unknown,
    Processing,
    Contradiction,
}

impl Display for ResultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultValue::Pending => write!(f, "Pending"),
            ResultValue::Ok => write!(f, "Ok"),
            ResultValue::Failed => write!(f, "Failed"),
            ResultValue::Unknown => write!(f, "Unknown"),
            ResultValue::Processing => write!(f, "Processing"),
            ResultValue::Contradiction => write!(f, "Contradiction"),
        }
    }
}
