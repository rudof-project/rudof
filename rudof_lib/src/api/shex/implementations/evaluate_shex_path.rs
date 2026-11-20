use crate::{Result, Rudof, api::shex::ShExPathMatch, errors::ShExError};
use shex_ast::shexpath::ShExJDocument;

pub fn evaluate_shex_path(rudof: &Rudof, path: &str) -> Result<Vec<ShExPathMatch>> {
    let schema = rudof.shex_schema.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
    let failed = |error: String| ShExError::FailedEvaluatingShExPath {
        path: path.to_string(),
        error,
    };
    let doc = ShExJDocument::new(schema).map_err(|e| failed(e.to_string()))?;
    let items = doc.select(path).map_err(|e| failed(e.to_string()))?;
    Ok(items
        .iter()
        .map(|item| ShExPathMatch {
            location: item.location().to_string(),
            value: item.value().clone(),
        })
        .collect())
}
