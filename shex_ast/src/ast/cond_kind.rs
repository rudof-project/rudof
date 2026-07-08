use crate::{
    CResult, Node, Pred, SchemaIRError, ShapeLabelIdx,
    ast::NodeKind,
    ir::{semantic_action_context::SemanticActionContext, value_set::ValueSet},
};
use rbe::{MatchKind, Pending, RbeError};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::{
    Object,
    literal::{ConcreteLiteral, NumericLiteral},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

/// Closed enumeration of every matching primitive the ShEx compiler installs on a `SingleCond`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CondKind {
    Any,
    NodeKind(NodeKind),
    Datatype {
        iri: IriS,
        qualified: String,
    },
    Length(usize),
    MinLength(usize),
    MaxLength(usize),
    Pattern {
        regex: String,
        flags: Option<String>,
        base: Option<IriS>,
    },
    MinInclusive(NumericLiteral),
    MinExclusive(NumericLiteral),
    MaxInclusive(NumericLiteral),
    MaxExclusive(NumericLiteral),
    TotalDigits(usize),
    FractionDigits(usize),
    ValueSet(ValueSet),
    /// The registry is looked up from the `SemanticActionContext` at match time.
    /// The enum carries only the action IRI and its inline code so the whole variant stays serializable.
    SemAct {
        name: IriS,
        code: Option<String>,
    },
}

impl MatchKind<Pred, Node, ShapeLabelIdx, SemanticActionContext> for CondKind {
    fn eval(
        &self,
        v: &Node,
        ctx: &SemanticActionContext,
    ) -> Result<
        Pending<Pred, Node, ShapeLabelIdx>,
        RbeError<Pred, Node, ShapeLabelIdx, SemanticActionContext, Self>,
    > {
        let empty = || Ok(Pending::empty());
        let error = |msg: String| Err(RbeError::MsgError { msg });

        match self {
            CondKind::Any => empty(),
            CondKind::NodeKind(nk) => match check_node_node_kind(v, nk) {
                Ok(_) => empty(),
                Err(e) => error(format!("NodeKind error: {e:?}")),
            },
            CondKind::Datatype { iri, .. } => match check_node_datatype(v, iri) {
                Ok(_) => empty(),
                Err(e) => error(format!("Datatype error: {e:?}")),
            },
            CondKind::Length(n) => match check_node_length(v, *n) {
                Ok(_) => empty(),
                Err(e) => error(format!("Length error: {e:?}")),
            },
            CondKind::MinLength(n) => match check_node_min_length(v, *n) {
                Ok(_) => empty(),
                Err(e) => error(format!("MinLength error: {e:?}")),
            },
            CondKind::MaxLength(n) => match check_node_max_length(v, *n) {
                Ok(_) => empty(),
                Err(e) => error(format!("MaxLength error: {e:?}")),
            },
            CondKind::Pattern { regex, flags, base } => {
                match check_pattern(v, regex, flags.as_deref(), base) {
                    Ok(_) => empty(),
                    Err(e) => error(format!("Pattern error: {e:?}")),
                }
            },
            CondKind::MinInclusive(n) => match check_node_min_inclusive(v, n.clone()) {
                Ok(_) => empty(),
                Err(e) => error(format!("MinInclusive error: {e:?}")),
            },
            CondKind::MinExclusive(n) => match check_node_min_exclusive(v, n.clone()) {
                Ok(_) => empty(),
                Err(e) => error(format!("MinExclusive error: {e:?}")),
            },
            CondKind::MaxInclusive(n) => match check_node_max_inclusive(v, n.clone()) {
                Ok(_) => empty(),
                Err(e) => error(format!("MaxInclusive error: {e:?}")),
            },
            CondKind::MaxExclusive(n) => match check_node_max_exclusive(v, n.clone()) {
                Ok(_) => empty(),
                Err(e) => error(format!("MaxExclusive error: {e:?}")),
            },
            CondKind::TotalDigits(n) => match check_node_total_digits(v, *n) {
                Ok(_) => empty(),
                Err(e) => error(format!("TotalDigits error: {e:?}")),
            },
            CondKind::FractionDigits(n) => match check_node_fraction_digits(v, *n) {
                Ok(_) => empty(),
                Err(e) => error(format!("FractionDigits error: {e:?}")),
            },
            CondKind::ValueSet(vs) => {
                if vs.check_value(v.as_object()) {
                    empty()
                } else {
                    error(format!("Value {} not in {vs:?}", v.as_object()))
                }
            },
            CondKind::SemAct { name, code } => match ctx.registry() {
                Some(reg) => match reg.run_action(name, code.as_deref(), ctx) {
                    Ok(()) => empty(),
                    Err(e) => error(format!("Semantic action error for {name}: {e}")),
                },
                None => error(format!(
                    "Semantic action {name}: no SemanticActionsRegistry attached to context"
                )),
            },
        }
    }
}

