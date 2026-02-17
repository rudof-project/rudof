mod lang;
#[allow(clippy::module_inception)]
mod literal;
mod numeric_literal;
mod xsd_datetime;

pub use lang::{Lang, LangParseError};
pub use literal::{ConcreteLiteral, Literal};
pub use numeric_literal::NumericLiteral;
pub use xsd_datetime::{XsdDateTime, XsdDateTimeParseError};

#[cfg(test)]
mod tests {
    mod literal_tests;
    mod numeric_literal_tests;
}
