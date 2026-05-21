use crate::ast::ASTComponent;
use crate::rdf::parsers::non_shape::message;
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    SingleBoolPropertyParser, SingleStringPropertyParser, ValuesPropertyParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::{OwlVocab, ShaclVocab};
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use std::collections::HashSet;
use std::marker::PhantomData;

struct BasicSparqlConstraintParser<RDF>(PhantomData<RDF>);

impl<RDF: FocusRDF> RDFNodeParse<RDF> for BasicSparqlConstraintParser<RDF> {
    type Output = Vec<ASTComponent>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let fshape = rdf.get_focus().ok_or(RDFError::NoFocusNodeError)?.clone();

        let constraint_nodes = ValuesPropertyParser::new(ShaclVocab::sh_sparql()).parse_focused(rdf)?;

        let mut result = Vec::new();

        for node in constraint_nodes {
            rdf.set_focus(&node);

            let select = SingleStringPropertyParser::new(ShaclVocab::sh_select()).parse_focused(rdf)?;

            let message = message().optional().parse_focused(rdf)?;

            let deactivated = SingleBoolPropertyParser::new(ShaclVocab::sh_deactivated())
                .optional()
                .parse_focused(rdf)?;

            let mut pm = PrefixMap::new();
            let prefix_nodes = ValuesPropertyParser::new(ShaclVocab::sh_prefixes())
                .parse_focused(rdf)
                .unwrap_or_default();

            for pn in prefix_nodes {
                // Changes focus node, should be restored afterwards
                collect_prefixes(rdf, &pn, &mut pm, &mut HashSet::new());
            }
            rdf.set_focus(&node);

            let prefixes_opt = if pm.is_empty() { None } else { Some(pm) };

            result.push(ASTComponent::BasicSparql {
                prefixes: prefixes_opt,
                deactivated,
                select,
                message,
            })
        }

        rdf.set_focus(&fshape);
        Ok(result)
    }
}

/// Recursively collects `(prefix, namespace)` pairs from a SHACL prefix ontology node.
///
/// Follows `sh:declare` to pick up `sh:prefix` / `sh:namespace` pairs, and
/// `owl:imports` to include prefixes declared in imported ontologies.
fn collect_prefixes<RDF: FocusRDF>(
    rdf: &mut RDF,
    node: &RDF::Term,
    out: &mut PrefixMap,
    visited: &mut HashSet<String>,
) {
    let node_str = node.to_string();
    if !visited.insert(node_str) {
        return;
    }

    rdf.set_focus(node);

    // sh:declare: bnodes with sh:prefix and sh:namespace
    if let Ok(decl_nodes) = ValuesPropertyParser::new(ShaclVocab::sh_declare()).parse_focused(rdf) {
        for decl in decl_nodes {
            rdf.set_focus(&decl);
            let prefix = SingleStringPropertyParser::new(ShaclVocab::sh_prefix()).parse_focused(rdf);
            let ns = SingleStringPropertyParser::new(ShaclVocab::sh_namespace()).parse_focused(rdf);
            if let (Ok(p), Ok(n)) = (prefix, ns)
                && let Ok(iri) = IriS::new(&n)
            {
                out.add_prefix(&p, iri);
            }
        }
        rdf.set_focus(node);
    }

    // owl:imports
    if let Ok(import_nodes) = ValuesPropertyParser::new(OwlVocab::owl_imports()).parse_focused(rdf) {
        for imp in import_nodes {
            collect_prefixes(rdf, &imp, out, visited);
        }
        rdf.set_focus(node);
    }
}

pub(crate) fn basic_sparql<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    BasicSparqlConstraintParser(PhantomData)
}
