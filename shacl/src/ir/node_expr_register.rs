use crate::ir::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{BuildRDF, SHACLPath};
use std::collections::HashMap;

pub(crate) type Shapes = HashMap<ShapeLabelIdx, IRShape>;

pub(crate) struct NodeExprRegister<'a, RDF: BuildRDF> {
    graph: &'a mut RDF,
    shapes: &'a Shapes,
    subject: RDF::Subject,
}

impl<'a, RDF: BuildRDF> NodeExprRegister<'a, RDF> {
    pub(crate) fn new(graph: &'a mut RDF, shapes: &'a Shapes) -> Result<Self, IRError> {
        let subject: RDF::Subject = graph
            .add_bnode()
            .map_err(|e| IRError::from_rdf_err::<RDF>("add a node expression blank node", e))?
            .into();

        Ok(Self { graph, shapes, subject })
    }

    /// The term that denotes the expression written on it.
    pub(crate) fn term(&self) -> RDF::Term {
        self.subject.clone().into()
    }

    pub(crate) fn arc(&mut self, property: IriS, value: RDF::Term) -> Result<(), IRError> {
        self.graph
            .add_triple(self.subject.clone(), property, value)
            .map_err(IRError::add_triple::<RDF>)
    }

    pub(crate) fn iri(&mut self, property: IriS, value: &IriS) -> Result<(), IRError> {
        self.arc(property, value.clone().into())
    }

    pub(crate) fn string(&mut self, property: IriS, value: &str) -> Result<(), IRError> {
        let literal: RDF::Literal = value.to_string().into();
        self.arc(property, literal.into())
    }

    pub(crate) fn integer(&mut self, property: IriS, value: usize) -> Result<(), IRError> {
        let literal: RDF::Literal = (value as i128).into();
        self.arc(property, literal.into())
    }

    pub(crate) fn boolean(&mut self, property: IriS, value: bool) -> Result<(), IRError> {
        let literal: RDF::Literal = value.into();
        self.arc(property, literal.into())
    }

    /// Writes a SHACL path, as `shnex:pathValues` takes one.
    pub(crate) fn path(&mut self, property: IriS, path: &SHACLPath) -> Result<(), IRError> {
        let path = self
            .graph
            .add_shacl_path(path)
            .map_err(|e| IRError::from_rdf_err::<RDF>("add a node expression path", e))?;
        self.arc(property, path)
    }

    /// The shape `idx` denotes, by its id.
    pub(crate) fn shape(&mut self, property: IriS, idx: &ShapeLabelIdx) -> Result<(), IRError> {
        let shape = self.shapes.get(idx).ok_or(IRError::ShapeNotFound(*idx))?;
        let id = RDF::object_as_term(shape.id());
        self.arc(property, id)
    }

    pub(crate) fn expr(&mut self, property: IriS, expr: &IRNodeExpr) -> Result<(), IRError> {
        let value = expr.register(self.graph, self.shapes)?;
        self.arc(property, value)
    }

    pub(crate) fn optional_expr(&mut self, property: IriS, expr: Option<&IRNodeExpr>) -> Result<(), IRError> {
        match expr {
            None => Ok(()),
            Some(expr) => self.expr(property, expr),
        }
    }

    /// The `shnex:nodes` the expression is applied to.
    pub(crate) fn nodes(&mut self, expr: &IRNodeExpr) -> Result<(), IRError> {
        self.expr(ShaclNodeExprVocab::shnex_nodes(), expr)
    }

    pub(crate) fn expr_list<'e>(
        &mut self,
        property: IriS,
        exprs: impl IntoIterator<Item = &'e IRNodeExpr>,
    ) -> Result<(), IRError> {
        let mut items = Vec::new();
        for expr in exprs {
            items.push(expr.register(self.graph, self.shapes)?);
        }

        let head = self
            .graph
            .add_rdf_list(items)
            .map_err(|e| IRError::from_rdf_err::<RDF>("add a node expression argument list", e))?;
        self.arc(property, head)
    }
}
