//! Input adapters: one per schema language, each translating into the IR.
pub mod shacl;
pub mod shex;

pub use shacl::ShaclToUnified;
pub use shex::ShExToUnified;
