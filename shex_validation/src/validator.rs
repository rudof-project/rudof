use crate::validator_error::*;
use crate::result_map::*;
use std::{collections::HashMap, fmt::{Display, Formatter}};
use iri_s::IriS;
use shex_ast::{compiled_schema::CompiledSchema, ShapeLabel};
use shex_ast::compiled_schema::*;
use srdf::{Object, SRDF, SRDFComparisons};
use std::hash::Hash;

type Result<T> = std::result::Result<T, ValidatorError>;

pub struct Validator 
{
    schema: CompiledSchema,
    result_map: ResultMap
}


impl Validator
{

    pub fn new(schema: CompiledSchema) -> Validator {
        Validator {
            schema,
            result_map: ResultMap::new()
        }
    }
    
    pub fn validate_node_shape<S> (&mut self, 
        node: Object, 
        shape: ShapeLabel, 
        rdf: S) -> Result<()> 
        where 
        S: SRDF 
    {
        if let Some((idx, se)) = self.schema.find_label(&shape) {
           self.result_map.insert(node, *idx);
           Ok(())
        } else {
            Err(ValidatorError::NotFoundShapeLabel{shape})
        }
    }

    

    pub fn validate_node_shape_expr<S>(&mut self, 
        node: Object, 
        se: &ShapeExpr, 
        rdf: S) -> Result<()> 
        where 
        S: SRDF 
    {
        match se {
            ShapeExpr::NodeConstraint { node_kind, datatype, xs_facet, values } => {
                todo!()
            },
            ShapeExpr::Ref { idx } => {
                todo!()
            },
            ShapeExpr::ShapeAnd { exprs } => {
                todo!()
            },
            ShapeExpr::ShapeNot { expr } => {
                todo!()
            },
            ShapeExpr::ShapeOr { exprs } => {
                todo!()
            },
            ShapeExpr::Shape { closed, extra, rbe_table, sem_acts, annotations }  => {
                let values = self.neighs(node, rdf)?;
                let rs = rbe_table.matches(values).collect();
                Ok(())
            },
            ShapeExpr::Empty => {
                Ok(())
            },
            ShapeExpr::ShapeExternal {  } => {
                todo!()
            }
        }
    }

    fn neighs<S>(&self, node: Object, rdf: S) -> Result<Vec<(IriS, Object)>> {
        todo!()
    }

    /*pub fn get_rdf_node<S>(node: &Object) -> Result<S::Term> 
     where S: SRDF + SRDFComparisons {
        match node {
           Object::Iri { iri} => {
            let iri = S::iri_from_str(iri.as_str()).map_err(|e| {  
                ValidatorError::SRDFError { error: format!("{e}") }
            })?;
            Ok(S::iri_as_term(iri))
           },
           Object::BlankNode(id) => { todo!() },
           Object::Literal(lit) => { todo!() }
        }
    }*/

    pub fn result_map(&self) -> ResultMap {
       self.result_map.clone()
    }
}


