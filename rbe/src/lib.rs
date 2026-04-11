//! Regular Bag Expressions (rbe)
//!
//! Provides an implementation of Regular Bag Expressions which are the expressions internally employed by
//! the implementations of Shape Expressions
//! More information about Regular Bag Expressions:
//!
//! [Complexity and Expressiveness of ShEx for RDF](https://labra.weso.es/publication/2015_complexityexpressivenessshexrdf/)
//! S. Staworko, I. Boneva, J. Labra, S. Hym, E. Prud'hommeaux, H. Solbrig
//!
pub mod bag;
pub mod candidate;
pub mod cardinality;
pub mod component;
pub mod deriv_error;
pub mod deriv_n;
pub mod empty_iter;
pub mod failures;
pub mod keys;
pub mod match_cond;
pub mod max;
pub mod min;
pub mod pending;
pub mod rbe;
pub mod rbe_error;
pub mod rbe_pretty_printer;
pub mod rbe_schema;
pub mod rbe_table;
pub mod values;

pub use crate::cardinality::*;
pub use crate::component::*;
pub use crate::deriv_n::*;
pub use crate::empty_iter::*;
pub use crate::failures::*;
pub use crate::keys::*;
pub use crate::match_cond::*;
pub use crate::max::*;
pub use crate::min::*;
pub use crate::pending::*;
pub use crate::rbe_error::*;
pub use crate::rbe_pretty_printer::*;
pub use crate::rbe_schema::*;
pub use crate::rbe_table::*;
pub use crate::rbe1::*;
pub use crate::rbe1_matcher::*;
pub use crate::values::*;

// We may remove the following
pub mod rbe1;
pub mod rbe1_matcher;
// pub use crate::rbe::*;
// pub use crate::deriv_error::*;
pub use crate::bag::*;
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// The `Key`trait represents the type of keys that can be used in the conditions. It must implement `Eq`, `Hash`, `Debug`, `Default`, `Display`, and `Clone`.
pub trait Key: Eq + Hash + Debug + Default + Display + Clone {}

/// The `Value` trait represents the type of values that can be used in the conditions. It must implement `Eq`, `Hash`, `Debug`, `Default`, `Display`, and `Clone`.
pub trait Value: Eq + Hash + Debug + Default + Display + Clone {}

/// The `Ref` trait represents the type of references that can be used in the conditions. It must implement `Eq`, `Hash`, `Debug`, `Default`, `Display`, and `Clone`.
pub trait Ref: Eq + Hash + Debug + Default + Display + Clone {}

/// The `Context` trait represents the type of contexts that can be used in the conditions. It must implement `Eq`, `Hash`, `Debug`, `Default`, `Display`, and `Clone`.
pub trait Context: Eq + Hash + Debug + Default + Display + Clone {}

/// The `State` trait represents the type of states that can be used in the conditions. It must implement `Eq`, `Hash`, `Debug`, `Default`, `Display`, and `Clone`.
pub trait State: Eq + Hash + Debug + Default + Display + Clone {}

/// A no-op state type for when no state is needed.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NoState;

impl State for NoState {}

impl std::fmt::Display for NoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
