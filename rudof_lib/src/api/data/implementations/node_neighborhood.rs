use crate::{
    Result, Rudof, errors::DataError, formats::{IriNormalizationMode, NodeInspectionMode}, types::{Data, NodeNeighborhood},
    api::data::implementations::show_node_info::{convert_predicate_strings_to_iris, parse_node_selector}
};
use sparql_service::RdfData;

/// Returns a lazy iterator over the neighborhood of every node matched by `node`
pub fn node_neighborhood<'a>(
    rudof: &'a Rudof,
    node: &str,
    predicates: Option<&[String]>,
    mode: Option<&NodeInspectionMode>,
    depth: Option<usize>,
    iri_mode: IriNormalizationMode
) -> Result<NodeNeighborhood<'a>> {
    let Some(Data::RDFData(rdf)) = rudof.data.as_ref() else {
        return Err(Box::new(DataError::NoRdfDataLoaded).into());
    };
    let rdf: &'a RdfData = rdf;

    let node_selector = parse_node_selector(node, iri_mode)?;
    let roots = node_selector.nodes(rdf).map_err(|e| Box::new(DataError::FailedArcRetrieval { error: e.to_string() }))?;

    let predicates = convert_predicate_strings_to_iris(predicates.unwrap_or_default(), rdf)?;
    let directions = mode.copied().unwrap_or_default().directions();

    Ok(NodeNeighborhood::new(
        rdf,
        roots,
        predicates,
        directions,
        depth.unwrap_or(1)
    ))
}