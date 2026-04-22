use crate::{Result, Rudof, errors::MaterializeError, formats::ResultDataFormat};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::InMemoryGraph;
use shex_ast::Node;
use shex_ast::materialize::Materializer;
use std::io;
use std::str::FromStr;

pub fn materialize<W: io::Write>(
    rudof: &Rudof,
    initial_node_iri: Option<&str>,
    result_format: Option<&ResultDataFormat>,
    writer: &mut W,
) -> Result<()> {
    let schema = rudof.shex_schema.as_ref().ok_or(MaterializeError::NoShExSchemaLoaded)?;

    let map_state = rudof.map_state.as_ref().ok_or(MaterializeError::NoMapStateLoaded)?;

    let initial_node = match initial_node_iri {
        Some(iri_str) => {
            let iri = IriS::from_str(iri_str).map_err(|e| MaterializeError::InvalidIri {
                iri: iri_str.to_string(),
                error: e.to_string(),
            })?;
            Some(Node::iri(iri))
        },
        None => None,
    };

    let materializer = Materializer::new();
    let graph: InMemoryGraph = materializer
        .materialize(schema, map_state, initial_node)
        .map_err(|e| MaterializeError::FailedMaterialization { error: e.to_string() })?;

    let rdf_format =
        result_format
            .copied()
            .unwrap_or_default()
            .try_into()
            .map_err(
                |e: Box<crate::errors::DataError>| MaterializeError::FailedSerializingGraph {
                    format: result_format.map_or_else(|| "turtle".to_string(), |f| f.to_string()),
                    error: e.to_string(),
                },
            )?;

    graph
        .serialize(&rdf_format, writer)
        .map_err(|e| MaterializeError::FailedSerializingGraph {
            format: result_format.map_or_else(|| "turtle".to_string(), |f| f.to_string()),
            error: e.to_string(),
        })?;

    Ok(())
}
