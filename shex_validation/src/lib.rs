use shapemap::{ShapeMap, ShapeMapState};
use shex_ast::compiled_schema::{ShapeExpr, TripleExpr};
use shex_ast::{CompiledSchema, CompiledSchemaError, SchemaJson, ShapeLabel};
use srdf::{IriS, SRDF};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError<'a, SL>
where
    SL: Debug,
{
    #[error("ShapeLabel not found {shape_label:?} Labels: {existing_labels:?}")]
    LabelNotFoundError {
        shape_label: &'a SL,
        existing_labels: Vec<&'a SL>,
    },
    #[error("Converting Json String: {str:?}")]
    FromJsonStr { str: String, err: String },

    #[error("Compiling schema: {error:?}")]
    CompilingSchema { error: CompiledSchemaError },

    #[error("SRDF Error {error:?}")]
    SRDFError { error: String },
}

struct Validator<SL> {
    schema: CompiledSchema<SL>,
}

impl<SL> Validator<SL>
where
    SL: Eq + Hash + Debug + FromStr,
{
    fn new(schema: CompiledSchema<SL>) -> Validator<SL> {
        Validator { schema: schema }
    }

    fn from_json_str(json_str: String) -> Result<Validator<SL>, ValidationError<'static, SL>> {
        match serde_json::from_str::<SchemaJson>(json_str.as_str()) {
            Ok(schema_json) => {
                let schema = CompiledSchema::from_schema_json(schema_json).map_err(|e| todo!())?;
                Ok(Validator::new(schema))
            }
            Err(e) => Err(ValidationError::FromJsonStr {
                str: json_str,
                err: e.to_string(),
            }),
        }
    }

    async fn check_node_shape_label<'a, SM, G>(
        &'a self,
        node: &'a SM::NodeIdx,
        shape_label: &'a SL,
        shape_map: SM,
        graph: G,
    ) -> Result<SM, ValidationError<'a, SL>>
    where
        G: SRDF,
        SM: ShapeMap<'a, NodeIdx = G::Term, ShapeIdx = SL> + 'a,
    {
        match shape_map.state(&node, shape_label) {
            ShapeMapState::Conforms => Ok(shape_map),
            ShapeMapState::Fails => Ok(shape_map),
            ShapeMapState::Inconsistent => Ok(shape_map),
            ShapeMapState::Unknown => match self.schema.find_label(shape_label) {
                None => Err(ValidationError::LabelNotFoundError {
                    shape_label,
                    existing_labels: self.schema.existing_labels(),
                }),
                Some(shape_expr) => {
                    self.check_node_shape_expr(node, shape_expr, shape_map, graph)
                        .await
                }
            },
            ShapeMapState::Pending { pairs } => match self.schema.find_label(shape_label) {
                None => Err(ValidationError::LabelNotFoundError {
                    shape_label,
                    existing_labels: self.schema.existing_labels(),
                }),
                Some(shape_expr) => {
                    self.check_node_shape_expr(node, shape_expr, shape_map, graph)
                        .await
                }
            },
        }
    }

    async fn check_node_shape_expr<'a, SM, G>(
        &'a self,
        node: &SM::NodeIdx,
        shape_expr: &ShapeExpr,
        shape_map: SM,
        graph: G,
    ) -> Result<SM, ValidationError<'a, SL>>
    where
        G: SRDF,
        SM: ShapeMap<'a, ShapeIdx = SL, NodeIdx = G::Term> + 'a,
    {
        match shape_expr {
            ShapeExpr::Shape {
                expression: Some(triple_expr),
                ..
            } => {
                self.check_node_triple_expr(node, triple_expr, shape_map, graph)
                    .await
            }
            _ => todo!(),
        }
    }

    async fn check_node_triple_expr<'a, SM, G>(
        &'a self,
        node: &SM::NodeIdx,
        triple_expr: &TripleExpr,
        shape_map: SM,
        graph: G,
    ) -> Result<SM, ValidationError<'a, SL>>
    where
        G: SRDF,
        SM: ShapeMap<'a, NodeIdx = G::Term, ShapeIdx = SL> + 'a,
    {
        match triple_expr {
            TripleExpr::TripleConstraint {
                id,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                if let Some(subject) = graph.term_as_subject(node) {
                    let os = graph
                        .get_objects_for_subject_predicate(
                            &subject,
                            &graph.iri_from_str(predicate.to_string()),
                        )
                        .await;
                    todo!();
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use iri_s::*;
    use oxrdf::*;
    use shapemap_oxgraph::*;
    use srdf_oxgraph::SRDFGraph;

    #[tokio::test]
    async fn test_not_found_label() {
        let str = r#"{
            "@context": "http://www.w3.org/ns/shex.jsonld",
            "type": "Schema",
            "shapes": [
                {
                    "type": "ShapeDecl",
                    "id": "http://a.example/S1",
                    "shapeExpr": {
                        "type": "Shape",
                        "expression": {
                            "type": "TripleConstraint",
                            "predicate": "http://a.example/p1"
                        }
                    }
                }
            ]
        }"#;
        let validator: Validator<ShapeLabelOxGraph> =
            Validator::from_json_str(str.to_string()).unwrap();

        let rdf_str = r#"
        prefix : <http://a.example/>

        :x :p1 :y .
        "#;
        let graph = SRDFGraph::from_str(rdf_str.to_string()).unwrap();
        let node = Term::NamedNode(NamedNode::new_unchecked("http://a.example/x"));
        let shape_label: shapemap_oxgraph::ShapeLabelOxGraph =
            shapemap_oxgraph::ShapeLabelOxGraph::Iri(NamedNode::new_unchecked(
                "http://a.example/S1",
            ));
        let shape_map = ShapeMapOxGraph::new();
        let result: ShapeMapOxGraph = validator
            .check_node_shape_label(&node, &shape_label, shape_map, graph)
            .await
            .unwrap();
        let conforms: ShapeMapState<'_, oxrdf::Term, ShapeLabelOxGraph> = ShapeMapState::Conforms;
        let r: &ShapeMapState<'_, oxrdf::Term, ShapeLabelOxGraph> =
            result.state(&node, &shape_label);
        assert_eq!(r, &conforms);
    }
}
