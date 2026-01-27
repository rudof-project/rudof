pub mod iri_ref;
pub mod deref;
mod visitor;
mod test;

pub use deref::{Deref, DerefError};
pub use iri_ref::{IriRef, IriRefError};
