use crate::{
    edge::Edge, edge_label_property_spec::EdgeLabelPropertySpec, evidence::Evidence, formal_base_type::FormalBaseType,
    label_property_spec::LabelPropertySpec, pg::PropertyGraph, pgs::PropertyGraphSchema, pgs_error::PgsError,
};
use either::Either;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct EdgeType {
    pub source: LabelPropertySpec,
    pub edge: EdgeLabelPropertySpec,
    pub target: LabelPropertySpec,
}
impl EdgeType {
    pub fn new(source: LabelPropertySpec, edge: EdgeLabelPropertySpec, target: LabelPropertySpec) -> Self {
        EdgeType { source, edge, target }
    }

    pub fn semantics(&self, schema: &PropertyGraphSchema) -> Result<EdgeSemantics, PgsError> {
        let source = self.source.semantics(schema)?;
        let edge = self.edge.semantics(schema)?;
        let target = self.target.semantics(schema)?;
        Ok(EdgeSemantics { source, edge, target })
    }
}

impl Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeType(({})-[{}]->({}))", self.source, self.edge, self.target)
    }
}

pub struct EdgeSemantics {
    pub source: FormalBaseType,
    pub edge: FormalBaseType,
    pub target: FormalBaseType,
}

impl EdgeSemantics {
    pub fn conforms_edge(
        &self,
        _type_name: &str,
        edge: &Edge,
        graph: &PropertyGraph,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        let source = edge.source();
        let source_node = match graph.node(source) {
            Some(node) => node,
            None => return Either::Left(vec![PgsError::MissingNodeId { id: source.to_string() }]),
        };
        let edge_labels = edge.labels();
        let edge_content = edge.content();
        let target = edge.target();
        let target_node = match graph.node(target) {
            Some(node) => node,
            None => return Either::Left(vec![PgsError::MissingNodeId { id: target.to_string() }]),
        };

        let source_conforms = self.source.conforms(source_node.labels(), source_node.content());
        let target_conforms = self.target.conforms(target_node.labels(), target_node.content());
        let edge_conforms = self.edge.conforms(edge_labels, edge_content);
        let results = vec![source_conforms, edge_conforms, target_conforms];
        results.into_iter().fold(Either::Right(Vec::new()), |acc, res| {
            match (acc, res) {
                // Both succeeded: combine the evidences
                (Either::Right(mut acc_ev), Either::Right(new_ev)) => {
                    acc_ev.extend(new_ev);
                    Either::Right(acc_ev)
                },
                // First error encountered: start tracking errors
                (Either::Right(_), Either::Left(errs)) => Either::Left(errs),
                // Already tracking errors, new one encountered: combine errors
                (Either::Left(mut acc_errs), Either::Left(new_errs)) => {
                    acc_errs.extend(new_errs);
                    Either::Left(acc_errs)
                },
                // Already failed, ignore subsequent successes
                (Either::Left(acc_errs), Either::Right(_)) => Either::Left(acc_errs),
            }
        })
    }
}

impl Display for EdgeSemantics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EdgeSemantics(Source: {}, Edge: {}, Target: {})",
            self.source, self.edge, self.target
        )
    }
}
