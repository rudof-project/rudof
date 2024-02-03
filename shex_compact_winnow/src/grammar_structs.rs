use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{SemAct, ShapeExpr, ShapeExprLabel};

#[derive(Debug, PartialEq)]
pub(crate) enum ShExStatement<'a> {
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
    StartActions {
        actions: Vec<SemAct>,
    },
    StartDecl {
        shape_expr: ShapeExpr,
    },
    ShapeDecl {
        is_abstract: bool,
        shape_label: ShapeExprLabel,
        shape_expr: ShapeExpr,
    },
    Empty
}

impl<'a> ShExStatement<'a> {
    pub fn is_empty(&self) -> bool {
       match self {
        ShExStatement::Empty => true,
        _ => false
       }
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum Qualifier {
    Closed,
    Extra(Vec<IriRef>),
    Extends(ShapeExprLabel),
}

#[derive(PartialEq, Debug)]
pub(crate) struct Cardinality {
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

    pub fn exact(n: i32) -> Cardinality {
        Cardinality {
            min: Some(n),
            max: Some(n),
        }
    }

    pub fn only_min(n: i32) -> Cardinality {
        Cardinality {
            min: Some(n),
            max: None,
        }
    }

    pub fn min_max(min: i32, max: i32) -> Cardinality {
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

pub(crate) enum NumericLength {
    TotalDigits,
    FractionDigits,
}

pub(crate) enum NumericRange {
    MinInclusive,
    MinExclusive,
    MaxInclusive,
    MaxExclusive,
}

pub(crate) struct SenseFlags {
    pub(crate) inverse: Option<bool>,
    pub(crate) negated: Option<bool>,
}

impl SenseFlags {
    pub(crate) fn extract(&self) -> (Option<bool>, Option<bool>) {
        (self.negated, self.inverse)
    }
}
