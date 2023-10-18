use std::{result, str::FromStr};

use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use crate::ast::serde_string_or_struct::*;

use super::{
    annotation::Annotation, iri_ref::IriRef, sem_act::SemAct, shape_expr::ShapeExpr,
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
    
    pub fn triple_constraint(predicate: IriRef, se: Option<ShapeExpr>, min: Option<i32>, max: Option<i32>) -> TripleExpr {
        TripleExpr::TripleConstraint { 
            id: None, 
            inverse: None, predicate, 
            value_expr: se.map(|se| Box::new(se)), 
            min, 
            max, 
            sem_acts: None, 
            annotations: None 
        }
    }
}

impl FromStr for TripleExpr {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TripleExpr::TripleExprRef(TripleExprLabel::IriRef {
            value: IriRef {
                value: s.to_string(),
            },
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

impl Into<TripleExprWrapper> for TripleExpr {
    fn into(self) -> TripleExprWrapper {
        TripleExprWrapper { te: self }
    }
}
