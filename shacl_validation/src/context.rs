use shacl_ast::compiled::component::Component;
use shacl_ast::compiled::shape::Shape;
use srdf::SRDFBasic;

/// The context is an auxilary data structure that enables the creation of
/// detailed Validation Results.
pub struct Context<'a, S: SRDFBasic> {
    component: &'a Component<S>,
    shape: &'a Shape<S>,
}

impl<'a, S: SRDFBasic> Context<'a, S> {
    pub fn new(component: &'a Component<S>, shape: &'a Shape<S>) -> Self {
        Self { component, shape }
    }

    pub fn component(&self) -> &Component<S> {
        self.component
    }

    pub(crate) fn shape(&self) -> S::Term {
        // match self.shape {
        //     Shape::NodeShape(ns) => S::object_as_term(ns.id()),
        //     Shape::PropertyShape(ps) => S::object_as_term(ps.id()),
        // }
        todo!()
    }

    pub fn source_constraint_component(&self) -> S::Term {
        // S::iri_s2term(self.component)
        todo!()
    }

    pub(crate) fn result_severity(&self) -> Option<S::Term> {
        // let severity = match self.shape {
        //     Shape::NodeShape(ns) => ns.severity(),
        //     Shape::PropertyShape(ps) => ps.severity(),
        // };
        // severity.map(|severity| S::iri_s2term(&severity.to_owned().into()))
        todo!()
    }
}
