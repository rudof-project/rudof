use std::io::Read;

use crate::{
    Result, Rudof, errors::{DataError, ShExError}, formats::{InputSpec, ShapeMapFormat}, types::Data, utils::get_base_iri
};
use iri_s::IriS;
use rudof_rdf::rdf_core::Rdf;
use shex_ast::{ShapeMapParser, shapemap::QueryShapeMap};
use shex_validation::Validator as ShExValidator;

pub fn load_shapemap(
    rudof: &mut Rudof,
    shapemap: &InputSpec,
    shapemap_format: Option<&ShapeMapFormat>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
) -> Result<()> {
    let (shapemap_format, base_nodes, base_shapes) = init_defaults(rudof, shapemap_format, base_nodes, base_shapes)?;

    let (data, shex_validator) = validate_loaded_data_and_schema(rudof)?;

    match shapemap_format {
        ShapeMapFormat::Compact => {
            let shapemap = read_shapemap_compact(shapemap, data, shex_validator, base_nodes, base_shapes)?;
            rudof.shapemap = Some(shapemap);
        },
        _ => {
            todo!("ShapeMap format {:?} not yet implemented", shapemap_format);
        },
    }

    Ok(())
}

fn init_defaults(
    rudof: &mut Rudof,
    shapemap_format: Option<&ShapeMapFormat>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
) -> Result<(ShapeMapFormat, IriS, IriS)> {
    let base_nodes = get_base_iri(rudof, base_nodes)?;
    let base_shapes = get_base_iri(rudof, base_shapes)?;
    Ok((shapemap_format.copied().unwrap_or_default(), base_nodes, base_shapes))
}

fn validate_loaded_data_and_schema(rudof: &mut Rudof) -> Result<(&mut Data, &ShExValidator)> {
    let data = rudof.data.as_mut().ok_or(DataError::NoDataLoaded)?;

    if !data.is_rdf() {
        Err(DataError::NoRdfDataLoaded)?
    }

    let shex_validator = rudof.shex_validator.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    Ok((data, shex_validator))
}

fn read_shapemap_compact(
    shapemap: &InputSpec,
    data: &mut Data,
    shex_validator: &ShExValidator,
    base_nodes: IriS,
    base_shapes: IriS,
) -> Result<QueryShapeMap> {
    let mut shapemap_reader = shapemap
        .open_read(None, "ShapeMap data")
        .map_err(|error| ShExError::DataSourceSpec {
            message: format!("Failed to open shapemap source '{}': {error}", shapemap.source_name()),
        })?;

    let mut v = Vec::new();
    shapemap_reader
        .read_to_end(&mut v)
        .map_err(|error| ShExError::DataSourceSpec {
            message: format!("Failed to read shapemap source '{}': {error}", shapemap.source_name()),
        })?;
    let shapemap_string = String::from_utf8(v).map_err(|error| ShExError::DataSourceSpec {
        message: format!("Failed to read shapemap source '{}': {error}", shapemap.source_name()),
    })?;

    let shapemap = ShapeMapParser::parse(
        shapemap_string.as_str(),
        &Some(data.unwrap_rdf_mut().prefixmap().unwrap_or_default()),
        &Some(base_nodes),
        &Some(shex_validator.shapes_prefixmap()),
        &Some(base_shapes),
    )
    .map_err(|e| ShExError::FailedParsingShapeMap {
        source_name: shapemap.source_name().to_string(),
        error: e.to_string(),
    })?;

    Ok(shapemap)
}
