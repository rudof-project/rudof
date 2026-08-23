mod color;
mod error_display;
mod writer;

pub use color::{ColorSupport, detect_color_support_cached, error_prefix};
pub use error_display::format_error;
pub use writer::get_writer;
