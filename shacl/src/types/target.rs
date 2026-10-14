use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::fmt::{Display, Formatter};

/// Represents target declarations
#[derive(Debug, Clone, PartialEq)]
///
/// Now Target::Node depends on the compilation stage.
/// You can check both [`ASTTarget`](crate::ast::ASTTarget) and [`IRTarget`](crate::ir::IRTarget)
pub enum Target<E> {
    Node(E),
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

impl<E: Display> Display for Target<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Node(e) => write!(f, "targetNode({e})"),
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
