use crate::{ComparatorError, ShaCo};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

// Common Shape Model
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CoShaMo {
    constraints: HashMap<IriS, ValueDescription>,
    prefixmap: Option<PrefixMap>,
}

impl CoShaMo {
    pub fn new() -> Self {
        CoShaMo {
            constraints: HashMap::new(),
            prefixmap: None,
        }
    }

    pub fn with_prefixmap(mut self, prefixmap: Option<PrefixMap>) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn add_constraint(&mut self, predicate: &IriS, description: ValueDescription) {
        self.constraints.insert(predicate.clone(), description);
    }

    pub fn resolve(&self, iri_ref: &IriRef) -> Result<IriS, ComparatorError> {
        if let Some(prefixmap) = &self.prefixmap {
            prefixmap
                .resolve_iriref(iri_ref.clone())
                .map_err(|e| ComparatorError::ResolveError {
                    iri_ref: iri_ref.to_string(),
                    error: e.to_string(),
                })
        } else {
            Err(ComparatorError::NoPrefixMapDerefrencingIriRef {
                iri_ref: iri_ref.to_string(),
            })
        }
    }

    pub fn compare(&self, other: &CoShaMo) -> ShaCo {
        let mut shaco = ShaCo::new();
        for (property1, descr1) in self.constraints.iter() {
            if let Some(descr2) = other.constraints.get(property1) {
                shaco.add_equals_property(property1.clone(), descr1.clone(), descr2.clone());
            } else {
                shaco.add_diff_property1(property1.clone(), descr1.clone());
            }
        }
        for (property2, descr2) in other.constraints.iter() {
            if self.constraints.contains_key(property2) {
                // Nothing to do, as it should have already been inserted in equals properties
            } else {
                shaco.add_diff_property2(property2.clone(), descr2.clone());
            }
        }
        shaco
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueDescription {
    iri_ref: IriRef,

    #[serde(skip_serializing_if = "is_any")]
    value_constraint: ValueConstraint,

    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<Percentage>,
}

fn is_any(vc: &ValueConstraint) -> bool {
    matches!(vc, ValueConstraint::Any)
}

impl ValueDescription {
    pub fn new(iri_ref: &IriRef) -> Self {
        ValueDescription {
            iri_ref: iri_ref.clone(),
            value_constraint: ValueConstraint::Any,
            percentage: None,
        }
    }

    pub fn with_value_constraint(mut self, vc: ValueConstraint) -> Self {
        self.value_constraint = vc;
        self
    }

    pub fn with_percentage(mut self, percentage: Percentage) -> Self {
        self.percentage = Some(percentage);
        self
    }
}

impl Display for ValueDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  - value: {}", self.iri_ref)?;
        writeln!(f, "  - datatype: {}", self.value_constraint)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ValueConstraint {
    Datatype(IriS),
    NodeKind(String),
    Other,
    Ref(String),
    #[default]
    Any,
}

impl ValueConstraint {
    pub fn datatype(iri: IriS) -> Self {
        ValueConstraint::Datatype(iri)
    }

    pub fn nodekind(str: &str) -> Self {
        ValueConstraint::NodeKind(str.to_string())
    }

    pub fn reference(r: &str) -> Result<Self, ComparatorError> {
        Ok(ValueConstraint::Ref(r.to_string()))
    }
}

impl Display for ValueConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueConstraint::Datatype(iri_s) => write!(f, "{iri_s}"),
            ValueConstraint::Other => write!(f, "other"),
            ValueConstraint::Any => write!(f, "_"),
            ValueConstraint::Ref(r) => write!(f, "@{r}"),
            ValueConstraint::NodeKind(nk) => write!(f, "nodekind({nk})"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Percentage(f64);

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}

impl Display for CoShaMo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Common Shape Model:")?;
        for (property, descr) in self.constraints.iter() {
            writeln!(f, "- Property: {property}")?;
            writeln!(f, "{descr}")?;
        }
        Ok(())
    }
}
