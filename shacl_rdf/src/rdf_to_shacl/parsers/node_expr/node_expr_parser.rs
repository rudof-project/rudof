use crate::rdf_to_shacl::parsers::{node_shape, property_shape};
use iri_s::IriS;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    ListParser, ShaclPathParser, SingleIntegerPropertyParser, SingleIriOrBlankNodePropertyParser,
    SingleIriPropertyParser, SingleStringPropertyParser, SingleValuePropertyAsListParser, SingleValuePropertyParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use shacl_ast::NodeExpr;
use shacl_ast::shape::Shape;

pub(crate) struct NodeExprParser<RDF: FocusRDF> {
    term: RDF::Term,
}

impl<RDF: FocusRDF> NodeExprParser<RDF> {
    pub fn new(term: RDF::Term) -> Self {
        Self { term }
    }

    fn parse_bnode_expr(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        if let Ok(empty) = Self::try_empty(rdf) {
            return Ok(empty);
        }
        if let Ok(var) = Self::try_var(rdf) {
            return Ok(var);
        }
        if let Ok(list) = Self::try_list(rdf) {
            return Ok(list);
        }
        if let Ok(path) = Self::try_path(rdf) {
            return Ok(path);
        }
        if let Ok(exists) = Self::try_exists(rdf) {
            return Ok(exists);
        }
        if let Ok(if_) = Self::try_if(rdf) {
            return Ok(if_);
        }

        if let Ok(distinct) = Self::try_distinct(rdf) {
            return Ok(distinct);
        }
        if let Ok(intersection) = Self::try_intersection(rdf) {
            return Ok(intersection);
        }
        if let Ok(concat) = Self::try_concat(rdf) {
            return Ok(concat);
        }
        if let Ok(remove) = Self::try_remove(rdf) {
            return Ok(remove);
        }
        if let Ok(filter) = Self::try_filter(rdf) {
            return Ok(filter);
        }
        if let Ok(limit) = Self::try_limit(rdf) {
            return Ok(limit);
        }
        if let Ok(offset) = Self::try_offset(rdf) {
            return Ok(offset);
        }
        // TODO - Order by (not in spec)

        if let Ok(flat_map) = Self::try_flat_map(rdf) {
            return Ok(flat_map);
        }
        if let Ok(find_first) = Self::try_find_first(rdf) {
            return Ok(find_first);
        }
        if let Ok(match_all) = Self::try_match_all(rdf) {
            return Ok(match_all);
        }

        if let Ok(count) = Self::try_count(rdf) {
            return Ok(count);
        }
        if let Ok(min) = Self::try_min(rdf) {
            return Ok(min);
        }
        if let Ok(max) = Self::try_max(rdf) {
            return Ok(max);
        }
        if let Ok(sum) = Self::try_sum(rdf) {
            return Ok(sum);
        }

        if let Ok(instances_of) = Self::try_instances_of(rdf) {
            return Ok(instances_of);
        }
        if let Ok(nodes_matching) = Self::try_nodes_matching(rdf) {
            return Ok(nodes_matching);
        }

        Err(RDFError::ConversionError {
            msg: "Blank node does not match any known node expression type".to_string(), // TODO - Add custom error
        })
    }
}

impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn node_expr_list(iri: IriS, rdf: &mut RDF) -> Result<Vec<NodeExpr<RDF>>, RDFError> {
        let list = SingleValuePropertyAsListParser::new(iri).parse_focused(rdf)?;

        list.into_iter()
            .map(|nexpr| NodeExprParser::new(nexpr).parse_focused(rdf))
            .collect::<Result<Vec<_>, _>>()
    }
}

// Misc node expressions
impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn try_instances_of(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let instance_of = SingleIriPropertyParser::new(ShaclNodeExprVocab::shnex_instances_of().clone())
            .parse_focused(rdf)
            .map(IriRef::Iri)?;

        Ok(NodeExpr::InstancesOf(instance_of))
    }

    fn try_nodes_matching(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let nodes_matching =
            SingleIriOrBlankNodePropertyParser::new(ShaclNodeExprVocab::shnex_nodes_matching().clone())
                .parse_focused(rdf)?;

        Ok(NodeExpr::NodesMatching(nodes_matching))
    }
}

