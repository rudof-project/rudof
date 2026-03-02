pub mod deref_iri;
pub mod iri_ref;

#[cfg(all(test, not(target_family = "wasm")))]
mod test;
mod visitor;

pub use deref_iri::DerefIri;
pub use iri_ref::IriRef;
