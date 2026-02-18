use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{FocusParser, InstancesParser};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocab::rdfs_class;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use shacl_ast::ShaclVocab;
use shacl_ast::target::Target;

pub(crate) fn targets_implicit_class<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>> {
    InstancesParser::new(rdfs_class().clone())
        .and(InstancesParser::new(ShaclVocab::sh_property_shape().clone()))
        .and(InstancesParser::new(ShaclVocab::sh_node_shape().clone()))
        .and(FocusParser::new())
        .flat_map(
            move |(((class, property_shapes), node_shapes), focus): (_, RDF::Term)| {
                let result: Result<Vec<Target<RDF>>, RDFError> = class
                    .into_iter()
                    .filter(|t: &RDF::Subject| property_shapes.contains(t) || node_shapes.contains(t))
                    .map(Into::into)
                    .filter(|t: &RDF::Term| t.clone() == focus)
                    .map(|t: RDF::Term| {
                        let t_name = t.to_string();
                        let obj = t
                            .clone()
                            .try_into()
                            .map_err(|_| RDFError::FailedTermToRDFNodeError { term: t_name })?;
                        Ok(Target::ImplicitClass(obj))
                    })
                    .collect();
                let ts = result?;
                Ok(ts)
            },
        )
}
