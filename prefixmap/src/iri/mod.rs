pub mod deref;
pub mod iri_ref;

#[cfg(test)]
mod test;
mod visitor;

pub use deref::Deref;
pub use iri_ref::IriRef;
