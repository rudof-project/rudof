use shex_ast::Schema ;
use srdf::SRDF ;

struct FixedShapeMap {

}

struct ResultShapeMap {

}

enum ValidationError {
    
}

fn validate<D: SRDF>(sm: FixedShapeMap, schema: Schema, data: D) -> Result<ResultShapeMap, ValidationError> {
    todo!()
}
