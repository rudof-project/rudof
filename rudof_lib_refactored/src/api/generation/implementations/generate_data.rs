use crate::{
    Result, Rudof, 
    formats::{InputSpec, GenerationSchemaFormat, DataFormat}
};

pub fn generate_data(
    rudof: &Rudof,
    schema: &InputSpec,
    schema_format: &GenerationSchemaFormat,
    result_format: Option<&DataFormat>,
    number_entities: usize,
    seed: Option<u64>,
    parallel: Option<usize>,
) -> Result<()> {
    todo!()
}
