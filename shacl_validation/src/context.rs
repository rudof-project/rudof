use shacl_ast::component::Component;
use shacl_ast::shape::Shape;
use srdf::SRDFBasic;

pub struct Context {
    component: Component,
    shape: Shape,
}

impl Context {
    pub fn new(component: &Component, shape: Shape) -> Self {
        Self {
            component: component.to_owned(),
            shape,
        }
    }

    pub fn component(&self) -> &Component {
        &self.component
    }

    pub fn source_constraint_component<S: SRDFBasic>(&self) -> S::Term {
        S::iri_s2term(&self.component.clone().into())
    }

    pub(crate) fn result_severity<S: SRDFBasic>(&self) -> Option<S::Term> {
        let severity = match &self.shape {
            Shape::NodeShape(ns) => ns.severity(),
            Shape::PropertyShape(ps) => ps.severity(),
        };
        severity.map(|severity| S::iri_s2term(&severity.to_owned().into()))
    }

    pub(crate) fn source_shape<S: SRDFBasic>(&self) -> Option<S::Term> {
        let focus_node = match &self.shape {
            Shape::NodeShape(ns) => S::object_as_term(&ns.id()),
            Shape::PropertyShape(ps) => S::object_as_term(ps.id()),
        };
        Some(focus_node)
    }
}
