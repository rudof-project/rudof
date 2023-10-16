use iri_s::IriS;
use shex_ast::{ShapeExpr, ShapeLabel};

#[derive(Debug, PartialEq)]
pub enum ShExStatement<'a> {
    PrefixDecl {
        alias: &'a str,
        iri: IriS,
    },
    BaseDecl {
        iri: IriS,
    },
    ImportDecl {
        iri: IriS,
    },
    StartDecl {
        shape_expr: ShapeExpr,
    },
    ShapeDecl {
        shape_label: ShapeLabel,
        shape_expr: ShapeExpr,
    },
}
