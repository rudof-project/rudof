use crate::{Result, Rudof, errors::ShExError, types::ShExPathMatch};
use shex_ast::shexpath::shexj_document;

pub fn evaluate_shex_path(rudof: &Rudof, path: &str) -> Result<Vec<ShExPathMatch>> {
    let schema = rudof.shex_schema.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
    let failed = |error: String| ShExError::FailedEvaluatingShExPath {
        path: path.to_string(),
        error,
    };
    let doc = shexj_document(schema).map_err(|e| failed(e.to_string()))?;
    let items = doc.select(path).map_err(|e| failed(e.to_string()))?;
    Ok(items
        .iter()
        .map(|item| ShExPathMatch {
            location: item.location().to_string(),
            value: item.value().clone(),
        })
        .collect())
}
