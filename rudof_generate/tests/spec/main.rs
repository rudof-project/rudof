//! Behaviour specifications: one module per concern. See docs/testing.md in the super repo.
#![cfg(not(target_family = "wasm"))]

#[path = "../common/mod.rs"]
mod common;

mod basic_passthrough;
mod cardinality_semantics;
mod coherence;
mod config_v2;
mod configuration;
mod conformance_metric;
mod constraint_passthrough;
mod focused_constraint;
mod generator_constraints;
mod iri_valued_property;
mod pattern_generator;
mod pattern_generator_advanced;
mod pattern_generator_integration;
mod schema_conformance;
mod shacl_integration;
mod shapemap;
mod target_class_and_seed;
