use crate::{
    Result, Rudof,
    errors::{DataError, ShaclError},
    formats::{DataReaderMode, InputSpec, ShaclFormat},
    utils::{PrefixDirective, default_prefix_header, get_base_iri},
};
use rudof_iri::{IriS, MimeType};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use shacl::error::IRError;
use shacl::ir::IRSchema;
use shacl::rdf::ShaclParser;
use sparql_service::RdfData;
use std::io::Read;

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

    // Only Turtle uses `@prefix` declarations that default prefixes can
    // usefully supplement; see the equivalent check in `load_data`.
    let rdf_graph = if matches!(schema_format, ShaclFormat::Turtle) {
        let mut content = String::new();
        schema_reader
            .read_to_string(&mut content)
            .map_err(|error| ShaclError::DataSourceSpec {
                message: format!("Failed to read shacl schema source '{}': {error}", schema.source_name()),
            })?;
        let header = default_prefix_header(rudof, &content, PrefixDirective::Turtle);
        let mut prefixed_reader = std::io::Cursor::new(format!("{header}{content}").into_bytes());
        OxigraphInMemory::from_reader(
            &mut prefixed_reader,
            &schema.source_name(),
            &schema_format.try_into()?,
            Some(base.as_str()),
            &reader_mode.into(),
        )
    } else {
        OxigraphInMemory::from_reader(
            &mut schema_reader,
            &schema.source_name(),
            &schema_format.try_into()?,
            Some(base.as_str()),
            &reader_mode.into(),
        )
    }
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

    let recursion_semantics = rudof.config().execute().shacl().recursion_semantics();
    rudof.shacl_shapes = Some(
        IRSchema::compile_with_recursion(&shacl_schema, recursion_semantics).map_err(|e: IRError| {
            ShaclError::FailedParsingShaclSchema {
                source_name: schema.source_name(),
                format: schema_format.to_string(),
                error: e.to_string(),
            }
        })?,
    );

    Ok(())
}

fn extract_shacl_shapes_from_data(rudof: &mut Rudof) -> Result<()> {
    let recursion_semantics = rudof.config().execute().shacl().recursion_semantics();

    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    let rdf_data = data.unwrap_rdf_mut();

    let shacl_schema =
        ShaclParser::new(rdf_data.clone())
            .parse()
            .map_err(|error| ShaclError::FailedParsingShaclSchema {
                source_name: "loaded RDF data".to_string(),
                format: "loaded RDF data format".to_string(),
                error: error.to_string(),
            })?;

    rudof.shacl_shapes = Some(
        IRSchema::compile_with_recursion(&shacl_schema, recursion_semantics).map_err(|e: IRError| {
            ShaclError::FailedParsingShaclSchema {
                source_name: "loaded RDF data".to_string(),
                format: "loaded RDF data format".to_string(),
                error: e.to_string(),
            }
        })?,
    );

    Ok(())
}
