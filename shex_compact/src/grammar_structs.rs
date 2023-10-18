use iri_s::IriS;
use shex_ast::{IriRef, Ref, ShapeExpr};

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
        shape_label: Ref,
        shape_expr: ShapeExpr,
    },
}

#[derive(PartialEq, Debug)]
pub enum Qualifier {
    Closed,
    Extra(Vec<IriRef>),
}

pub struct Cardinality {
    min: Option<i32>,
    max: Option<i32>,
}

impl Cardinality {
    pub fn plus() -> Cardinality {
        Cardinality {
            min: Some(1),
            max: Some(-1),
        }
    }

    pub fn star() -> Cardinality {
        Cardinality {
            min: Some(0),
            max: Some(-1),
        }
    }

    pub fn optional() -> Cardinality {
        Cardinality {
            min: Some(0),
            max: Some(1),
        }
    }

    pub fn range(min: i32, max: i32) -> Cardinality {
        Cardinality {
            min: Some(min),
            max: Some(max),
        }
    }

    pub fn min(&self) -> Option<i32> {
        self.min
    }

    pub fn max(&self) -> Option<i32> {
        self.max
    }
}

impl Default for Cardinality {
    fn default() -> Self {
        Self {
            min: Some(1),
            max: Some(1),
        }
    }
}
