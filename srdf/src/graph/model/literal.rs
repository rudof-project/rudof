use oxrdf::Literal as OxLiteral;
use oxrdf::LiteralRef as OxLiteralRef;

use crate::model::Literal;

impl Literal for OxLiteral {
    fn datatype(&self) -> &str {
        self.datatype().as_str()
    }

    fn as_string(&self) -> Option<String> {
        Some(self.value().to_string())
    }
}

impl Literal for OxLiteralRef<'_> {
    fn datatype(&self) -> &str {
        OxLiteralRef::datatype(*self).as_str()
    }

    fn as_string(&self) -> Option<String> {
        Some(self.value().to_string())
    }
}
