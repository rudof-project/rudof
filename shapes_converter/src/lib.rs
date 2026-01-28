//! Shapes converter
//!
//!
pub mod converter_config;
pub mod converter_error;
pub mod landing_html_template;
pub mod service_to_mie;
pub mod shacl_to_shex;
pub mod shex_to_html;
pub mod shex_to_sparql;
pub mod shex_to_uml;
pub mod tap_to_shex;

use iri_s::IriS;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use shex_ast::Annotation;
use shex_ast::ObjectValue;

pub use crate::converter_config::*;
pub use crate::converter_error::*;
pub use crate::service_to_mie::service2mie::*;
pub use crate::shacl_to_shex::shacl2shex::*;
pub use crate::shacl_to_shex::shacl2shex_config::*;
pub use crate::shacl_to_shex::shacl2shex_error::*;
pub use crate::shex_to_html::shex2html::*;
pub use crate::shex_to_html::shex2html_config::*;
pub use crate::shex_to_html::shex2html_error::*;
pub use crate::shex_to_sparql::shex2sparql::*;
pub use crate::shex_to_sparql::shex2sparql_config::*;
pub use crate::shex_to_sparql::shex2sparql_error::*;
pub use crate::shex_to_uml::shex2uml::*;
pub use crate::shex_to_uml::shex2uml_config::*;
pub use crate::shex_to_uml::shex2uml_error::*;
pub use crate::tap_to_shex::tap2shex::*;
pub use crate::tap_to_shex::tap2shex_config::*;
pub use crate::tap_to_shex::tap2shex_error::*;

pub const DEFAULT_REPLACE_IRI_BY_LABEL: bool = true;

pub fn find_annotation(
    annotations: &Option<Vec<Annotation>>,
    predicate: &IriS,
    prefixmap: &PrefixMap,
) -> std::result::Result<Option<ObjectValue>, PrefixMapError> {
    if let Some(anns) = annotations {
        for a in anns.iter() {
            let iri_predicate = prefixmap.resolve_iriref(a.predicate())?;
            if *predicate == iri_predicate {
                return Ok(Some(a.object()));
            }
        }
        Ok(None)
    } else {
        Ok(None)
    }
}

fn object_value2string(object_value: &ObjectValue) -> String {
    match object_value {
        ObjectValue::IriRef(_) => todo!(),
        ObjectValue::Literal(lit) => lit.lexical_form(),
    }
}
