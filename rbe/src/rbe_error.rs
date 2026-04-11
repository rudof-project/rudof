use crate::Cardinality;
use crate::Context;
use crate::Key;
use crate::Keys;
use crate::Ref;
use crate::State;
use crate::Value;
use crate::Values;
use crate::failures::Failures;
use crate::rbe1::Rbe;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a regular bag expression error.
#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum RbeError<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
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
        expr: Box<Rbe<K, V, R, Ctx, St>>,
        card: Cardinality,
    },

    #[error("Derived expr: {non_nullable_rbe} is not nullable\nExpr {expr}")]
    NonNullableMatch {
        non_nullable_rbe: Box<Rbe<K, V, R, Ctx, St>>,
        expr: Box<Rbe<K, V, R, Ctx, St>>,
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
        e: Box<Rbe<K, V, R, Ctx, St>>,
        failures: Failures<K, V, R, Ctx, St>,
    },

    #[error("All values in or branch failed")]
    MkOrValuesFail,

    #[error("Error matching iterator: {error_msg}\nExpr: {expr}\nCurrent:{current}\nkey: {key}\nopen: {open}")]
    DerivIterError {
        error_msg: String,
        processed: Vec<(K, V, Ctx, St)>,
        expr: Box<Rbe<K, V, R, Ctx, St>>,
        current: Box<Rbe<K, V, R, Ctx, St>>,
        key: K,
        open: bool,
    },

    #[error("{msg}")]
    MsgError { msg: String },

    #[error("Empty candidates for regular expression: {rbe} and values: {values}")]
    EmptyCandidates {
        rbe: Box<Rbe<K, V, R, Ctx, St>>,
        values: Values<K, V, Ctx, St>,
    },

    #[error("RbeTable: Key {key} has no component associated. Available keys: {available_keys}")]
    RbeTableKeyWithoutComponent { key: K, available_keys: Keys<K> },
}
