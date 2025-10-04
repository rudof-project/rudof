use crate::ShExFormat;

/// Method employed to resolve imports when ghessing the format of an import
#[derive(Debug, Clone)]
pub enum ResolveMethod {
    RotatingFormats(Vec<ShExFormat>),
    ByGuessingExtension,
    ByContentNegotiation,
}

impl Default for ResolveMethod {
    fn default() -> Self {
        ResolveMethod::RotatingFormats(vec![ShExFormat::ShExC, ShExFormat::ShExJ])
    }
}
