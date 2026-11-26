use crate::types::MessageMap;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::LiteralsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn name<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = MessageMap> {
    LiteralsPropertyParser::new(ShaclVocab::sh_name()).flat_map(|lits| {
        // TODO: Check that the values of name are only of type xsd:string, rdf:langString or rdf:dirLangString according to SHACL 1.2
        if lits.is_empty() {
            return Err(RDFError::ParseFailError {
                msg: "No value for sh:name found".to_string(),
            });
        }
        let map = lits.into_iter().fold(MessageMap::new(), |acc, lit| {
            let lang = lit.lang();
            let text = lit.lexical_form().to_string();
            acc.with_message(lang, text)
        });
        Ok(map)
    })
}
