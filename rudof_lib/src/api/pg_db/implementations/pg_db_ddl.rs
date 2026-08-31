use crate::{
    Result, Rudof,
    errors::PgDbError,
    formats::{DataFormat, DataReaderMode, DdlDialect, InputSpec},
};
use oxrdf::Triple as OxTriple;
use rudof_iri::MimeType;
use rudof_rdf::rdf_core::RDFFormat;
use sparql_service::RdfData;

/// Derives a property graph schema from `data` and emits it as DDL for
/// `dialect`. Parses `data` into a throwaway [`RdfData`] rather than going
/// through `rudof.load_data()`, so this never touches any RDF data already
/// loaded into `rudof`'s session state -- matching how the CLI's `ddl`
/// command never opens a database or otherwise mutates session state.
pub fn pg_db_ddl(
    _rudof: &Rudof,
    data: &[InputSpec],
    dialect: Option<&DdlDialect>,
    graph_type_name: Option<&str>,
    data_format: Option<&DataFormat>,
    base_data: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<String> {
    let data_format = data_format.copied().unwrap_or_default();
    let dialect = dialect.copied().unwrap_or_default();
    let graph_type_name = graph_type_name.unwrap_or("rudof_graph");
    let reader_mode = reader_mode.copied().unwrap_or_default();

    let rdf_format: RDFFormat = (&data_format).try_into().map_err(|error| PgDbError::DataSourceSpec {
        message: format!("Unsupported RDF data format: {error}"),
    })?;
    let reader_mode_rdf: rudof_rdf::rdf_impl::ReaderMode = reader_mode.into();

    let mut rdf_data = RdfData::new();
    for spec in data {
        let mut reader = spec
            .open_read(Some(data_format.mime_type()), "RDF data for ddl")
            .map_err(|error| PgDbError::DataSourceSpec {
                message: format!("Failed to open data source '{}': {error}", spec.source_name()),
            })?;
        rdf_data
            .merge_from_reader(
                &mut reader,
                spec.source_name().as_str(),
                &rdf_format,
                base_data,
                &reader_mode_rdf,
            )
            .map_err(|error| PgDbError::DataSourceSpec {
                message: format!("Failed to parse RDF data from '{}': {error}", spec.source_name()),
            })?;
    }

    let all_triples: Vec<OxTriple> = rdf_data
        .all_triples()
        .map_err(|error| PgDbError::DataSourceSpec {
            message: format!("Failed to enumerate triples: {error}"),
        })?
        .collect();

    let model = super::schema::derive_pg_schema(&all_triples);
    Ok(super::schema::emit_ddl(&model, dialect, graph_type_name))
}
