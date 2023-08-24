use crate::result_map::*;
use crate::validation_state::*;
use crate::validator_error::*;
use iri_s::IriS;
use shex_ast::compiled_schema::*;
use shex_ast::ShapeLabelIdx;
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
    state: ValidationState<Object, ShapeLabelIdx>,
}

impl Validator {
    pub fn new(schema: CompiledSchema) -> Validator {
        Validator {
            schema,
            state: ValidationState::new(),
        }
    }

    fn add_current(&mut self, node: &Object, shape: &ShapeLabelIdx) {
        self.state.set_current_goal(&node, &shape);
    }

    fn add_pending(&mut self, node: Object, shape: ShapeLabelIdx) {
        self.state.add_pending(node, shape)
    }

    fn more_pending(&self) -> bool {
        self.state.more_pending()
    }

    fn no_end_steps(&self) -> bool {
        self.state.steps() < self.state.max_steps()
    }

    fn pop_pending(&mut self) -> Result<(Object, ShapeLabelIdx)> {
        match self.state.pop_pending() {
            Some((n, s)) => Ok((n, s)),
            None => {
                // Raise internal error
                todo!()
            }
        }
    }

    pub fn validate_node_shape<S>(&mut self, node: Object, shape: ShapeLabel, rdf: &S) -> Result<()>
    where
        S: SRDF,
    {
        if let Some((idx, _se)) = self.schema.find_label(&shape) {
            self.add_pending(node, *idx);
            while self.no_end_steps() && self.more_pending() {
                let (n, s) = self.pop_pending()?;
                self.add_current(&n, &s);
                self.check_node_shape(&n, &s, rdf)?;
            }
            Ok(())
        } else {
            Err(ValidatorError::NotFoundShapeLabel { shape })
        }
    }

    fn find_shape_expr<'a>(idx: &'a ShapeLabelIdx, schema: &'a CompiledSchema) -> &'a ShapeExpr {
        let se = schema.get_shape_expr(idx).unwrap();
        se
    }

    pub fn check_node_shape<S>(
        &mut self,
        node: &Object,
        shape: &ShapeLabelIdx,
        rdf: &S,
    ) -> Result<()>
    where
        S: SRDF,
    {
        todo!()
        //let se = self.schema.get_shape_expr(shape).unwrap(); // Self::find_shape_expr(shape, &self.schema);
        //self.check_node_shape_expr(node, se, rdf)
    }

    fn check_node_shape_expr<S>(&mut self, node: &Object, se: &ShapeExpr, rdf: &S) -> Result<()>
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
                let values = self.neighs(node, rdf)?;
                let rs = rbe_table.matches(values);
                Ok(())
            }
            ShapeExpr::Empty => Ok(()),
            ShapeExpr::ShapeExternal {} => {
                todo!()
            }
        }
    }

    fn cnv_iri<S>(&self, iri: S::IRI) -> IriS
    where
        S: SRDF,
    {
        S::iri2iri_s(iri)
    }

    fn cnv_object<S>(&self, term: S::Term) -> Object
    where
        S: SRDF,
    {
        S::term2object(term)
    }

    fn neighs<S>(&self, node: &Object, rdf: &S) -> Result<Vec<(IriS, Object)>>
    where
        S: SRDF,
    {
        let node = self.get_rdf_node(&node, rdf);
        if let Some(subject) = rdf.term_as_subject(&node) {
            let preds = rdf
                .get_predicates_for_subject(&subject)
                .map_err(|e| self.cnv_err::<S>(e))?;
            let mut result = Vec::new();
            for p in preds {
                let objects = rdf
                    .get_objects_for_subject_predicate(&subject, &p)
                    .map_err(|e| self.cnv_err::<S>(e))?;
                for o in objects {
                    let object = self.cnv_object::<S>(o);
                    let iri = self.cnv_iri::<S>(p.clone());
                    result.push((iri, object));
                }
            }
            Ok(result)
        } else {
            todo!()
        }
    }

    fn cnv_err<S>(&self, err: S::Err) -> ValidatorError
    where
        S: SRDF,
    {
        todo!()
    }

    fn get_rdf_node<S>(&self, node: &Object, rdf: &S) -> S::Term
    where
        S: SRDF,
    {
        match node {
            Object::Iri { iri } => {
                let i = S::iri_s2iri(iri);
                S::iri_as_term((*i).clone())
            }
            Object::BlankNode(id) => {
                todo!()
            }
            Object::Literal(lit) => {
                todo!()
            }
        }
    }

    /*pub fn result_map(&self) -> ResultMap<Object, ShapeLabelIdx> {
        self.validation_state.result_map.clone()
    }*/
}
