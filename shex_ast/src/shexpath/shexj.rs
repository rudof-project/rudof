//! The ShExJ document a ShExPath is evaluated over.
//!
//! ShExPath steps are named for ShExJ's property names, and an item is any node of the ShExJ
//! document, so a path is evaluated over the schema written as ShExJ. That is the schema as rudof
//! serializes it, with two differences:
//!
//! - the `prefixmap` and `base` keys rudof adds to the schema are left out, since ShExJ has no such
//!   properties;
//! - every semantic action has its ShExJ `type`, `SemAct`, which rudof does not write.
//!
//! JSON objects are unordered, and so are the members of an object here.

use crate::Schema;
use crate::shexpath::ShExPathError;
use serde_json::Value;

/// The ShExJ form of `schema`.
pub fn shexj(schema: &Schema) -> Result<Value, ShExPathError> {
    let mut doc = serde_json::to_value(schema).map_err(|e| ShExPathError::Serialization { msg: e.to_string() })?;
    if let Value::Object(map) = &mut doc {
        map.remove("prefixmap");
        map.remove("base");
    }
    add_sem_act_types(&mut doc);
    Ok(doc)
}

/// Adds `"type": "SemAct"` to the members of every `semActs` and `startActs` array.
fn add_sem_act_types(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, member) in map.iter_mut() {
                if (key == "semActs" || key == "startActs")
                    && let Value::Array(acts) = member
                {
                    acts.iter_mut().for_each(add_sem_act_type);
                }
                add_sem_act_types(member);
            }
        },
        Value::Array(members) => members.iter_mut().for_each(add_sem_act_types),
        _ => {},
    }
}

fn add_sem_act_type(act: &mut Value) {
    if let Value::Object(map) = act {
        map.entry("type").or_insert_with(|| Value::String("SemAct".to_string()));
    }
}
