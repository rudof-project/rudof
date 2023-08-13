use shex_ast::{CompiledSchema, Iri, ShapeLabel};
use srdf::{Object, SRDF, SRDFComparisons};
use thiserror::Error;

type Result<T> = std::result::Result<T, ValidatorError>;

pub struct Validator {
    schema: CompiledSchema
}


impl Validator {
    
    pub fn validate_node_shape<S>(
        node: Object, 
        shape: ShapeLabel, 
        rdf: S) -> Result<ResultMap> 
        where 
        S: SRDF 
    {
        todo!()
    }

    pub fn get_rdf_node<S>(node: Object) -> Result<S as SRDF::Term> 
    where S: SRDF + SRDFComparisons {
        match node {
           Object::Iri { iri} => {
              
           },
           Object::BlankNode(id) => { todo!() },
           Object::Literal(lit) => { todo!() }
        }
    }
}

pub struct ResultMap{

}

#[derive(Error, Debug)]
pub enum ValidatorError {

}