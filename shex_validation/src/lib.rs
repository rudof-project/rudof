mod result_map;
mod result_value;
// mod validation_state;
pub mod validator;
pub mod validator_error;

pub use crate::result_map::*;
pub use crate::validator::*;
pub use crate::validator_error::*;
pub use crate::result_value::*;

// pub mod validation_error;
// pub mod cardinality_error;
// pub mod cardinality;

// pub use crate::cardinality_error::*;
// pub use crate::cardinality::*;
// pub use crate::validation_error::*;

const MAX_STEPS: usize = 20;
