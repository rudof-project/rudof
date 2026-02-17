mod arrow_style;
mod stereotype_style;
#[allow(clippy::module_inception)]
mod style;
mod thickness_style;
mod uml_color;

pub use arrow_style::{
    ArrowStyle, DEFAULT_OBJECT_ARROW_STYLE, DEFAULT_PREDICATE_ARROW_STYLE, DEFAULT_SUBJECT_ARROW_STYLE,
};
pub use stereotype_style::StereotypeStyle;
pub use style::Style;
pub use thickness_style::ThicknessStyle;
pub use uml_color::UmlColor;
