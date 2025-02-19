use crate::failures::Failures;
use crate::rbe1::Rbe;
use crate::Cardinality;
use crate::Key;
use crate::Keys;
use crate::Ref;
use crate::Value;
use crate::Values;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;

/// Represents a regular bag expression error.
#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum RbeError<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
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
        expr: Box<Rbe<K, V, R>>,
        card: Cardinality,
    },

    #[error("Derived expr: {non_nullable_rbe} is not nullable\nExpr {expr}")]
    NonNullableMatch {
        non_nullable_rbe: Box<Rbe<K, V, R>>,
        expr: Box<Rbe<K, V, R>>,
    },

    #[error("Cardinality failed for symbol {symbol}. Current number: {current_number}, expected cardinality: {expected_cardinality}")]
    CardinalityFail {
        symbol: K,
        expected_cardinality: Cardinality,
        current_number: usize,
    },

    #[error("Cardinality failed for expr. Current number: {current_number}, expected cardinality: {expected_cardinality}")]
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
        e: Box<Rbe<K, V, R>>,
        failures: Failures<K, V, R>,
    },

    #[error("All values in or branch failed")]
    MkOrValuesFail,

    #[error("Error matching iterator: {error_msg}\nExpr: {expr}\nCurrent:{current}\nkey: {key}\nopen: {open}")]
    DerivIterError {
        error_msg: String,
        processed: Vec<(K, V)>,
        expr: Box<Rbe<K, V, R>>,
        current: Box<Rbe<K, V, R>>,
        key: K,
        open: bool,
    },

    #[error("{msg}")]
    MsgError { msg: String },

    #[error("Empty candidates for: \nRegular expression: {rbe}\nValues:{values}")]
    EmptyCandidates {
        rbe: Box<Rbe<K, V, R>>,
        values: Values<K, V>,
    },

    #[error("RbeTable: Key {key} has no component associated. Available keys: {available_keys}")]
    RbeTableKeyWithoutComponent { key: K, available_keys: Keys<K> },
}
