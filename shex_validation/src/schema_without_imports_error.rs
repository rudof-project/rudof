use iri_s::{IriS, IriSError};
use shex_ast::{ShapeExpr, ShapeExprLabel};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SchemaWithoutImportsError {
    #[error("Obtaining schema from IRI {iri}. Tried to parse this list of formats: {formats} but they failed")]
    SchemaFromIriRotatingFormats { iri: IriS, formats: String },

    #[error("Dereferencing IRI {iri}. Error: {error}")]
    DereferencingIri { iri: IriS, error: String },

    #[error("ShExC error {error}. String: {content}")]
    ShExCError { error: String, content: String },

    #[error("ShExJ error at IRI: {iri}. Error: {error}")]
    ShExJError { iri: IriS, error: String },

    #[error("Duplicated declaration for shape expr with label {label}\nPrevious shape expr from {imported_from:?}\n{old_shape_expr:?}\nShape Expr2 {shape_expr2:?}")]
    DuplicatedShapeDecl {
        label: ShapeExprLabel,
        old_shape_expr: Box<ShapeExpr>,
        imported_from: IriS,
        shape_expr2: Box<ShapeExpr>,
    },

    #[error("Resolving string: {str} as IRI with base: {base}")]
    ResolvingStrIri {
        str: String,
        base: IriS,
        error: IriSError,
    },
}
