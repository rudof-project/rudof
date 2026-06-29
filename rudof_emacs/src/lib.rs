//! Emacs dynamic module exposing rudof's ShEx conformance validation.
//!
//! Not usable on `wasm32` targets (Emacs dynamic modules are native shared
//! libraries) -- see [`validate`] for everything this crate actually does.

#[cfg(not(target_family = "wasm"))]
pub mod validate;
