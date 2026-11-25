mod prefix_header;
#[expect(clippy::module_inception)]
mod utils;
mod wasm_stubs;

pub(crate) use prefix_header::{PrefixDirective, default_prefix_header};
pub use utils::*;
#[allow(unused_imports, reason = "the stubs are used on wasm only")]
pub use wasm_stubs::*;
