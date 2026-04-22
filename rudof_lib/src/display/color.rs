use colored::Color as C;
use shacl::types::Severity;

pub trait Color {
    fn color(&self) -> C;
}

impl Color for Severity {
    fn color(&self) -> C {
        match self {
            Severity::Trace => C::Cyan,
            Severity::Debug => C::Magenta,
            Severity::Info => C::Blue,
            Severity::Warning => C::Yellow,
            Severity::Violation => C::Red,
            Severity::Generic(_) => C::White,
        }
    }
}
