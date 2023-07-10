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
        shape_map: &'a mut SM,
        graph: &G,
    ) -> Result<&SM, ValidationError<'a, SL>>
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
        shape_map: &'a mut SM,
        graph: &G,
    ) -> Result<&SM, ValidationError<'a, SL>>
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
        shape_map: &'a mut SM,
        graph: &G,
    ) -> Result<&SM, ValidationError<'a, SL>>
    where
        G: SRDF,
        SM: ShapeMap<'a, NodeIdx = G::Term, ShapeIdx = SL> + 'a,
    {
        match triple_expr {
            TripleExpr::TripleConstraint {
                id: _,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                if let Some(subject) = graph.term_as_subject(node) {
                    println!(
                        "Before obtaining objects for subject {} and predicate {}",
                        subject, predicate
                    );
                    let os = graph
                        .get_objects_for_subject_predicate(
                            &subject,
                            &graph.iri_from_str(predicate.to_string()),
                        )
                        .await
                        .map_err(|e| ValidationError::SRDFError {
                            error: format!(
                                "Obtaining objects for {} and predicate {}",
                                subject, predicate
                            ),
                        })?;
                    let ps = graph.get_predicates_subject(&subject).await.map_err(|e| {
                        ValidationError::SRDFError {
                            error: format!("Obtaining predicates for {}", subject),
                        }
                    })?;
                    println!("Result of predicates: {:?}", ps.len());
                    /*  if let Some(value_expr) = value_expr {
                        for object in os {
                            let result = self
                                .check_node_shape_expr(&object, value_expr, shape_map, graph)
                                .await?;
                        }
                    } */
                    if self.check_cardinality(os.len(), min, max) {
                        Ok(shape_map)
                    } else {
                        println!(
                            "Cardinality failed: {} min {:?} max {:?}",
                            os.len(),
                            min,
                            max
                        );
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn check_cardinality(&self, c: usize, min: &Option<i32>, max: &Option<i32>) -> bool {
        let min = min.unwrap_or(1);
        if c < min.try_into().unwrap() {
            return false;
        }
        let max = max.unwrap_or(1);
        if max == -1 {
            return true;
        }
        if c > max.try_into().unwrap() {
            return false;
        }
        true
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
    async fn test_simple() {
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
        // Debug...
        let x = NamedNode::new_unchecked("http://a.example/x".to_string());
        let subject = Subject::NamedNode(x);
        let pred = NamedNode::new_unchecked("http://a.example/p1".to_string());
        let os = graph
            .get_objects_for_subject_predicate(&subject, &pred)
            .await
            .unwrap();
        println!("Result of objects for subject predicate: {:?}", os);
        // end debug
        let shape_label: shapemap_oxgraph::ShapeLabelOxGraph =
            shapemap_oxgraph::ShapeLabelOxGraph::Iri(NamedNode::new_unchecked(
                "http://a.example/S1",
            ));
        let mut shape_map = ShapeMapOxGraph::new();
        let node = Term::NamedNode(NamedNode::new_unchecked("http://a.example/x".to_string()));
        let result: &ShapeMapOxGraph = validator
            .check_node_shape_label(&node, &shape_label, &mut shape_map, &graph)
            .await
            .unwrap();
        let conforms: ShapeMapState<'_, oxrdf::Term, ShapeLabelOxGraph> = ShapeMapState::Conforms;
        let r: &ShapeMapState<'_, oxrdf::Term, ShapeLabelOxGraph> =
            result.state(&node, &shape_label);
        assert_eq!(r, &conforms);
    }
}
