use crate::result_map::*;
use crate::validator_error::*;
use iri_s::IriS;
use shex_ast::compiled_schema::*;
use shex_ast::{compiled_schema::CompiledSchema, ShapeLabel};
use srdf::NeighsIterator;
use srdf::{Object, SRDFComparisons, SRDF};
use std::collections::HashSet;
use std::hash::Hash;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

type Result<T> = std::result::Result<T, ValidatorError>;

pub struct Validator {
    schema: CompiledSchema,
    result_map: ResultMap,
}

impl Validator {
    pub fn new(schema: CompiledSchema) -> Validator {
        Validator {
            schema,
            result_map: ResultMap::new(),
        }
    }

    pub fn validate_node_shape<S>(&mut self, node: Object, shape: ShapeLabel, rdf: S) -> Result<()>
    where
        S: SRDF,
    {
        if let Some((idx, se)) = self.schema.find_label(&shape) {
            self.result_map.insert(node, *idx);
            Ok(())
        } else {
            Err(ValidatorError::NotFoundShapeLabel { shape })
        }
    }

    pub fn validate_node_shape_expr<S>(
        &mut self,
        node: Object,
        se: &ShapeExpr,
        rdf: S,
    ) -> Result<()>
    where
        S: SRDF,
    {
        match se {
            ShapeExpr::NodeConstraint {
                node_kind,
                datatype,
                xs_facet,
                values,
            } => {
                todo!()
            }
            ShapeExpr::Ref { idx } => {
                todo!()
            }
            ShapeExpr::ShapeAnd { exprs } => {
                todo!()
            }
            ShapeExpr::ShapeNot { expr } => {
                todo!()
            }
            ShapeExpr::ShapeOr { exprs } => {
                todo!()
            }
            ShapeExpr::Shape {
                closed,
                extra,
                rbe_table,
                sem_acts,
                annotations,
            } => {
                let values = self.neighs(&node, &rdf)?;
                let rs = rbe_table.matches(values);
                Ok(())
            }
            ShapeExpr::Empty => Ok(()),
            ShapeExpr::ShapeExternal {} => {
                todo!()
            }
        }
    }

    fn cnv_iri<S>(&self, iri: &S::IRI) -> IriS
    where
        S: SRDF,
    {
        todo!()
    }

    fn cnv_object<S>(&self, term: &S::Term) -> Object
    where
        S: SRDF,
    {
        todo!()
    }

    fn neighs<S>(&self, node: &Object, rdf: &S) -> Result<Vec<(IriS, Object)>>
    where
        S: SRDF,
    {
        let node = self.get_rdf_node(node, rdf)?;
        if let Some(subject) = rdf.term_as_subject(&node) {
            let preds = rdf
                .get_predicates_for_subject(&subject)
                .map_err(|e| todo!())?;
            let mut result = Vec::new();
            for p in preds {
                let objects = rdf
                    .get_objects_for_subject_predicate(&subject, &p)
                    .map_err(|e| todo!())?;
                for o in objects {
                    let object = self.cnv_object::<S>(&o);
                    let iri = self.cnv_iri::<S>(&p);
                    result.push((iri, object));
                }
            }
            Ok(result)
        } else {
            todo!()
        }
    }

    fn get_rdf_node<S>(&self, node: &Object, rdf: &S) -> Result<S::Term>
    where
        S: SRDF,
    {
        match node {
            Object::Iri { iri } => {
                let iri = S::iri_from_str(iri.as_str()).map_err(|e| ValidatorError::SRDFError {
                    error: format!("{e}"),
                })?;
                Ok(S::iri_as_term(iri))
            }
            Object::BlankNode(id) => {
                todo!()
            }
            Object::Literal(lit) => {
                todo!()
            }
        }
    }

    pub fn result_map(&self) -> ResultMap {
        self.result_map.clone()
    }
}
