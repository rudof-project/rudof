use crate::Cardinality;
use crate::Context;
use crate::Key;
use crate::Keys;
use crate::Ref;
use crate::Value;
use crate::Values;
use crate::failures::Failures;
use crate::rbe_cond::RbeCond;
use either::Either;
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

    #[error("No candidates. Expr: {rbe}, Values: [{values}]")]
    EmptyCandidates {
        rbe: Box<RbeCond<K, V, R, Ctx>>,
        values: Values<K, V, Ctx>,
    },

    #[error("No values for non-nullable expr: {rbe}")]
    EmptyCandidatesNoValues { rbe: Box<RbeCond<K, V, R, Ctx>> },

    #[error("RbeTable: Key {key} has no component associated. Available keys: {available_keys}")]
    RbeTableKeyWithoutComponent { key: K, available_keys: Keys<K> },
}

impl<K, V, R, Ctx> RbeError<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    /// Renders this error the same way `Display` does, except every `key`
    /// and `value` it mentions is rendered through the caller-supplied
    /// closures instead of `Display`. Lets a caller with more context (e.g.
    /// a `PrefixMap`) show qualified names instead of full IRIs, without
    /// this crate depending on anything IRI/prefix-specific.
    pub fn show_qualified(&self, show_key: &impl Fn(&K) -> String, show_value: &impl Fn(&V) -> String) -> String {
        match self {
            RbeError::UnexpectedEmpty { x, open } => {
                format!("Symbol {} doesn't match with empty. Open: {open}", show_key(x))
            },
            RbeError::UnexpectedSymbol { x, expected, open } => format!(
                "Symbol {} doesn't match with expected symbol {}. Open: {open}",
                show_key(x),
                show_key(expected)
            ),
            RbeError::MaxCardinalityZeroFoundValue { x } => {
                format!("Max cardinality 0, but found symbol {}", show_key(x))
            },
            RbeError::RangeNegativeLowerBound { min } => format!("Negative lower bound: {min}"),
            RbeError::RangeLowerBoundBiggerMax { symbol, card } => {
                format!("Min > Max in cardinality {card} for {}", show_key(symbol))
            },
            RbeError::RangeLowerBoundBiggerMaxExpr { expr, card } => format!(
                "Min > Max in cardinality {card} for {}",
                expr.show_qualified(show_key, show_value)
            ),
            RbeError::NonNullableMatch { non_nullable_rbe, expr } => format!(
                "Derived expr: {} is not nullable\nExpr {}",
                non_nullable_rbe.show_qualified(show_key, show_value),
                expr.show_qualified(show_key, show_value)
            ),
            RbeError::CardinalityFail {
                symbol,
                expected_cardinality,
                current_number,
            } => format!(
                "Cardinality failed for symbol {}. Current number: {current_number}, expected cardinality: {expected_cardinality}",
                show_key(symbol)
            ),
            RbeError::CardinalityFailRepeat {
                expected_cardinality,
                current_number,
            } => format!(
                "Cardinality failed for expr. Current number: {current_number}, expected cardinality: {expected_cardinality}"
            ),
            RbeError::CardinalityZeroZeroDeriv { .. } => {
                "Cardinality(0,0) but found symbol after derivative".to_string()
            },
            RbeError::ShouldFailButPassed { name } => format!("Should fail but passed: {name}"),
            RbeError::OrValuesFail { e, failures } => format!(
                "Or values failed {}\n {}",
                e.show_qualified(show_key, show_value),
                failures.show_qualified(show_key, show_value)
            ),
            RbeError::MkOrValuesFail => "All values in or branch failed".to_string(),
            RbeError::DerivIterError {
                error_msg,
                expr,
                current,
                key,
                open,
                ..
            } => format!(
                "Error matching iterator: {error_msg}\nExpr: {}\nCurrent:{}\nkey: {}\nopen: {open}",
                expr.show_qualified(show_key, show_value),
                current.show_qualified(show_key, show_value),
                show_key(key)
            ),
            RbeError::MsgError { msg } => msg.clone(),
            RbeError::EmptyCandidates { rbe, values } => format!(
                "No candidates. Mandatory values: [{}], Values: [{}]",
                show_mandatory_values(rbe, show_key, show_value),
                values.show_qualified(show_key, show_value)
            ),
            RbeError::EmptyCandidatesNoValues { rbe } => format!(
                "No values to match expression. Mandatory values: [{}]",
                show_mandatory_values(rbe, show_key, show_value)
            ),
            RbeError::RbeTableKeyWithoutComponent { key, available_keys } => format!(
                "Key {} has no component associated. Available keys: [{}]",
                show_key(key),
                available_keys.show_qualified(show_key)
            ),
        }
    }
}

/// Shows only the mandatory keys of `rbe` (the ones that must appear for it
/// to match), qualified through `show_key`. If `rbe` contains a `Fail` node,
/// there are no real keys to report, so its underlying error(s) are shown
/// instead, qualified through `show_key`/`show_value`.
fn show_mandatory_values<K, V, R, Ctx>(
    rbe: &RbeCond<K, V, R, Ctx>,
    show_key: &impl Fn(&K) -> String,
    show_value: &impl Fn(&V) -> String,
) -> String
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    match rbe.mandatory_values() {
        Either::Right(values) => values.iter().map(show_key).collect::<Vec<_>>().join(", "),
        Either::Left(errors) => errors
            .iter()
            .map(|e| e.show_qualified(show_key, show_value))
            .collect::<Vec<_>>()
            .join("; "),
    }
}
