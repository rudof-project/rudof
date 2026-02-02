pub mod deref;
pub mod iri_ref;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod test;
mod visitor;

pub use deref::Deref;
pub use iri_ref::IriRef;
