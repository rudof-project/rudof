//! Input adapters: one per schema language, each translating into the IR.
pub mod pgschema;
pub mod shacl;
pub mod shex;

pub use pgschema::PgSchemaToUnified;
pub use shacl::ShaclToUnified;
pub use shex::ShExToUnified;
