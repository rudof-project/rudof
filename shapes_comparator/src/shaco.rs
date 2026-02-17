use crate::{ComparatorError, ValueDescription};
use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

// Shapes Comparison: Captures the results of comparing two shapes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaCo {
    equal_properties: HashMap<IriS, EqualProperty>,
    properties1: HashMap<IriS, DiffProperty>,
    properties2: HashMap<IriS, DiffProperty>,
}

impl ShaCo {
    pub fn new() -> Self {
        ShaCo {
            equal_properties: HashMap::new(),
            properties1: HashMap::new(),
            properties2: HashMap::new(),
        }
    }

    pub fn add_equals_property(&mut self, iri_s: IriS, descr1: ValueDescription, descr2: ValueDescription) {
        self.equal_properties.insert(
            iri_s,
            EqualProperty {
                description1: Some(descr1),
                description2: Some(descr2),
            },
        );
    }

    pub fn add_diff_property1(&mut self, iri_s: IriS, descr: ValueDescription) {
        self.properties1.insert(iri_s, DiffProperty::new(descr));
    }

    pub fn add_diff_property2(&mut self, iri_s: IriS, descr: ValueDescription) {
        self.properties2.insert(iri_s, DiffProperty::new(descr));
    }

    pub fn as_json(&self) -> Result<String, ComparatorError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ComparatorError::JsonSerializationError { error: format!("{e}") })
    }
}

impl Default for ShaCo {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqualProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    description1: Option<ValueDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description2: Option<ValueDescription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<ValueDescription>,
}

impl DiffProperty {
    pub fn new(descr: ValueDescription) -> Self {
        DiffProperty {
            description: Some(descr.clone()),
        }
    }
}

impl Display for ShaCo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Shapes Comparison:")?;
        if !self.equal_properties.is_empty() {
            writeln!(f, " Equal properties:")?;
            for (property, equals) in self.equal_properties.iter() {
                writeln!(f, "  - {property}: {equals}")?;
            }
        }
        if !self.properties1.is_empty() {
            writeln!(f, " Properties in shape 1 that are not in shape 2:")?;
            for (property, descr) in self.properties1.iter() {
                writeln!(f, "  - {property}: {descr}")?;
            }
        }
        if !self.properties2.is_empty() {
            writeln!(f, " Properties in shape 2 that are not in shape 1:")?;
            for (property, descr) in self.properties2.iter() {
                writeln!(f, "  - {property}: {descr}")?;
            }
        }
        Ok(())
    }
}

impl Display for EqualProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n  - descr1: {}",
            self.description1.as_ref().map(|d| d.to_string()).unwrap_or_default()
        )?;
        writeln!(
            f,
            "  - descr2: {}",
            self.description2.as_ref().map(|d| d.to_string()).unwrap_or_default()
        )?;
        Ok(())
    }
}

impl Display for DiffProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n  - descr: {}",
            self.description.as_ref().map(|d| d.to_string()).unwrap_or_default()
        )?;
        Ok(())
    }
}
