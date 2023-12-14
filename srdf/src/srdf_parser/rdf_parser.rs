use super::rdf_parser_error::RDFParseError;
use crate::SRDF;
use std::{error::Error, fmt::Display};

trait RDFParser<RDF: SRDF> {
    /// The type which is returned if the parser is successful.    
    type Output;

    fn parse(&mut self, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

trait RDFNodeParser<RDF: SRDF> {
    type Output;

    fn parse_subject(&mut self, node: RDF::Subject, rdf: RDF) -> Result<Self::Output, RDF::Err>;
}

/*fn parse_list_for_predicate<RDF>(
    node: RDF::Subject,
    pred: RDF::IRI,
    rdf: RDF,
) -> Result<Vec<RDF::Term>, RDF::Err>
where
    RDF: SRDF,
{
    let object = predicate_value(node, pred, rdf)?;
    let rest = parse_list_for_predicate_visited(object, pred, rdf, Vec::new())?;
}*/

pub fn predicate_value<RDF>(
    node: RDF::Subject,
    pred: RDF::IRI,
    rdf: RDF,
) -> Result<RDF::Term, RDFParseError>
where
    RDF: SRDF,
{
    let values = rdf
        .get_objects_for_subject_predicate(&node, &pred)
        .map_err(|e| RDFParseError::SRDFError {
            err: format!("{e}"),
        })?;
    match values.len() {
        0 => Err(RDFParseError::NoValuesPredicate {
            node: format!("{node}"),
            pred: format!("{pred}"),
        }),
        1 => {
            let value = values.into_iter().next().unwrap();
            Ok(value)
        }
        _ => Err(RDFParseError::MoreThanOneValuePredicate {
            node: format!("{node}"),
            pred: format!("{pred}"),
            values: format!("{values:?}"),
        }),
    }
}

/*fn parse_list_for_predicate_visited<RDF>(
    node: RDF::Subject,
    pred: RDF::IRI,
    rdf: RDF,
    mut visited: Vec<RDF::Term>,
) -> Result<Vec<RDF::Term>, RDF::Err>
where
    RDF: SRDF,
{
    let object = predicate_value(node, pred, rdf)?;
    let value = predicate_value(object, rdf_first(), rdf)?;
    if visited.contains(object) {
        todo!()
        // return error
    }
    visited.push(object);
    let rest = parse_list_for_predicate_visited(object, pred, rdf, visited)?;
    Ok(rest.push(object))
}
*/
