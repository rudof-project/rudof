use crate::{Result, Rudof, errors::ShExError, utils::get_base_iri};
use shex_ast::{ShapeMapParser, shapemap::{QueryShapeMap, ShapeSelector}};

pub fn add_node_shape_to_shapemap(
    rudof: &mut Rudof,
    node: &str,
    shape: Option<&str>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
) -> Result<()> {
    let base_nodes_iri = get_base_iri(rudof, base_nodes)?;
    let base_shapes_iri = get_base_iri(rudof, base_shapes)?;

    let node_selector = ShapeMapParser::parse_node_selector(node)
        .map_err(|e| ShExError::NodeSelectorParseError {
            node_selector: node.to_string(),
            error: e.to_string(),
        })?;

    let shape_selector: ShapeSelector = match shape {
        Some(s) => ShapeMapParser::parse_shape_selector(s)
            .map_err(|e| ShExError::ShapeSelectorParseError {
                shape_selector: s.to_string(),
                error: e.to_string(),
            })?,
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
