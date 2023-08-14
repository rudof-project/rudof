use std::collections::HashMap;

use shex_ast::{CompiledSchema, Iri, ShapeLabel};
use srdf::{Object, SRDF, SRDFComparisons};
use thiserror::Error;

type Result<T> = std::result::Result<T, ValidatorError>;

pub struct Validator {
    schema: CompiledSchema
}


impl Validator {
    
    pub fn validate_node_shape<S>(&self, 
        node: Object, 
        shape: ShapeLabel, 
        rdf: S) -> Result<ResultMap> 
        where 
        S: SRDF 
    {
        let mut result_map = ResultMap::new();
        let term = Self::get_rdf_node::<S>(&node)?;
        if let Some(se) = self.schema.find_label(&shape) {
           result_map.insert(node, shape);
           Ok(result_map)
        } else {
            Err(ValidatorError::NotFoundShapeLabel{shape})
        }
    }

    pub fn get_rdf_node<S>(node: &Object) -> Result<S::Term> 
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
    }
}

pub struct ResultMap{
  result_map: HashMap<Object, ShapeLabel>
}

impl ResultMap {
    pub fn new() -> ResultMap {
        ResultMap { result_map: HashMap::new() }
    }

    pub fn insert(&mut self, node: Object, shape: ShapeLabel) {
        self.result_map.insert(node, shape);
    }
}

#[derive(Error, Debug)]
pub enum ValidatorError {

    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel{ shape: ShapeLabel }
}

