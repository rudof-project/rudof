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
use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::Schema;
use shacl_ast::value::Value;
use shape::ShapeIR;
use srdf::Object;
use srdf::RDFNode;
use srdf::Rdf;

use crate::dependency_graph::PosNeg;
use crate::schema_ir::SchemaIR;
use crate::shape_label_idx::ShapeLabelIdx;

/// Type alias for dependencies: a vector of (PosNeg, ShapeLabelIdx) pairs
type Deps = Vec<(PosNeg, ShapeLabelIdx)>;

fn convert_iri_ref(iri_ref: IriRef) -> Result<IriS, Box<CompiledShaclError>> {
    let iri = iri_ref.get_iri().map_err(|err| {
        Box::new(CompiledShaclError::IriRefConversion {
            iri_ref: iri_ref.to_string(),
            err: err.to_string(),
        })
    })?;
    Ok(iri)
}

fn compile_shape<S: Rdf>(
    node: &Object,
    schema: &Schema<S>,
    schema_ir: &mut SchemaIR,
) -> Result<(ShapeLabelIdx, Deps), Box<CompiledShaclError>> {
    let shape = schema
        .get_shape(node)
        .ok_or(CompiledShaclError::ShapeNotFound {
            shape: Box::new(node.clone()),
        })?;
    let idx = schema_ir.add_shape_idx(node.clone())?;
    ShapeIR::compile(shape.to_owned(), schema, &idx, schema_ir)
}

fn compile_shapes<S: Rdf>(
    shapes: Vec<Object>,
    schema: &Schema<S>,
    schema_ir: &mut SchemaIR,
) -> Result<Vec<(ShapeLabelIdx, Deps)>, Box<CompiledShaclError>> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<S>(&shape, schema, schema_ir))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}

fn convert_value(value: Value) -> Result<RDFNode, Box<CompiledShaclError>> {
    let ans = match value {
        Value::Iri(iri_ref) => {
            let iri = convert_iri_ref(iri_ref)?;

            RDFNode::iri(iri)
        }
        Value::Literal(literal) => RDFNode::literal(literal),
    };
    Ok(ans)
}
