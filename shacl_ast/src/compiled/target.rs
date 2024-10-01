use srdf::SRDFBasic;

#[derive(Hash, PartialEq, Eq)]
pub enum Target<S: SRDFBasic> {
    TargetNode(S::Term),
    TargetClass(S::Term),
    TargetSubjectsOf(S::IRI),
    TargetObjectsOf(S::IRI),
}