pub(crate) fn check_pattern(
    node: &Node,
    regex: &str,
    flags: Option<&str>,
    base: &Option<IriS>,
) -> CResult<()> {
    trace!("check_pattern: node: {node}, regex: {regex}, flags: {flags:?}, base: {base:?}");
    let lexical_form = match node.as_object() {
        Object::Literal(lit) => Ok(lit.lexical_form()),
        Object::BlankNode(b) => Ok(b.clone()),
        Object::Iri(iri) => Ok(iri.to_string()),
        Object::Triple { .. } => Err(Box::new(SchemaIRError::PatternTripleTerm {
            node: node.to_string(),
            regex: regex.to_string(),
            flags: flags.map(|f| f.to_string()),
        })),
    }?;
    let effective_regex = match flags {
        Some(f) if !f.is_empty() => format!("(?{f}){regex}"),
        _ => regex.to_string(),
    };
    if let Ok(re) = regex::Regex::new(&effective_regex) {
        if re.is_match(&lexical_form) {
            Ok(())
        } else {
            Err(Box::new(SchemaIRError::PatternError {
                regex: regex.to_string(),
                flags: flags.unwrap_or("").to_string(),
                lexical_form: lexical_form.clone(),
            }))
        }
    } else {
        Err(Box::new(SchemaIRError::InvalidRegex {
            regex: regex.to_string(),
        }))
    }
}

pub(crate) fn check_node_node_kind(node: &Node, nk: &NodeKind) -> CResult<()> {
    match (nk, node.as_object()) {
        (NodeKind::Iri, Object::Iri { .. }) => Ok(()),
        (NodeKind::Iri, _) => Err(Box::new(SchemaIRError::NodeKindIri { node: node.clone() })),
        (NodeKind::BNode, Object::BlankNode(_)) => Ok(()),
        (NodeKind::BNode, _) => Err(Box::new(SchemaIRError::NodeKindBNode { node: node.clone() })),
        (NodeKind::Literal, Object::Literal(_)) => Ok(()),
        (NodeKind::Literal, _) => Err(Box::new(SchemaIRError::NodeKindLiteral { node: node.clone() })),
        (NodeKind::NonLiteral, Object::BlankNode(_)) => Ok(()),
        (NodeKind::NonLiteral, Object::Iri { .. }) => Ok(()),
        (NodeKind::NonLiteral, _) => Err(Box::new(SchemaIRError::NodeKindNonLiteral { node: node.clone() })),
    }
}

pub(crate) fn check_node_datatype(node: &Node, dt: &IriS) -> CResult<()> {
    let object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_datatype: as_checked_object error: {e}"),
        })
    })?;
    match object {
        Object::Literal(sliteral) => check_literal_datatype(&sliteral, dt, node),
        Object::Iri(_) | Object::BlankNode(_) | Object::Triple { .. } => {
            Err(Box::new(SchemaIRError::DatatypeNoLiteral {
                expected: Box::new(dt.clone()),
                node: Box::new(node.clone()),
            }))
        },
    }
}

