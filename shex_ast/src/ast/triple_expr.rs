use std::{result, str::FromStr};

use iri_s::{IriS, IriSError};
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use serde::{Serialize as SerializeTrait, Serializer};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::ast::serde_string_or_struct::*;

use super::{
    annotation::Annotation, sem_act::SemAct, shape_expr::ShapeExpr,
    triple_expr_label::TripleExprLabel,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum TripleExpr {
    EachOf {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,

        expressions: Vec<TripleExprWrapper>,

        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<i32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<i32>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },

    OneOf {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,

        expressions: Vec<TripleExprWrapper>,

        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<i32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<i32>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },
    TripleConstraint {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<TripleExprLabel>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        negated: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        inverse: Option<bool>,

        predicate: IriRef,

        #[serde(
            default,
            rename = "valueExpr",
            skip_serializing_if = "Option::is_none",
            serialize_with = "serialize_opt_box_string_or_struct",
            deserialize_with = "deserialize_opt_box_string_or_struct"
        )]
        value_expr: Option<Box<ShapeExpr>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<i32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<i32>,

        #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
        sem_acts: Option<Vec<SemAct>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },

    TripleExprRef(TripleExprLabel),
}

impl TripleExpr {
    pub fn triple_constraint(
        negated: Option<bool>,
        inverse: Option<bool>,
        predicate: IriRef,
        se: Option<ShapeExpr>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> TripleExpr {
        TripleExpr::TripleConstraint {
            id: None,
            negated,
            inverse,
            predicate,
            value_expr: se.map(|se| Box::new(se)),
            min,
            max,
            sem_acts: None,
            annotations: None,
        }
    }

    pub fn each_of(tes: Vec<TripleExpr>) -> TripleExpr {
        let mut tews = Vec::new();
        for te in tes {
            tews.push(te.into())
        }
        TripleExpr::EachOf {
            id: None,
            expressions: tews,
            min: None,
            max: None,
            sem_acts: None,
            annotations: None,
        }
    }

    pub fn one_of(tes: Vec<TripleExpr>) -> TripleExpr {
        let mut tews = Vec::new();
        for te in tes {
            tews.push(te.into())
        }
        TripleExpr::OneOf {
            id: None,
            expressions: tews,
            min: None,
            max: None,
            sem_acts: None,
            annotations: None,
        }
    }

    pub fn with_id(mut self, id: Option<TripleExprLabel>) -> Self {
        self = match self {
            TripleExpr::EachOf {
                id: _,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::OneOf {
                id: _,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleConstraint {
                id: _,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleExprRef(lbl) => {
                panic!("Can't update id to TripleExprRef({lbl:?}")
            }
        };
        self
    }

    pub fn with_min(mut self, new_min: Option<i32>) -> Self {
        self = match self {
            TripleExpr::EachOf {
                id,
                expressions,
                min: _,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::EachOf {
                id,
                expressions,
                min: new_min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::OneOf {
                id,
                expressions,
                min: _,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::OneOf {
                id,
                expressions,
                min: new_min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min: _,
                max,
                sem_acts,
                annotations,
            } => TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min: new_min,
                max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleExprRef(lbl) => {
                panic!("Can't update min to TripleExprRef({lbl:?}")
            }
        };
        self
    }

    pub fn with_max(mut self, new_max: Option<i32>) -> Self {
        self = match self {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max: _,
                sem_acts,
                annotations,
            } => TripleExpr::EachOf {
                id,
                expressions,
                min,
                max: new_max,
                sem_acts,
                annotations,
            },
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max: _,
                sem_acts,
                annotations,
            } => TripleExpr::OneOf {
                id,
                expressions,
                min,
                max: new_max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max: _,
                sem_acts,
                annotations,
            } => TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max: new_max,
                sem_acts,
                annotations,
            },
            TripleExpr::TripleExprRef(lbl) => {
                panic!("Can't update max to TripleExprRef({lbl:?}")
            }
        };
        self
    }

    pub fn with_sem_acts(mut self, new_sem_acts: Option<Vec<SemAct>>) -> Self {
        self = match self {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts: _,
                annotations,
            } => TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts: new_sem_acts,
                annotations,
            },
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts: _,
                annotations,
            } => TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts: new_sem_acts,
                annotations,
            },
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts: _,
                annotations,
            } => TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts: new_sem_acts,
                annotations,
            },
            TripleExpr::TripleExprRef(lbl) => {
                panic!("Can't update sem_acts to TripleExprRef({lbl:?}")
            }
        };
        self
    }

    pub fn with_annotations(mut self, new_annotations: Option<Vec<Annotation>>) -> Self {
        self = match self {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations: _,
            } => TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations: new_annotations,
            },
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations: new_annotations,
            } => TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations: new_annotations,
            },
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations: _,
            } => TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations: new_annotations,
            },
            TripleExpr::TripleExprRef(lbl) => {
                panic!("Can't update annotations to TripleExprRef({lbl:?}")
            }
        };
        self
    }
}

