//! ShEx validation
//!
//!
pub mod atom;
pub mod engine;
pub mod k_partitions;
pub mod partition;
pub mod reason;
pub mod reasons;
mod result_value;
pub mod rule;
pub mod schema_without_imports;
pub mod schema_without_imports_error;
pub mod shex_config;
pub mod shex_format;
pub mod solver;
pub mod typing;
pub mod validator;
pub mod validator_config;
pub mod validator_error;
pub mod validator_errors;

pub use crate::atom::*;
pub use crate::engine::*;
pub use crate::k_partitions::*;
pub use crate::partition::*;
pub use crate::reason::*;
pub use crate::reasons::*;
pub use crate::result_value::*;
pub use crate::rule::*;
pub use crate::schema_without_imports::*;
pub use crate::schema_without_imports_error::*;
pub use crate::shex_config::*;
pub use crate::typing::*;
pub use crate::validator::*;
pub use crate::validator_config::*;
pub use crate::validator_error::*;
pub use crate::validator_errors::*;

/// Default MAX STEPS
/// This value can be overriden in the Validator configuration
const MAX_STEPS: usize = 20;
