use crate::rbe::Rbe;
use crate::Bag;
use crate::Cardinality;
use std::hash::Hash;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use std::fmt::Display;

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum RbeError<A>
where A: Hash + PartialEq + Eq + Display,
{
    #[error("Symbol {x} doesn't match with empty. Open: {open}")]
    UnexpectedEmpty { x: A, open: bool },

    #[error("Symbol {x} doesn't match with expected symbol {expected}. Open: {open}")]
    UnexpectedSymbol { x: A, expected: A, open: bool },

    #[error("Max cardinality 0, but found symbol {x}")]
    MaxCardinalityZeroFoundValue { x: A },

    // TODO: Maybe this error is redundant?
    #[error("Negative lower bound: {min}")]
    RangeNegativeLowerBound { min: usize },

    #[error("Min > Max in cardinality {card} for {symbol}")]
    RangeLowerBoundBiggerMax { symbol: A, card: Cardinality },

    #[error("Min > Max in cardinality {card} for {expr}")]
    RangeLowerBoundBiggerMaxExpr { expr: Box<Rbe<A>>, card: Cardinality },

    #[error("Expected {non_nullable_rbe} but all symbols in bag: {bag} have been processed")]
    NonNullable {
        non_nullable_rbe: Box<Rbe<A>>,
        bag: Bag<A>,
    },

    #[error("Cardinality failed for symbol {symbol}. Current number: {current_number}, expected cardinality: {expected_cardinality}")]
    CardinalityFail {
        symbol: A,
        expected_cardinality: Cardinality,
        current_number: usize,
    },

    #[error("Cardinality failed for expr. Current number: {current_number}, expected cardinality: {expected_cardinality}")]
    CardinalityFailRepeat {
        expected_cardinality: Cardinality,
        current_number: usize,
    },

    #[error("Cardinality(0,0) but found symbol after derivative")]
    CardinalityZeroZeroDeriv {
        symbol: A
    },

    #[error("Should fail but passed: {name}")]
    ShouldFailButPassed {
        name: String
    },


    #[error("Or values failed {e}. ")]
    OrValuesFail{ 
        e: Box<Rbe<A>>,
        // failures: Vec<(Box<Rbe<A>>, Box<RbeError<A>>)>
    } ,

    #[error("MkOr values failed")]
    MkOrValuesFail,

    #[error("Error matching bag:...\nBag: {bag}\nExpr: {expr}\nCurrent:{current}\nValue: {value}")]
    DerivBagError { 
        // error: Box<RbeError<A>>, 
        processed: Vec<A>,
        bag: Bag<A>,
        expr: Box<Rbe<A>>,
        current: Box<Rbe<A>>,
        value: A
    }
}
