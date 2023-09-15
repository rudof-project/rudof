mod result_map;
mod result_value;
// mod validation_state;
pub mod solver;
pub mod validator;
pub mod validator_error;
pub mod validator_runner;

pub use crate::result_map::*;
pub use crate::result_value::*;
pub use crate::solver::*;
pub use crate::validator::*;
pub use crate::validator_error::*;
pub use crate::validator_runner::*;

// pub mod validation_error;
// pub mod cardinality_error;
// pub mod cardinality;

// pub use crate::cardinality_error::*;
// pub use crate::cardinality::*;
// pub use crate::validation_error::*;

/// Default MAX STEPS
/// This value can be overriden in the Validator configuration
const MAX_STEPS: usize = 20;

type Result<T> = std::result::Result<T, ValidatorError>;
