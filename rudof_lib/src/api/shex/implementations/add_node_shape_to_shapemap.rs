use crate::{Result, Rudof, errors::ShExError, formats::IriNormalizationMode, utils::get_base_iri};
use shex_ast::{
    ShapeMapParser,
    shapemap::{QueryShapeMap, ShapeSelector},
};

pub fn add_node_shape_to_shapemap(
    rudof: &mut Rudof,
    node: &str,
    shape: Option<&str>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
    iri_mode: IriNormalizationMode,
) -> Result<()> {
    let base_nodes_iri = get_base_iri(rudof, base_nodes)?;
    let base_shapes_iri = get_base_iri(rudof, base_shapes)?;

    let normalized_node = crate::utils::normalize_iri_str(node, iri_mode);
    let node_selector =
        ShapeMapParser::parse_node_selector(&normalized_node).map_err(|e| ShExError::NodeSelectorParseError {
            node_selector: node.to_string(),
            error: e.to_string(),
        })?;

    let shape_selector: ShapeSelector = match shape {
        Some(s) => {
            let normalized_shape = crate::utils::normalize_iri_str(s, iri_mode);
            ShapeMapParser::parse_shape_selector(&normalized_shape).map_err(|e| ShExError::ShapeSelectorParseError {
                shape_selector: s.to_string(),
                error: e.to_string(),
            })?
        },
        None => ShapeSelector::start(),
    };

    let shapemap = rudof.shapemap.get_or_insert_with(QueryShapeMap::new);
    shapemap
        .add_association(
            node_selector,
            &Some(base_nodes_iri),
            shape_selector,
            &Some(base_shapes_iri),
        )
        .map_err(|e| ShExError::DataSourceSpec { message: e.to_string() })?;

    Ok(())
}