fn check_literal_datatype(sliteral: &ConcreteLiteral, expected: &IriS, node: &Node) -> CResult<()> {
    let checked_literal = sliteral.clone().into_checked_literal().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_literal_datatype: into_checked_literal error: {e}"),
        })
    })?;
    match checked_literal {
        ConcreteLiteral::WrongDatatypeLiteral {
            lexical_form,
            datatype,
            error,
        } => Err(Box::new(SchemaIRError::WrongDatatypeLiteralMatch {
            datatype: datatype.clone(),
            error: error.clone(),
            expected: expected.clone(),
            lexical_form: lexical_form.to_string(),
        })),
        _ => {
            let node_dt = checked_literal.datatype();
            let node_dt_iri = node_dt.get_iri().map_err(|e| {
                Box::new(SchemaIRError::CheckLiteralDatatypeCnvIriRef2IriError {
                    iri_ref: node_dt.clone(),
                    error: e.to_string(),
                })
            })?;
            if node_dt_iri == expected {
                Ok(())
            } else {
                Err(Box::new(SchemaIRError::DatatypeDontMatch {
                    expected: expected.clone(),
                    found: node_dt,
                    lexical_form: node.to_string(),
                }))
            }
        },
    }
}

pub(crate) fn check_node_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_length: {node:?} length: {len}");
    let node_length = node.length();
    if node_length == len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::LengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}

pub(crate) fn check_node_min_inclusive(node: &Node, min: NumericLiteral) -> CResult<()> {
    trace!("check_node_min_inclusive: {node:?} min_inclusive: {min}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_min_inclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_inclusive: as_numeric error".to_string(),
        })
    })?;
    if !node_num.less_than(&min) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinInclusiveError {
            expected: min.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_min_exclusive(node: &Node, min: NumericLiteral) -> CResult<()> {
    trace!("check_node_min_exclusive: {node:?} min_exclusive: {min}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_min_exclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_exclusive: as_numeric error".to_string(),
        })
    })?;
    if !node_num.less_than_or_eq(&min) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinExclusiveError {
            expected: min.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_total_digits(node: &Node, total: usize) -> CResult<()> {
    trace!("check_node_total_digits: {node:?} total: {total}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_total_digits: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_total_digits: as_numeric error".to_string(),
        })
    })?;
    if let Some(num_digits) = node_num.total_digits() {
        if num_digits <= total {
            Ok(())
        } else {
            Err(Box::new(SchemaIRError::TotalDigitsError {
                expected: total,
                found: node_num,
                node: node.to_string(),
            }))
        }
    } else {
        Err(Box::new(SchemaIRError::TotalDigitsError {
            expected: total,
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_fraction_digits(node: &Node, fd: usize) -> CResult<()> {
    trace!("check_node_fraction_digits: {node:?} total: {fd}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_fraction_digits: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_fraction_digits: as_numeric error".to_string(),
        })
    })?;
    if let Some(num_fd) = node_num.fraction_digits() {
        if num_fd <= fd {
            Ok(())
        } else {
            Err(Box::new(SchemaIRError::FractionDigitsError {
                expected: fd,
                found: node_num,
                node: node.to_string(),
            }))
        }
    } else {
        Err(Box::new(SchemaIRError::FractionDigitsError {
            expected: fd,
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_max_exclusive(node: &Node, max: NumericLiteral) -> CResult<()> {
    trace!("check_node_max_exclusive: {node:?} max_exclusive: {max:?}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_max_exclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_exclusive: as_numeric error".to_string(),
        })
    })?;
    if node_num.less_than(&max) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinExclusiveError {
            expected: max.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_max_inclusive(node: &Node, max: NumericLiteral) -> CResult<()> {
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_max_inclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_max_inclusive: as_numeric error".to_string(),
        })
    })?;
    if node_num.less_than_or_eq(&max) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MaxInclusiveError {
            expected: max.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

pub(crate) fn check_node_min_length(node: &Node, len: usize) -> CResult<()> {
    let node_length = node.length();
    if node_length >= len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}

pub(crate) fn check_node_max_length(node: &Node, len: usize) -> CResult<()> {
    let node_length = node.length();
    if node_length <= len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MaxLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}
