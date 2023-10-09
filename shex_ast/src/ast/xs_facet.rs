use serde_derive::{Deserialize, Serialize};
use rust_decimal::prelude::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum XsFacet {
    StringFacet(StringFacet),
    NumericFacet(NumericFacet),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum StringFacet {
    Length(usize),
    MinLength(usize),
    MaxLength(usize),

    #[serde(rename = "pattern")]
    Pattern { 
        str: String, 
        
        #[serde(default, skip_serializing_if = "Option::is_none")]
        flags: Option<String> 
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum NumericFacet {
    MinInclusive(NumericLiteral),
    MinExclusive(NumericLiteral),
    MaxInclusive(NumericLiteral),
    MaxExclusive(NumericLiteral),
    TotalDigits(usize),
    FractionDigits(usize)
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum NumericLiteral {
    Integer(usize),
    Decimal(Decimal),
    Double(f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_xsfacet() {
        let pattern = StringFacet::Pattern { str: "o*".to_string(), flags: None };

        let json_pattern = serde_json::to_string(&pattern).unwrap();
        assert_eq!(json_pattern, "{\"pattern\": \"o*\"}");
    }
}