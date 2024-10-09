//! ShEx validation
//!
//!
mod result_map;
mod result_value;
// mod validation_state;
pub mod atom;
pub mod reason;
pub mod rule;
pub mod schema_without_imports;
pub mod schema_without_imports_error;
pub mod shex_config;
pub mod shex_format;
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
pub use crate::schema_without_imports::*;
pub use crate::schema_without_imports_error::*;
pub use crate::shex_config::*;
pub use crate::shex_format::*;
pub use crate::validator::*;
pub use crate::validator_config::*;
pub use crate::validator_error::*;
pub use crate::validator_runner::*;

/// Default MAX STEPS
/// This value can be overriden in the Validator configuration
const MAX_STEPS: usize = 20;

/// Method employed to resolve imports when ghessing the format of an import
#[derive(Debug, Clone)]
pub enum ResolveMethod {
    RotatingFormats(Vec<ShExFormat>),
    ByGuessingExtension,
    ByContentNegotiation,
}

impl Default for ResolveMethod {
    fn default() -> Self {
        ResolveMethod::RotatingFormats(vec![ShExFormat::ShExC, ShExFormat::ShExJ])
    }
}
