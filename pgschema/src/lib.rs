#![allow(non_camel_case_types)] // Disables warnings for non-camel-case type names
#![allow(non_snake_case)] // Disables warnings for non-snake-case function/variable names
#![allow(non_upper_case_globals)] // Disables warnings for non-upper-case static constants

#[cfg(not(target_family = "wasm"))]
pub mod boolean_expr;
#[cfg(not(target_family = "wasm"))]
pub mod card;
#[cfg(not(target_family = "wasm"))]
pub mod cli;
#[cfg(not(target_family = "wasm"))]
pub mod edge;
#[cfg(not(target_family = "wasm"))]
pub mod edge_id;
#[cfg(not(target_family = "wasm"))]
pub mod edge_type;
#[cfg(not(target_family = "wasm"))]
pub mod evidence;
#[cfg(not(target_family = "wasm"))]
pub mod formal_base_type;
#[cfg(not(target_family = "wasm"))]
pub mod key;
#[cfg(not(target_family = "wasm"))]
pub mod label_property_spec;
#[cfg(not(target_family = "wasm"))]
pub mod node;
#[cfg(not(target_family = "wasm"))]
pub mod node_id;

#[cfg(not(target_family = "wasm"))]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod parser;
#[cfg(not(target_family = "wasm"))]
pub mod pg;
#[cfg(not(target_family = "wasm"))]
pub mod pgs;
#[cfg(not(target_family = "wasm"))]
pub mod pgs_error;
#[cfg(not(target_family = "wasm"))]
pub mod property_value_spec;
#[cfg(not(target_family = "wasm"))]
pub mod record;
#[cfg(not(target_family = "wasm"))]
pub mod record_type;
#[cfg(not(target_family = "wasm"))]
pub mod result_association;
#[cfg(not(target_family = "wasm"))]
pub mod type_map;
#[cfg(not(target_family = "wasm"))]
pub mod type_name;
#[cfg(not(target_family = "wasm"))]
pub mod validation_result;
#[cfg(not(target_family = "wasm"))]
pub mod value;
#[cfg(not(target_family = "wasm"))]
pub mod value_type;

#[cfg(not(target_family = "wasm"))]
pub enum PgSchemaFormat {
    PgSchemaC,
}
