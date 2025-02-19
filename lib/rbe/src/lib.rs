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
pub mod deriv_error;
pub mod keys;
pub mod max;
pub mod min;
pub mod rbe;
pub mod values;

pub mod component;
pub mod deriv_n;
pub mod failures;
pub mod match_cond;
pub mod pending;
pub mod rbe_error;
pub mod rbe_schema;
pub mod rbe_table;

pub use crate::cardinality::*;
pub use crate::component::*;
pub use crate::deriv_n::*;
pub use crate::failures::*;
pub use crate::keys::*;
pub use crate::match_cond::*;
pub use crate::max::*;
pub use crate::min::*;
pub use crate::pending::*;
pub use crate::rbe1::*;
pub use crate::rbe1_matcher::*;
pub use crate::rbe_error::*;
pub use crate::rbe_schema::*;
pub use crate::rbe_table::*;
pub use crate::values::*;

// We may remove the following
pub mod rbe1;
pub mod rbe1_matcher;
// pub use crate::rbe::*;
// pub use crate::deriv_error::*;
pub use crate::bag::*;
use core::hash::Hash;
use std::fmt::{Debug, Display};

pub trait Key: Eq + Hash + Debug + Default + Display + Clone {}
pub trait Value: Eq + Hash + Debug + Default + Display + Clone {}

pub trait Ref: Eq + Hash + Debug + Default + Display + Clone {}
