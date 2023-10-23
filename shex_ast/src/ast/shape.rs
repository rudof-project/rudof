use iri_s::{IriS, IriSError};
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};

use crate::{Annotation, IriRef, SemAct, TripleExpr, TripleExprWrapper};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct Shape {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<IriRef>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<TripleExprWrapper>,

    #[serde(default, rename = "semActs", skip_serializing_if = "Option::is_none")]
    pub sem_acts: Option<Vec<SemAct>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
}

impl Shape {
    pub fn new(
        closed: Option<bool>,
        extra: Option<Vec<IriRef>>,
        expression: Option<TripleExpr>,
    ) -> Self {
        Shape {
            closed,
            extra,
            expression: expression.map(|e| e.into()),
            sem_acts: None,
            annotations: None,
        }
    }

    pub fn with_expression(mut self, expression: TripleExpr) -> Self {
        self.expression = Some(expression.into());
        self
    }

    pub fn with_sem_acts(mut self, sem_acts: Option<Vec<SemAct>>) -> Self {
        self.sem_acts = sem_acts;
        self
    }

    pub fn with_annotations(mut self, annotations: Option<Vec<Annotation>>) -> Self {
        self.annotations = annotations;
        self
    }

    pub fn deref(
        mut self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, IriSError> {
        self = Shape {
            closed: self.closed,
            extra: self.extra.map(|es| {
                es.into_iter()
                    .map(|e| {
                        let e = e.deref(base, prefixmap)?;
                        e
                    })
                    .collect()
            }),
            expression: self.expression.map(|e| {
                let e = e.deref(base, prefixmap)?;
                e
            }),
            sem_acts: self.sem_acts,
            annotations: self
                .annotations
                .map(|anns| anns.into_iter().map(|a| a.deref(base, prefixmap)).collect()),
        };
        self
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            closed: None,
            extra: None,
            expression: None,
            sem_acts: None,
            annotations: None,
        }
    }
}