// Aggregation Expressions
impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn try_count(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let count = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_count().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Count(count))
    }

    fn try_min(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let min = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_min().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Min(min))
    }

    fn try_max(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let max = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_max().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Max(max))
    }

    fn try_sum(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let sum = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_sum().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Sum(sum))
    }
}

/// Advanced Sequence Operations
impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn try_flat_map(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let flat_map = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_flat_map().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;
        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::FlatMap { flat_map, nodes })
    }

    fn try_find_first(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let find_first =
            SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_find_first().clone()).parse_focused(rdf)?;

        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        rdf.set_focus(&find_first);

        let result = property_shape().parse_focused(rdf);

        if let Ok(result) = result {
            Ok(NodeExpr::FindFirst {
                nodes,
                find_first: Shape::PropertyShape(Box::new(result)),
            })
        } else {
            let result = node_shape().parse_focused(rdf);

            if let Ok(result) = result {
                Ok(NodeExpr::FindFirst {
                    nodes,
                    find_first: Shape::NodeShape(Box::new(result)),
                })
            } else {
                Err(RDFError::ConversionError {
                    msg: "Unable to parse find first shape".to_string(),
                })
            }
        }
    }

    fn try_match_all(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let match_all =
            SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_match_all().clone()).parse_focused(rdf)?;

        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        rdf.set_focus(&match_all);

        let result = property_shape().parse_focused(rdf);

        if let Ok(result) = result {
            Ok(NodeExpr::MatchAll {
                nodes,
                match_all: Shape::PropertyShape(Box::new(result)),
            })
        } else {
            let result = node_shape().parse_focused(rdf);

            if let Ok(result) = result {
                Ok(NodeExpr::MatchAll {
                    nodes,
                    match_all: Shape::NodeShape(Box::new(result)),
                })
            } else {
                Err(RDFError::ConversionError {
                    msg: "Unable to parse match all shape".to_string(),
                })
            }
        }
    }
}

/// List Operator Expressions
impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn try_distinct(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let distinct = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_distinct().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Distinct(distinct))
    }

    // TODO - Test
    fn try_intersection(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let nodes = Self::node_expr_list(ShaclNodeExprVocab::shnex_intersection().clone(), rdf)?;

        Ok(NodeExpr::Intersection(nodes))
    }

    // TODO - Test
    fn try_concat(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let nodes = Self::node_expr_list(ShaclNodeExprVocab::shnex_concat().clone(), rdf)?;

        Ok(NodeExpr::Concat(nodes))
    }

    fn try_remove(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let remove = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_remove().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Remove { remove, nodes })
    }

    fn try_filter(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let shape =
            SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_filter_shape().clone()).parse_focused(rdf)?;

        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        rdf.set_focus(&shape);

        let result = property_shape().parse_focused(rdf);

        if let Ok(result) = result {
            Ok(NodeExpr::Filter {
                nodes,
                filter_shape: Shape::PropertyShape(Box::new(result)),
            })
        } else {
            let result = node_shape().parse_focused(rdf);
            if let Ok(result) = result {
                Ok(NodeExpr::Filter {
                    nodes,
                    filter_shape: Shape::NodeShape(Box::new(result)),
                })
            } else {
                Err(RDFError::ConversionError {
                    msg: "Unable to parse filter shape".to_string(),
                })
            }
        }
    }

    fn try_limit(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let limit: usize = SingleIntegerPropertyParser::new(ShaclNodeExprVocab::shnex_limit().clone())
            .parse_focused(rdf)?
            .try_into()
            .map_err(|err| RDFError::ConversionError {
                msg: format!("Unable to convert limit value to usize: {err}"),
            })?;
        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Limit { limit, nodes })
    }

    fn try_offset(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let offset: usize = SingleIntegerPropertyParser::new(ShaclNodeExprVocab::shnex_offset().clone())
            .parse_focused(rdf)?
            .try_into()
            .map_err(|err| RDFError::ConversionError {
                msg: format!("Unable to convert offset value to usize: {err}"),
            })?;
        let nodes = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_nodes().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Offset { offset, nodes })
    }

    // TODO - Order by (not in spec)
}

