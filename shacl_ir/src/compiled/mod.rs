use compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use prefixmap::IriRef;
use shape::CompiledShape;
use srdf::Object;
use srdf::RDFNode;
use srdf::Rdf;

use shacl_ast::Schema;
use shacl_ast::value::Value;

pub mod compiled_shacl_error;
pub mod component;
pub mod node_shape;
pub mod property_shape;
pub mod schema;
pub mod severity;
pub mod shape;
pub mod target;

fn convert_iri_ref(iri_ref: IriRef) -> Result<IriS, CompiledShaclError> {
    let iri = iri_ref
        .get_iri()
        .map_err(|_| CompiledShaclError::IriRefConversion)?;
    Ok(iri)
}

fn compile_shape<S: Rdf>(
    shape: Object,
    schema: &Schema<S>,
) -> Result<CompiledShape, CompiledShaclError> {
    let shape = schema
        .get_shape(&shape)
        .ok_or(CompiledShaclError::ShapeNotFound)?;
    CompiledShape::compile(shape.to_owned(), schema)
}

fn compile_shapes<S: Rdf>(
    shapes: Vec<Object>,
    schema: &Schema<S>,
) -> Result<Vec<CompiledShape>, CompiledShaclError> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<S>(shape, schema))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}

fn convert_value(value: Value) -> Result<RDFNode, CompiledShaclError> {
    let ans = match value {
        Value::Iri(iri_ref) => {
            let iri = convert_iri_ref(iri_ref)?;

            RDFNode::iri(iri)
        }
        Value::Literal(literal) => RDFNode::literal(literal),
    };
    Ok(ans)
}
