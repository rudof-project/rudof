#![allow(non_camel_case_types)] // Disables warnings for non-camel-case type names
#![allow(non_snake_case)] // Disables warnings for non-snake-case function/variable names
#![allow(non_upper_case_globals)] // Disables warnings for non-upper-case static constants

pub mod boolean_expr;
pub mod card;
pub mod cli;
pub mod edge;
pub mod edge_id;
pub mod edge_type;
pub mod evidence;
pub mod formal_base_type;
pub mod key;
pub mod label_property_spec;
pub mod node;
pub mod node_id;

#[rustfmt::skip]
#[allow(clippy::all)]
pub mod parser;
pub mod pg;
pub mod pgs;
pub mod pgs_error;
pub mod property_value_spec;
pub mod record;
pub mod record_type;
pub mod result_association;
pub mod type_map;
pub mod type_name;
pub mod validation_result;
pub mod value;
pub mod value_type;

pub enum PgSchemaFormat {
    PgSchemaC,
}