/// Basic Node Expressions
// TODO - Move to separate files and maybe move to parser functions? (like the other SHACL parsers)
impl<RDF: FocusRDF> NodeExprParser<RDF> {
    fn try_empty(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let term = rdf.get_focus().ok_or(RDFError::NoFocusNodeError)?;
        let subject = RDF::term_as_subject(term).map_err(|_| RDFError::ExpectedSubjectError {
            node: format!("{term}"),
            context: "try_basic_expr".to_string(),
        })?;

        let count = rdf
            .triples_with_subject(&subject)
            .map_err(|e| RDFError::ConversionError {
                msg: format!("Error querying triples: {e}"),
            })?;

        if count.count() == 0 {
            return Ok(NodeExpr::Empty);
        }

        Err(RDFError::ConversionError {
            msg: "Blank node is not an empty node expression".to_string(),
        })
    }

    fn try_var(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let term = SingleStringPropertyParser::new(ShaclNodeExprVocab::shnex_var().clone()).parse_focused(rdf)?;
        Ok(NodeExpr::Var(term))
    }

    fn try_list(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let list = ListParser::new()
            .parse_focused(rdf)?
            .iter()
            .map(|term| RDF::term_as_object(term))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(NodeExpr::List(list))
    }

    fn try_path(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let path = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_path_values().clone())
            .then(ShaclPathParser::new)
            .parse_focused(rdf)?;

        let focus_node = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_focus_node().clone())
            .then(NodeExprParser::new)
            .optional()
            .parse_focused(rdf)?
            .map(Box::new);

        Ok(NodeExpr::PathValues { path, focus_node })
    }

    fn try_exists(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let nexpr = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_exists().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        Ok(NodeExpr::Exists(nexpr))
    }

    fn try_if(rdf: &mut RDF) -> Result<NodeExpr<RDF>, RDFError> {
        let if_condition = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_if().clone())
            .then(NodeExprParser::new)
            .parse_focused(rdf)
            .map(Box::new)?;

        let then = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_then().clone())
            .then(NodeExprParser::new)
            .optional()
            .parse_focused(rdf)?;

        let else_ = SingleValuePropertyParser::new(ShaclNodeExprVocab::shnex_else().clone())
            .then(NodeExprParser::new)
            .optional()
            .parse_focused(rdf)?;

        // TODO - Maybe needs to be in validation?
        // if then.is_none() && else_.is_none() {
        //     return Err(RDFError::ConversionError {
        //         msg: "shnex:IfExpression requires at least one of shnex:then or shnex:else".to_string(),
        //     });
        // }

        Ok(NodeExpr::IfExpression {
            if_condition,
            then: then.map(Box::new),
            else_expression: else_.map(Box::new),
        })
    }
}

impl<RDF: FocusRDF> RDFNodeParse<RDF> for NodeExprParser<RDF> {
    type Output = NodeExpr<RDF>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let term = &self.term;

        if let Ok(iri) = RDF::term_as_iri(term) {
            return Ok(NodeExpr::Iri(iri.into().into()));
        }

        if let Ok(literal) = RDF::term_as_sliteral(term) {
            return Ok(NodeExpr::Literal(literal));
        }

        if RDF::term_as_bnode(term).is_ok() {
            rdf.set_focus(term);
            return Self::parse_bnode_expr(rdf);
        }

        Err(RDFError::ConversionError {
            // TODO - Add custom error
            msg: "Node expression cannot be a triple".to_string(),
        })
    }
}

