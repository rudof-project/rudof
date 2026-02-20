use crate::DataGeneratorError;
use crate::output::OutputWriter;

impl OutputWriter {
    pub(super) async fn create_parallel_manifest(&self, _: usize) -> Result<(), DataGeneratorError> {
        Err(DataGeneratorError::OutputWriting(
            "Unable to create manifest in WASM environment".to_string(),
        ))
    }
}
