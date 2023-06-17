use shex_ast::Schema ;
use srdf::{SRDF} ;

struct FixedShapeMap {
}

struct ResultShapeMap {
    
}

enum ValidationError {

}

fn validate<D>(sm: FixedShapeMap, schema: Schema, data: D) -> Result<ResultShapeMap, ValidationError> 
where D: SRDF {
    todo!()
}

fn validate_node<D,I>(node: D::IRI, schema: Schema, data: D) -> Result<ResultShapeMap, ValidationError> 
  where D: SRDF,
 {
    todo!()
}
