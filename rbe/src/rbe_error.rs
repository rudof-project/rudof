use crate::Cardinality;
use crate::Context;
use crate::Key;
use crate::Keys;
use crate::Ref;
use crate::Value;
use crate::Values;
use crate::failures::Failures;
use crate::rbe_cond::RbeCond;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a regular bag expression error.
#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum RbeError<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    #[error("Symbol {x} doesn't match with empty. Open: {open}")]
    UnexpectedEmpty { x: K, open: bool },

    #[error("Symbol {x} doesn't match with expected symbol {expected}. Open: {open}")]
    UnexpectedSymbol { x: K, expected: K, open: bool },

    #[error("Max cardinality 0, but found symbol {x}")]
    MaxCardinalityZeroFoundValue { x: K },

    // TODO: Maybe this error is redundant?
    #[error("Negative lower bound: {min}")]
    RangeNegativeLowerBound { min: usize },

    #[error("Min > Max in cardinality {card} for {symbol}")]
    RangeLowerBoundBiggerMax { symbol: K, card: Cardinality },

    #[error("Min > Max in cardinality {card} for {expr}")]
    RangeLowerBoundBiggerMaxExpr {
        expr: Box<RbeCond<K, V, R, Ctx>>,
        card: Cardinality,
    },

    #[error("Derived expr: {non_nullable_rbe} is not nullable\nExpr {expr}")]
    NonNullableMatch {
        non_nullable_rbe: Box<RbeCond<K, V, R, Ctx>>,
        expr: Box<RbeCond<K, V, R, Ctx>>,
    },

    #[error(
        "Cardinality failed for symbol {symbol}. Current number: {current_number}, expected cardinality: {expected_cardinality}"
    )]
    CardinalityFail {
        symbol: K,
        expected_cardinality: Cardinality,
        current_number: usize,
    },

    #[error(
        "Cardinality failed for expr. Current number: {current_number}, expected cardinality: {expected_cardinality}"
    )]
    CardinalityFailRepeat {
        expected_cardinality: Cardinality,
        current_number: usize,
    },

    #[error("Cardinality(0,0) but found symbol after derivative")]
    CardinalityZeroZeroDeriv { symbol: K },

    #[error("Should fail but passed: {name}")]
    ShouldFailButPassed { name: String },

    #[error("Or values failed {e}\n {failures}")]
    OrValuesFail {
        e: Box<RbeCond<K, V, R, Ctx>>,
        failures: Failures<K, V, R, Ctx>,
    },

    #[error("All values in or branch failed")]
    MkOrValuesFail,

    #[error("Error matching iterator: {error_msg}\nExpr: {expr}\nCurrent:{current}\nkey: {key}\nopen: {open}")]
    DerivIterError {
        error_msg: String,
        processed: Vec<(K, V, Ctx)>,
        expr: Box<RbeCond<K, V, R, Ctx>>,
        current: Box<RbeCond<K, V, R, Ctx>>,
        key: K,
        open: bool,
    },

    #[error("{msg}")]
    MsgError { msg: String },

    #[error("Empty candidates for regular expression: {rbe} and values: {values}")]
    EmptyCandidates {
        rbe: Box<RbeCond<K, V, R, Ctx>>,
        values: Values<K, V, Ctx>,
    },

    #[error("RbeTable: Key {key} has no component associated. Available keys: {available_keys}")]
    RbeTableKeyWithoutComponent { key: K, available_keys: Keys<K> },
}
