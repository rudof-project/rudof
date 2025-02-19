use crate::rbe::Rbe;
use crate::Bag;
use crate::Cardinality;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Failures<A>
where
    A: Hash + Eq + Display,
{
    fs: Vec<(Box<Rbe<A>>, DerivError<A>)>,
}

impl<A> Failures<A>
where
    A: Hash + Eq + Display,
{
    pub fn new() -> Self {
        Self { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: Rbe<A>, err: DerivError<A>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl<A> Default for Failures<A>
where
    A: Hash + Eq + Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Display for Failures<A>
where
    A: Hash + Eq + Display + Display,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (expr, err) in &self.fs {
            writeln!(dest, "Error at {expr}: {err}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DerivError<A>
where
    A: Hash + PartialEq + Eq + Display,
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
    RangeLowerBoundBiggerMaxExpr {
        expr: Box<Rbe<A>>,
        card: Cardinality,
    },

    #[error("Derived expr: {non_nullable_rbe} is not nullable\nExpr {expr}\nBag: {bag}")]
    NonNullable {
        non_nullable_rbe: Box<Rbe<A>>,
        bag: Bag<A>,
        expr: Box<Rbe<A>>,
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
    CardinalityZeroZeroDeriv { symbol: A },

    #[error("Should fail but passed: {name}")]
    ShouldFailButPassed { name: String },

    #[error("Or values failed {e}\n {failures}")]
    OrValuesFail {
        e: Box<Rbe<A>>,
        failures: Failures<A>,
    },

    #[error("All values in or branch failed")]
    MkOrValuesFail,

    #[error("Error matching bag: {error_msg}\nBag: {bag}\nExpr: {expr}\nCurrent:{current}\nValue: {value}\nopen: {open}")]
    DerivBagError {
        error_msg: String,
        processed: Box<Bag<A>>,
        bag: Box<Bag<A>>,
        expr: Box<Rbe<A>>,
        current: Box<Rbe<A>>,
        value: A,
        open: bool,
    },
}
