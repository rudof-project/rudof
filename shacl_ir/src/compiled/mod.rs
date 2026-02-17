pub mod closed_info;
pub mod compiled_shacl_error;
pub mod component_ir;
pub mod dependency_graph;
pub mod node_shape;
pub mod property_shape;
pub mod reifier_info;
pub mod schema_ir;
pub mod severity;
pub mod shape;
pub mod shape_label_idx;
pub mod target;

use compiled_shacl_error::CompiledShaclError;
use either::Either;
use iri_s::IriS;
use prefixmap::IriRef;
use rdf::rdf_core::{Rdf, term::Object};
use shacl_ast::Schema;
use shacl_ast::value::Value;
use shape::ShapeIR;
use tracing::trace;

use crate::schema_ir::SchemaIR;
use crate::shape_label_idx::ShapeLabelIdx;

fn convert_iri_ref(iri_ref: IriRef) -> Result<IriS, Box<CompiledShaclError>> {
    let iri = iri_ref.get_iri().map_err(|err| {
        Box::new(CompiledShaclError::IriRefConversion {
            iri_ref: iri_ref.to_string(),
            err: err.to_string(),
        })
    })?;
    Ok(iri.clone())
}

fn compile_shape<S: Rdf>(
    node: &Object,
    schema: &Schema<S>,
    schema_ir: &mut SchemaIR,
) -> Result<ShapeLabelIdx, Box<CompiledShaclError>> {
    let shape = schema.get_shape(node).ok_or(CompiledShaclError::ShapeNotFound {
        shape: Box::new(node.clone()),
    })?;
    match schema_ir.add_shape_idx(node.clone())? {
        Either::Right(idx) => {
            trace!("Compiling shape {:?} with index {:?}", node, idx);
            ShapeIR::compile(shape.to_owned(), schema, &idx, schema_ir)
        },
        Either::Left(idx) => {
            trace!("Shape {:?} already compiled, skipping recompilation", node);
            Ok(idx)
        },
    }
}

fn compile_shapes<S: Rdf>(
    shapes: Vec<Object>,
    schema: &Schema<S>,
    schema_ir: &mut SchemaIR,
) -> Result<Vec<ShapeLabelIdx>, Box<CompiledShaclError>> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<S>(&shape, schema, schema_ir))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}

fn convert_value(value: Value) -> Result<Object, Box<CompiledShaclError>> {
    let ans = match value {
        Value::Iri(iri_ref) => {
            let iri = convert_iri_ref(iri_ref)?;

            Object::iri(iri)
        },
        Value::Literal(literal) => Object::literal(literal),
    };
    Ok(ans)
}
