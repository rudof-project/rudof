//! Regular Bag Expressions (rbe)
//! 
//! Provides an implementation of Regular Bag Expressions which are the expressions internally employed by 
//! the implementations of Shape Expressions
//! More information about Regular Bag Expressions:
//! 
//! [Complexity and Expressiveness of ShEx for RDF](https://labra.weso.es/publication/2015_complexityexpressivenessshexrdf/)
//! S. Staworko, I. Boneva, J. Labra, S. Hym, E. Prud'hommeaux, H. Solbrig
//! 
pub mod cardinality;
pub mod min;
pub mod max;

pub mod rbe_matcher;
pub mod rbe;
pub mod rbe_error;
pub mod match_cond;
pub mod pending;
pub mod deriv_n;
pub mod failures;

pub use crate::cardinality::*;
pub use crate::max::*;
pub use crate::min::*;
pub use crate::rbe::*;
pub use crate::rbe_error::*;
pub use crate::rbe_matcher::*;
pub use crate::match_cond::*;
pub use crate::pending::*;
pub use crate::deriv_n::*;
pub use crate::failures::*;

// We may remove the following
pub mod bag;
pub mod rbe0;
pub mod rbe0_error;
pub use crate::rbe0::*;
pub use crate::rbe0_error::*;
pub use crate::bag::*;


        