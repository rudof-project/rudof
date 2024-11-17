use compiled_shacl_error::CompiledShaclError;
use shape::CompiledShape;
use srdf::model::rdf::TObject;
use srdf::model::rdf::Rdf;

use crate::Schema;

pub mod compiled_shacl_error;
pub mod component;
pub mod node_shape;
pub mod property_shape;
pub mod schema;
pub mod shape;

fn compile_shape<R: Rdf + Clone>(
    shape: TObject<R>,
    schema: &Schema<R>,
) -> Result<CompiledShape<R>, CompiledShaclError> {
    let shape = schema
        .get_shape(&shape)
        .ok_or(CompiledShaclError::ShapeNotFound)?;
    CompiledShape::compile(shape.clone(), schema)
}

fn compile_shapes<R: Rdf + Clone>(
    shapes: Vec<TObject<R>>,
    schema: &Schema<R>,
) -> Result<Vec<CompiledShape<R>>, CompiledShaclError> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<R>(shape, schema))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}
