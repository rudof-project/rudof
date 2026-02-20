use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    Select,
    Construct,
    Ask,
    Describe,
}

impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            QueryType::Select => "SELECT",
            QueryType::Construct => "CONSTRUCT",
            QueryType::Ask => "ASK",
            QueryType::Describe => "DESCRIBE",
        };
        write!(f, "{s}")
    }
}

impl FromStr for QueryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "select" => Ok(QueryType::Select),
            "construct" => Ok(QueryType::Construct),
            "ask" => Ok(QueryType::Ask),
            "describe" => Ok(QueryType::Describe),
            _ => Err(format!("Unknown query type: {s}")),
        }
    }
}
