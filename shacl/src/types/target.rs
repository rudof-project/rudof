use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::fmt::{Display, Formatter};

/// Represents target declarations
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Target {
    Node(Object), // TODO - Replace with node expr
    Class(Object),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(Object),

    // The following target declaration are not well-formed, but we keep them
    // to generate violation errors for them
    WrongNode(Object),
    WrongClass(Object),
    WrongSubjectsOf(Object),
    WrongObjectsOf(Object),
    WrongImplicitClass(Object),
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Node(o) => write!(f, "targetNode({o})"),
            Target::Class(o) => write!(f, "targetClass({o})"),
            Target::SubjectsOf(iri) => write!(f, "targetSubjectsOf({iri})"),
            Target::ObjectsOf(iri) => write!(f, "targetObjectsOf({iri})"),
            Target::ImplicitClass(o) => write!(f, "targetImplicitClass({o})"),
            Target::WrongNode(o) => write!(f, "targetNode({o})"),
            Target::WrongClass(o) => write!(f, "targetClass({o})"),
            Target::WrongSubjectsOf(iri) => write!(f, "targetSubjectsOf({iri})"),
            Target::WrongObjectsOf(iri) => write!(f, "targetObjectsOf({iri})"),
            Target::WrongImplicitClass(o) => write!(f, "targetImplicitClass({o})"),
        }
    }
}
