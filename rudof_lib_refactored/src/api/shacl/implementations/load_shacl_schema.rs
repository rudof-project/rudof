use crate::{
    Result, Rudof,
    errors::{DataError, ShaclError},
    formats::{DataReaderMode, InputSpec, ShaclFormat},
    utils::get_base_iri,
};
use iri_s::{IriS, MimeType};
use rudof_rdf::rdf_impl::InMemoryGraph;
use shacl_ir::compiled::schema_ir::SchemaIR as ShaclSchemaIR;
use shacl_rdf::ShaclParser;
use sparql_service::RdfData;

pub fn load_shacl_schema(
    rudof: &mut Rudof,
    schema: Option<&InputSpec>,
    schema_format: Option<&ShaclFormat>,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    if let Some(schema) = schema {
        let (schema_format, base, reader_mode) = init_defaults(rudof, schema_format, base, reader_mode)?;
        read_shacl_schema(rudof, schema, schema_format, base, reader_mode)?;
    } else {
        extract_shacl_shapes_from_data(rudof)?;
    }

    compile_shacl_schema(rudof)?;

    Ok(())
}

fn init_defaults(
    rudof: &mut Rudof,
    schema_format: Option<&ShaclFormat>,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<(ShaclFormat, IriS, DataReaderMode)> {
    let base = get_base_iri(rudof, base)?;

    Ok((
        schema_format.copied().unwrap_or_default(),
        base,
        reader_mode.copied().unwrap_or_default(),
    ))
}

fn read_shacl_schema(
    rudof: &mut Rudof,
    schema: &InputSpec,
    schema_format: ShaclFormat,
    base: IriS,
    reader_mode: DataReaderMode,
) -> Result<()> {
    let mut schema_reader = schema
        .open_read(Some(schema_format.mime_type()), "SHACL shapes")
        .map_err(|error| ShaclError::DataSourceSpec {
            message: format!("Failed to open shacl schema source '{}': {error}", schema.source_name()),
        })?;

    let rdf_graph = InMemoryGraph::from_reader(
        &mut schema_reader,
        &schema.source_name(),
        &schema_format.try_into()?,
        Some(base.as_str()),
        &reader_mode.into(),
    )
    .map_err(|error| ShaclError::DataSourceSpec {
        message: format!("Failed to read shacl schema source '{}': {error}", schema.source_name()),
    })?;

    let rdf_data = RdfData::from_graph(rdf_graph).map_err(|error| ShaclError::DataSourceSpec {
        message: format!("Failed to read shacl schema source '{}': {error}", schema.source_name()),
    })?;

    let shacl_schema = ShaclParser::new(rdf_data)
        .parse()
        .map_err(|error| ShaclError::FailedParsingShaclSchema {
            source_name: schema.source_name(),
            format: schema_format.to_string(),
            error: error.to_string(),
        })?;

    rudof.shacl_shapes = Some(shacl_schema);

    Ok(())
}

fn compile_shacl_schema(rudof: &mut Rudof) -> Result<()> {
    let shacl_schema = rudof.shacl_shapes.as_ref().unwrap();
    let shacl_schema_ir = ShaclSchemaIR::compile(shacl_schema)
        .map_err(|e| ShaclError::FailedCompilingShaclSchema { error: e.to_string() })?;

    rudof.shacl_shapes_ir = Some(shacl_schema_ir);

    Ok(())
}

fn extract_shacl_shapes_from_data(rudof: &mut Rudof) -> Result<()> {
    let data = rudof.data.as_mut().ok_or(DataError::NoDataLoaded)?;

    if !data.is_rdf() {
        Err(DataError::NoRdfDataLoaded)?
    }

    let rdf_data = data.unwrap_rdf_mut();

    let shacl_schema = ShaclParser::new(rdf_data.clone())
        .parse()
        .map_err(|error| ShaclError::FailedParsingShaclSchema {
            source_name: "loaded RDF data".to_string(),
            format: "loaded RDF data format".to_string(),
            error: error.to_string(),
        })?;

    rudof.shacl_shapes = Some(shacl_schema);

    Ok(())
}
