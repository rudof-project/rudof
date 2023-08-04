use crate::rbe1::Rbe1;
use crate::bag1::Bag1;
use crate::Cardinality;
use std::fmt::Formatter;
use std::hash::Hash;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use std::fmt::Display;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
{
   fs: Vec<(Box<Rbe1<K,V,R>>, Rbe1Error<K,V,R>)>
}

impl <K, V, R> Failures<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
{
    pub fn new() -> Failures<K, V, R> {
       Failures {
        fs: Vec::new()
       }
    }

    pub fn push(&mut self, expr: Rbe1<K, V, R>, err: Rbe1Error<K, V, R>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl <K, V, R> Display for Failures<K, V, R> 
where K: Hash + Eq + Display + Display + Default,
      V: Hash + Default + Display + Eq + Clone,
      R: Default + Display + PartialEq + Clone
      {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        for (expr, err) in &self.fs {
            write!(dest, "Error at {expr}: {err}\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum Rbe1Error<K,V,R>
where K: Hash + PartialEq + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
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
    RangeLowerBoundBiggerMaxExpr { expr: Box<Rbe1<K,V,R>>, card: Cardinality },

    #[error("Derived expr: {non_nullable_rbe} is not nullable\nExpr {expr}\nBag: {bag}")]
    NonNullable {
        non_nullable_rbe: Box<Rbe1<K,V,R>>,
        bag: Bag1<K,V>,
        expr: Box<Rbe1<K,V,R>>
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
    CardinalityZeroZeroDeriv {
        symbol: K
    },

    #[error("Should fail but passed: {name}")]
    ShouldFailButPassed {
        name: String
    },


    #[error("Or values failed {e}\n {failures}")]
    OrValuesFail{ 
        e: Box<Rbe1<K,V,R>>,
        failures: Failures<K, V, R>
    } ,

    #[error("All values in or branch failed")]
    MkOrValuesFail,

    #[error("Error matching bag: {error_msg}\nBag: {bag}\nExpr: {expr}\nCurrent:{current}\nValue: {value}\nopen: {open}")]
    DerivBagError { 
        error_msg: String, 
        processed: Bag1<K,V>,
        bag: Bag1<K, V>,
        expr: Box<Rbe1<K,V,R>>,
        current: Box<Rbe1<K,V,R>>,
        value: K,
        open: bool,
    }
}
