pub mod iri_ref;
pub mod deref;
mod visitor;

pub use deref::{Deref, DerefError};
pub use iri_ref::{IriRef, IriRefError};
