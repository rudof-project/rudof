use prefixmap::IriRef;

#[derive(Debug, Clone)]
pub enum Severity {
    Violation,
    Warning,
    Info,
    Generic(IriRef)
}