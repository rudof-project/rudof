use compiled_shacl_error::CompiledShaclError;
use prefixmap::IriRef;
use shape::CompiledShape;
use srdf::Object;
use srdf::Rdf;

use shacl_ast::value::Value;
use shacl_ast::Schema;

pub mod compiled_shacl_error;
pub mod component;
pub mod node_shape;
pub mod property_shape;
pub mod schema;
pub mod severity;
pub mod shape;
pub mod target;

fn convert_iri_ref<S: Rdf>(iri_ref: IriRef) -> Result<S::IRI, CompiledShaclError> {
    let iri = iri_ref
        .get_iri()
        .map_err(|_| CompiledShaclError::IriRefConversion)?
        .into();
    Ok(iri)
}

fn compile_shape<S: Rdf>(
    shape: Object,
    schema: &Schema,
) -> Result<CompiledShape<S>, CompiledShaclError> {
    let shape = schema
        .get_shape(&shape)
        .ok_or(CompiledShaclError::ShapeNotFound)?;
    CompiledShape::compile(shape.to_owned(), schema)
}

fn compile_shapes<S: Rdf>(
    shapes: Vec<Object>,
    schema: &Schema,
) -> Result<Vec<CompiledShape<S>>, CompiledShaclError> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<S>(shape, schema))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}

fn convert_value<S: Rdf>(value: Value) -> Result<S::Term, CompiledShaclError> {
    let ans = match value {
        Value::Iri(iri_ref) => {
            let iri = convert_iri_ref::<S>(iri_ref)?;
            let term: S::Term = <S::Term as From<S::IRI>>::from(iri);
            term
        }
        Value::Literal(literal) => {
            let literal: S::Literal = literal.into();
            let term: S::Term = <S::Term as From<S::Literal>>::from(literal);
            term
        }
    };
    Ok(ans)
}
