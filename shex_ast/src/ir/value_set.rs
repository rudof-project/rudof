use srdf::Object;
use std::fmt::Display;

use super::value_set_value::ValueSetValue;

#[derive(Clone, Debug, Default)]
pub struct ValueSet {
    values: Vec<ValueSetValue>,
}

impl ValueSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_value(&mut self, v: ValueSetValue) {
        self.values.push(v);
    }

    pub fn check_value(&self, object: &Object) -> bool {
        self.values.iter().any(|vsv| vsv.match_value(object))
    }

    pub fn values(&self) -> &Vec<ValueSetValue> {
        &self.values
    }
}

impl Display for ValueSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for v in &self.values {
            write!(f, "{v} ")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
