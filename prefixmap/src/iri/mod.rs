pub mod deref;
pub mod iri_ref;

#[cfg(all(test, not(target_family = "wasm")))]
mod test;
mod visitor;

pub use deref::Deref;
pub use iri_ref::IriRef;
