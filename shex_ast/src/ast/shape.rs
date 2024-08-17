use iri_s::IriS;
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};

use crate::{Annotation, SemAct, ShapeExprLabel, TripleExpr, TripleExprWrapper};
use prefixmap::{Deref, DerefError, IriRef};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<Vec<ShapeExprLabel>>,
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
            extends: None,
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

    pub fn annotations(&self) -> Option<impl Iterator<Item = &Annotation>> {
        match &self.annotations {
            None => None,
            Some(anns) => Some(anns.iter()),
        }
    }

    pub fn has_annotations(&self) -> bool {
        match &self.annotations {
            None => false,
            Some(_) => true,
        }
    }

    pub fn with_annotations(mut self, annotations: Option<Vec<Annotation>>) -> Self {
        self.annotations = annotations;
        self
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if let Some(annotations) = &mut self.annotations {
            annotations.push(annotation)
        } else {
            self.annotations = Some(vec![annotation])
        }
    }

    pub fn with_extends(mut self, extends: Option<Vec<ShapeExprLabel>>) -> Self {
        self.extends = extends;
        self
    }

    pub fn add_extend(&mut self, extend: ShapeExprLabel) {
        match &mut self.extends {
            None => self.extends = Some(vec![extend]),
            Some(ref mut es) => es.push(extend),
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed.unwrap_or(false)
    }

    pub fn triple_expr(&self) -> Option<TripleExpr> {
        self.expression.as_ref().map(|tew| tew.te.clone())
    }
}

impl Deref for Shape {
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        let new_extra = match &self.extra {
            None => None,
            Some(es) => {
                let mut ves = Vec::new();
                for e in es {
                    ves.push(e.deref(base, prefixmap)?);
                }
                Some(ves)
            }
        };
        let new_expr = match &self.expression {
            None => None,
            Some(expr) => {
                let ed = expr.deref(base, prefixmap)?;
                Some(ed)
            }
        };
        let new_anns = match &self.annotations {
            None => None,
            Some(anns) => {
                let mut new_as = Vec::new();
                for a in anns {
                    new_as.push(a.deref(base, prefixmap)?);
                }
                Some(new_as)
            }
        };
        let new_sem_acts = match &self.sem_acts {
            None => None,
            Some(sem_acts) => {
                let mut new_sas = Vec::new();
                for sa in sem_acts {
                    new_sas.push(sa.deref(base, prefixmap)?);
                }
                Some(new_sas)
            }
        };
        let new_extends = match &self.extends {
            None => None,
            Some(extends) => {
                let mut new_extends = Vec::new();
                for e in extends {
                    new_extends.push(e.deref(base, prefixmap)?);
                }
                Some(new_extends)
            }
        };

        let shape = Shape {
            closed: self.closed,
            extra: new_extra,
            expression: new_expr,
            sem_acts: new_sem_acts,
            annotations: new_anns,
            extends: new_extends,
        };
        Ok(shape)
    }
}
