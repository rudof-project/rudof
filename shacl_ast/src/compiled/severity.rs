use srdf::SRDFBasic;

#[derive(Hash, PartialEq, Eq)]
pub enum Severity<S: SRDFBasic> {
    Violation,
    Warning,
    Info,
    Generic(S::IRI),
}