impl Deref for TripleExpr {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let id = <TripleExprLabel as Deref>::deref_opt(id, base, prefixmap)?;
                let annotations =
                    <Annotation as Deref>::deref_opt_vec(annotations, base, prefixmap)?;
                let sem_acts = <SemAct as Deref>::deref_opt_vec(sem_acts, base, prefixmap)?;
                let expressions =
                    <TripleExprWrapper as Deref>::deref_vec(expressions, base, prefixmap)?;
                Ok(TripleExpr::EachOf {
                    id,
                    expressions,
                    min: min.clone(),
                    max: max.clone(),
                    sem_acts,
                    annotations,
                })
            }
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let id = <TripleExprLabel as Deref>::deref_opt(id, base, prefixmap)?;
                let annotations =
                    <Annotation as Deref>::deref_opt_vec(annotations, base, prefixmap)?;
                let sem_acts = <SemAct as Deref>::deref_opt_vec(sem_acts, base, prefixmap)?;
                let expressions =
                    <TripleExprWrapper as Deref>::deref_vec(expressions, base, prefixmap)?;
                Ok(TripleExpr::OneOf {
                    id,
                    expressions,
                    min: min.clone(),
                    max: max.clone(),
                    sem_acts,
                    annotations,
                })
            }
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let id = <TripleExprLabel as Deref>::deref_opt(id, base, prefixmap)?;
                let annotations =
                    <Annotation as Deref>::deref_opt_vec(annotations, base, prefixmap)?;
                let sem_acts = <SemAct as Deref>::deref_opt_vec(sem_acts, base, prefixmap)?;
                let predicate = predicate.deref(base, prefixmap)?;
                let value_expr = <ShapeExpr as Deref>::deref_opt_box(value_expr, base, prefixmap)?;
                Ok(TripleExpr::TripleConstraint {
                    id,
                    negated: negated.clone(),
                    inverse: inverse.clone(),
                    predicate,
                    value_expr,
                    min: min.clone(),
                    max: max.clone(),
                    sem_acts,
                    annotations,
                })
            }
            TripleExpr::TripleExprRef(label) => {
                let label = label.deref(base, prefixmap)?;
                Ok(TripleExpr::TripleExprRef(label))
            }
        }
    }
}

impl FromStr for TripleExpr {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(TripleExpr::TripleExprRef(TripleExprLabel::IriRef {
            value: iri_ref,
        }))
    }
}

impl SerializeStringOrStruct for TripleExpr {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            TripleExpr::TripleExprRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct TripleExprWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub te: TripleExpr,
}

impl TripleExprWrapper {}

impl Deref for TripleExprWrapper {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        let te = self.te.deref(base, prefixmap)?;
        Ok(TripleExprWrapper { te })
    }
}

impl Into<TripleExprWrapper> for TripleExpr {
    fn into(self) -> TripleExprWrapper {
        TripleExprWrapper { te: self }
    }
}