// TODO - Add more tests
#[cfg(test)]
mod tests {
    use super::NodeExprParser;
    use iri_s::IriS;
    use prefixmap::IriRef;
    use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
    use shacl_ast::NodeExpr;

    #[test]
    fn test_empty_bnode() {
        let turtle = r#"
            @prefix ex: <http://example.org/> .

            ex:shape ex:ref [] .
        "#;

        let mut graph = InMemoryGraph::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

        let ex_ref = IriS::new_unchecked("http://example.org/ref");
        let ex_shape = IriS::new_unchecked("http://example.org/shape");
        let objects = graph.objects_for(&ex_shape.into(), &ex_ref.into()).unwrap();

        let bnode_term = objects.into_iter().next().unwrap();

        let parser = NodeExprParser::new(bnode_term);
        let result = parser.parse_focused(&mut graph).unwrap();

        assert!(matches!(result, NodeExpr::Empty));
    }

    #[test]
    fn test_non_empty_bnode_fails() {
        let turtle = r#"
            @prefix ex: <http://example.org/> .
            ex:shape ex:ref _:b0 .
            _:b0 ex:value "hello" .
        "#;

        let mut graph = InMemoryGraph::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

        let ex_ref = IriS::new_unchecked("http://example.org/ref");
        let ex_shape = IriS::new_unchecked("http://example.org/shape");
        let objects = graph.objects_for(&ex_shape.into(), &ex_ref.into()).unwrap();

        let bnode_term = objects.into_iter().next().unwrap();

        let parser = NodeExprParser::new(bnode_term);
        let result = parser.parse_focused(&mut graph);

        assert!(result.is_err());
    }

    #[test]
    fn test_iri_node_expr() {
        let turtle = r#"
            @prefix ex: <http://example.org/> .

            ex:shape ex:ref ex:someNode .
        "#;

        let mut graph = InMemoryGraph::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

        let ex_ref = IriS::new_unchecked("http://example.org/ref");
        let ex_shape = IriS::new_unchecked("http://example.org/shape");
        let objects = graph.objects_for(&ex_shape.into(), &ex_ref.into()).unwrap();

        let iri_term = objects.into_iter().next().unwrap();

        let parser = NodeExprParser::new(iri_term);
        let result = parser.parse_focused(&mut graph).unwrap();

        assert!(
            matches!(result, NodeExpr::Iri(iri) if iri == IriRef::Iri(IriS::new_unchecked("http://example.org/someNode")))
        );
    }

    #[test]
    fn test_literal_node_expr() {
        let turtle = r#"
            @prefix ex: <http://example.org/> .

            ex:shape ex:ref "hello" .
        "#;

        let mut graph = InMemoryGraph::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

        let ex_ref = IriS::new_unchecked("http://example.org/ref");
        let ex_shape = IriS::new_unchecked("http://example.org/shape");
        let objects = graph.objects_for(&ex_shape.into(), &ex_ref.into()).unwrap();

        let literal_term = objects.into_iter().next().unwrap();

        let parser = NodeExprParser::new(literal_term);
        let result = parser.parse_focused(&mut graph).unwrap();

        assert!(matches!(result, NodeExpr::Literal(h) if h.lexical_form() == "hello"));
    }

    #[test]
    fn test_var_node_expr() {
        let turtle = r#"
            @prefix ex: <http://example.org/> .
            @prefix shnex: <http://www.w3.org/ns/shacl-node-expr#> .

            ex:shape ex:ref [ shnex:var "focusNode" ] .
        "#;

        let mut graph = InMemoryGraph::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap();

        let ex_ref = IriS::new_unchecked("http://example.org/ref");
        let ex_shape = IriS::new_unchecked("http://example.org/shape");
        let objects = graph.objects_for(&ex_shape.into(), &ex_ref.into()).unwrap();

        let bnode_term = objects.into_iter().next().unwrap();

        let parser = NodeExprParser::new(bnode_term);
        let result = parser.parse_focused(&mut graph).unwrap();

        assert!(matches!(result, NodeExpr::Var(s) if s == "focusNode"));
    }
}
