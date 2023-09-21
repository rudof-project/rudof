use std::fmt::Display;
use srdf::Object;

use crate::ValueSetValue;

#[derive(Clone, Debug)]
pub struct ValueSet {
    values: Vec<ValueSetValue>
}

impl ValueSet {

    pub fn new() -> ValueSet {
        ValueSet {
            values: Vec::new()
        }
    }

    pub fn add_value(&mut self, v: ValueSetValue) {
        self.values.push(v);
    }
    
    pub fn check_value(&self, object: &Object) -> bool {
        true
    }
}

impl Display for ValueSet {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for v in &self.values {
            write!(f, "{v},")?;
        }
        write!(f,"]")?;
        Ok(())
    }
    
}

