use std::fmt::Display;

use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderValue {
    Integer(i128),
    Decimal(Decimal),
}

impl Display for OrderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderValue::Integer(i) => write!(f, "{}", i),
            OrderValue::Decimal(d) => write!(f, "{}", d),
        }
    }
}
