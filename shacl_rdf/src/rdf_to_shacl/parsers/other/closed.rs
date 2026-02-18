use iri_s::IriS;
use rdf::rdf_core::parser::rdf_node_parser::constructors::{SingleBoolPropertyParser, SingleValuePropertyAsListParser};
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rdf::rdf_core::term::Iri;
use rdf::rdf_core::{FocusRDF, RDFError};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;
use std::collections::HashSet;

pub(crate) fn closed<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    SingleBoolPropertyParser::new(ShaclVocab::sh_closed().clone())
        .optional()
        .then(move |maybe_closed| {
            ignored_properties().map(move |is| maybe_closed.map_or(vec![], |b| vec![Component::closed(b, is)]))
        })
}

fn ignored_properties<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<IriS>>
where
    RDF: FocusRDF,
{
    SingleValuePropertyAsListParser::new(ShaclVocab::sh_ignored_properties().clone())
        .optional()
        .flat_map(|is| match is {
            None => Ok(HashSet::new()),
            Some(vs) => {
                let mut hs = HashSet::new();
                for v in vs {
                    if let Ok(iri) = RDF::term_as_iri(&v) {
                        let iri: RDF::IRI = iri;
                        let iri_string = iri.as_str();
                        let iri_s = IriS::new_unchecked(iri_string);
                        hs.insert(iri_s);
                    } else {
                        return Err(RDFError::ExpectedIRIError { term: v.to_string() });
                    }
                }
                Ok(hs)
            },
        })
}
