use iri_s::IriS;
use prefixmap::PrefixMap;
use shex_ast::ShapeExpr;

pub struct ParserState {
    base: Option<IriS>,
    prefixmap: PrefixMap,
    start: Option<ShapeExpr>,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            base: None,
            prefixmap: PrefixMap::default(),
            start: None,
        }
    }
}
