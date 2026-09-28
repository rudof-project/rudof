use crate::error::Result;
use rudof_lib::errors::RudofError as CoreError;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub(crate) fn runtime() -> Result<&'static Runtime> {
    if let Some(rt) = RUNTIME.get() {
        return Ok(rt);
    }
    let rt = Runtime::new().map_err(|e| CoreError::Generic {
        error: format!("could not start the async runtime: {e}"),
    })?;
    Ok(RUNTIME.get_or_init(|| rt))
}
