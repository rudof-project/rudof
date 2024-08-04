//! ShEx validation
//!
//!
mod result_map;
mod result_value;
// mod validation_state;
pub mod atom;
pub mod reason;
pub mod rule;
pub mod solver;
pub mod validator;
pub mod validator_config;
pub mod validator_error;
pub mod validator_runner;

pub use crate::atom::*;
pub use crate::reason::*;
pub use crate::result_map::*;
pub use crate::result_value::*;
pub use crate::rule::*;
pub use crate::validator::*;
pub use crate::validator_config::*;
pub use crate::validator_error::*;
pub use crate::validator_runner::*;

/// Default MAX STEPS
/// This value can be overriden in the Validator configuration
const MAX_STEPS: usize = 20;
