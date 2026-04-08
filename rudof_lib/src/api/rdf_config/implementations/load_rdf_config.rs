use crate::{
    Result, Rudof,
    errors::RdfConfigError,
    formats::{InputSpec, RdfConfigFormat},
};
use rdf_config::RdfConfigModel;

pub fn load_rdf_config(
    rudof: &mut Rudof,
    rdf_config: &InputSpec,
    _rdf_config_format: Option<&RdfConfigFormat>,
) -> Result<()> {
    let rdf_config_reader =
        rdf_config
            .open_read(None, "RDF config")
            .map_err(|error| RdfConfigError::DataSourceSpec {
                message: format!(
                    "Failed to open RDF config source '{}': {error}",
                    rdf_config.source_name()
                ),
            })?;

    let rdf_config = RdfConfigModel::from_reader(rdf_config_reader, rdf_config.source_name()).map_err(|error| {
        RdfConfigError::DataSourceSpec {
            message: format!(
                "Failed to read RDF config source '{}': {error}",
                rdf_config.source_name()
            ),
        }
    })?;

    rudof.rdf_config = Some(rdf_config);

    Ok(())
}
