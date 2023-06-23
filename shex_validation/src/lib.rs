use shapemap::ShapeMap;
use shex_ast::Schema;
use srdf::SRDF;

enum ValidationError {}

fn validate<'a, SM, D>(sm: SM, schema: Schema, data: D) -> Result<SM, ValidationError>
where
    D: SRDF,
    SM: ShapeMap<'a>,
{
    todo!()
}

fn validate_node<'a, D, SM, I>(
    node: D::IRI,
    schema: Schema,
    data: D,
    shape_map: SM,
) -> Result<SM, ValidationError>
where
    D: SRDF,
    SM: ShapeMap<'a>,
{
    todo!()
}
