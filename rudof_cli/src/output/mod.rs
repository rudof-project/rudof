mod writer;
mod color;

pub use writer::get_writer;
pub use color::{ColorSupport, detect_color_support_cached};