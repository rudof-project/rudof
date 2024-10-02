use compiled_shacl_error::CompiledShaclError;
use prefixmap::IriRef;
use shape::CompiledShape;
use srdf::lang::Lang;
use srdf::literal::Literal;
use srdf::Object;
use srdf::RDFNode;
use srdf::SRDFBasic;

use crate::value::Value;
use crate::Schema;

pub mod compiled_shacl_error;
pub mod component;
pub mod node_shape;
pub mod property_shape;
pub mod schema;
pub mod severity;
pub mod shape;
pub mod target;

fn convert_iri_ref<S: SRDFBasic>(iri_ref: IriRef) -> Result<S::IRI, CompiledShaclError> {
    let iri_s = iri_ref
        .get_iri()
        .map_err(|_| CompiledShaclError::IriRefConversion)?;
    Ok(S::iri_s2iri(&iri_s))
}

fn convert_lang<S: SRDFBasic>(lang: Lang) -> Result<S::Literal, CompiledShaclError> {
    let object = RDFNode::literal(Literal::str(&lang.value()));
    let term = S::object_as_term(&object);
    S::term_as_literal(&term).ok_or(CompiledShaclError::LiteralConversion)
}

fn compile_shape<S: SRDFBasic>(
    shape: Object,
    schema: &Schema,
) -> Result<CompiledShape<S>, CompiledShaclError> {
    let shape = schema
        .get_shape(&shape)
        .ok_or(CompiledShaclError::ShapeNotFound)?;
    CompiledShape::compile(shape.to_owned(), schema)
}

fn compile_shapes<S: SRDFBasic>(
    shapes: Vec<Object>,
    schema: &Schema,
) -> Result<Vec<CompiledShape<S>>, CompiledShaclError> {
    let compiled_shapes = shapes
        .into_iter()
        .map(|shape| compile_shape::<S>(shape, schema))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(compiled_shapes)
}

fn convert_value<S: SRDFBasic>(value: Value) -> Result<S::Term, CompiledShaclError> {
    let ans = match value {
        Value::Iri(iri_ref) => {
            let iri_ref = convert_iri_ref::<S>(iri_ref)?;
            S::iri_as_term(iri_ref)
        }
        Value::Literal(literal) => S::object_as_term(&RDFNode::literal(literal)),
    };
    Ok(ans)
}
