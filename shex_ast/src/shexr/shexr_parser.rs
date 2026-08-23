use super::shexr_error::ShExRError;
use crate::ast::iri_exclusion::IriExclusion;
use crate::ast::language_exclusion::LanguageExclusion;
use crate::ast::literal_exclusion::LiteralExclusion;
use crate::{
    Annotation, BNode, IriOrStr, IriRefOrWildcard, LangOrWildcard, NodeConstraint, NodeKind, ObjectValue, Schema,
    SemAct, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, StringOrWildcard, TripleExpr, TripleExprLabel, ValueSetValue,
    XsFacet,
};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::{parse_list_recursive, term_to_iri, term_to_number};
use rudof_rdf::rdf_core::term::literal::Lang;
use rudof_rdf::rdf_core::vocabs::ShexRVocab;
use rudof_rdf::rdf_core::{
    FocusRDF, RDFError,
    parser::{
        RDFParse,
        rdf_node_parser::{
            ParserExt, RDFNodeParse,
            constructors::{SingleInstanceParser, SingleIriPropertyParser, SingleValuePropertyParser},
        },
    },
    term::Object,
};

/// Parses RDF encoding a ShEx schema in the ShExR vocabulary (`sx:`,
/// `http://www.w3.org/ns/shex#`) back into a [`Schema`] (the ShExJ-shaped
/// AST) — the read direction matching [`super::shexr_builder::ShExRBuilder`].
///
/// Covers everything the builder produces: shapes (`Shape`,
/// `ShapeAnd`/`Or`/`Not`, `NodeConstraint`, `ShapeExternal`, references,
/// `extends`), triple expressions (`TripleConstraint`, `EachOf`, `OneOf`),
/// `extra`/`semActs`/`annotations`, IRI- and blank-node-labelled shapes, a
/// schema's `start`/`startActs`/`imports`, and value sets (plain values,
/// IRI/literal/language stems and stem ranges with exclusions).
///
/// Not supported: sharing/reuse of a `TripleExpr` across more than one
/// position via its own `id` (`sx:TripleConstraint`/`EachOf`/`OneOf` nodes
/// are always parsed inline, at every position they're referenced from,
/// rather than once-and-referenced) — a rare feature (a handful of fixtures
/// in the ShExTest corpus). `ShapeExprLabel::Start` (the `START` pseudo
/// label) also has no ShExR representation to parse back.
pub struct ShExRParser<RDF>
where
    RDF: FocusRDF,
{
    rdf_parser: RDFParse<RDF>,
}

impl<RDF> ShExRParser<RDF>
where
    RDF: FocusRDF + 'static,
{
    pub fn new(rdf: RDF) -> ShExRParser<RDF> {
        ShExRParser {
            rdf_parser: RDFParse::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<Schema, ShExRError> {
        let rdf = self.rdf_parser.rdf_mut();
        let schema_node = SingleInstanceParser::new(ShexRVocab::sx_schema())
            .parse_focused(rdf)
            .map_err(|e| ShExRError::RDFParseError { err: e })?;
        // `SingleInstanceParser` only *finds* the instance; it doesn't move
        // focus onto it (despite what its own doc comment implies).
        rdf.set_focus(&schema_node.into());
        let schema = parse_schema(rdf).map_err(|e| ShExRError::RDFParseError { err: e })?;
        let prefixmap = self.rdf_parser.prefixmap();
        Ok(schema.with_prefixmap(prefixmap))
    }

    /// Kept for source compatibility with earlier callers; delegates to the
    /// hand-written recursive-descent parser in [`parse`](Self::parse).
    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        SchemaEntryParser {
            _marker: std::marker::PhantomData,
        }
    }
}

struct SchemaEntryParser<RDF> {
    _marker: std::marker::PhantomData<RDF>,
}

impl<RDF> RDFNodeParse<RDF> for SchemaEntryParser<RDF>
where
    RDF: FocusRDF + 'static,
{
    type Output = Schema;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Schema, RDFError> {
        let schema_node = SingleInstanceParser::new(ShexRVocab::sx_schema()).parse_focused(rdf)?;
        rdf.set_focus(&schema_node.into());
        parse_schema(rdf)
    }
}

/// Converts a raw RDF term into the [`Object`] it denotes (IRI, blank node
/// or literal), the common first step for every "the current node's own
/// identity becomes a label" conversion below.
fn term_to_object<RDF: FocusRDF>(term: &RDF::Term) -> Result<Object, ShExRError> {
    term.clone()
        .try_into()
        .map_err(|_| ShExRError::TermToRDFNodeFailed { term: term.to_string() })
}

fn term_to_shape_label<RDF: FocusRDF>(term: &RDF::Term) -> Result<ShapeExprLabel, ShExRError> {
    match term_to_object::<RDF>(term)? {
        Object::Iri(iri) => Ok(ShapeExprLabel::iri(iri)),
        Object::BlankNode(bnode) => Ok(ShapeExprLabel::bnode(BNode::new(bnode.as_str()))),
        Object::Literal(lit) => Err(ShExRError::ShapeExprLabelLiteral { term: lit.to_string() }),
        Object::Triple { .. } => Err(ShExRError::TermToRDFNodeFailed { term: term.to_string() }),
    }
}

fn term_to_triple_expr_label<RDF: FocusRDF>(term: &RDF::Term) -> Result<TripleExprLabel, ShExRError> {
    match term_to_object::<RDF>(term)? {
        Object::Iri(iri) => Ok(TripleExprLabel::IriRef {
            value: IriRef::iri(iri),
        }),
        Object::BlankNode(bnode) => Ok(TripleExprLabel::BNode {
            value: BNode::new(bnode.as_str()),
        }),
        other => Err(ShExRError::TermToRDFNodeFailed {
            term: other.to_string(),
        }),
    }
}

fn term_to_object_value<RDF: FocusRDF>(term: &RDF::Term) -> Result<ObjectValue, ShExRError> {
    match term_to_object::<RDF>(term)? {
        Object::Iri(iri) => Ok(ObjectValue::iri(iri)),
        Object::Literal(lit) => Ok(ObjectValue::literal(lit)),
        other => Err(ShExRError::TermToRDFNodeFailed {
            term: other.to_string(),
        }),
    }
}

fn to_rdf_err(e: ShExRError) -> RDFError {
    RDFError::DefaultError { msg: e.to_string() }
}

/// Reads a required property whose value is itself a node to recurse into,
/// calling `parse` with the focus moved to that node, then restores the
/// original focus so the caller can keep reading sibling properties.
fn read_node_property<RDF, T>(
    rdf: &mut RDF,
    property: IriS,
    parse: impl FnOnce(&mut RDF) -> Result<T, RDFError>,
) -> Result<T, RDFError>
where
    RDF: FocusRDF,
{
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let value = SingleValuePropertyParser::new(property).parse_focused(rdf)?;
    rdf.set_focus(&value);
    let result = parse(rdf)?;
    rdf.set_focus(&node);
    Ok(result)
}

/// Same as [`read_node_property`] but the property is optional.
fn read_opt_node_property<RDF, T>(
    rdf: &mut RDF,
    property: IriS,
    parse: impl FnOnce(&mut RDF) -> Result<T, RDFError>,
) -> Result<Option<T>, RDFError>
where
    RDF: FocusRDF,
{
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let value = SingleValuePropertyParser::new(property).optional().parse_focused(rdf)?;
    match value {
        None => Ok(None),
        Some(term) => {
            rdf.set_focus(&term);
            let result = parse(rdf)?;
            rdf.set_focus(&node);
            Ok(Some(result))
        },
    }
}

/// Reads an RDF-list-valued property (`sx:shapes`, `sx:shapeExprs`, ...),
/// applying `parse` to each element in turn (with focus moved to it), and
/// restores the original focus afterwards. `None` if the property is absent.
fn read_list_property<RDF, T>(
    rdf: &mut RDF,
    property: IriS,
    mut parse: impl FnMut(&mut RDF) -> Result<T, RDFError>,
) -> Result<Option<Vec<T>>, RDFError>
where
    RDF: FocusRDF,
{
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let head = SingleValuePropertyParser::new(property).optional().parse_focused(rdf)?;
    match head {
        None => Ok(None),
        Some(head) => {
            rdf.set_focus(&head);
            let elems = parse_list_recursive::<RDF>(vec![head], rdf)?;
            let mut results = Vec::with_capacity(elems.len());
            for elem in elems {
                rdf.set_focus(&elem);
                results.push(parse(rdf)?);
            }
            rdf.set_focus(&node);
            Ok(Some(results))
        },
    }
}

fn parse_schema<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<Schema, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;

    let start = read_opt_node_property(rdf, ShexRVocab::sx_start(), parse_shape_expr)?;

    rdf.set_focus(&node);
    let imports = read_list_property(rdf, ShexRVocab::sx_imports(), |rdf| {
        let term = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
        let iri = term_to_iri::<RDF>(&term)?;
        Ok(IriOrStr::IriRef(IriRef::iri(iri)))
    })?;

    rdf.set_focus(&node);
    let start_acts = read_list_property(rdf, ShexRVocab::sx_start_acts(), parse_sem_act)?;

    rdf.set_focus(&node);
    let shapes = read_list_property(rdf, ShexRVocab::sx_shapes(), parse_shape_decl)?;

    let mut schema = Schema::new(&rudof_iri::iri!("http://default/"))
        .with_start(start)
        .with_start_actions(start_acts)
        .with_shapes(shapes);
    if let Some(imports) = imports {
        for import in imports {
            schema = schema.with_import(match import {
                IriOrStr::IriRef(iri_ref) => iri_ref,
                IriOrStr::String(s) => IriRef::iri(IriS::new_unchecked(&s)),
            });
        }
    }
    Ok(schema)
}

fn parse_shape_decl<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<ShapeDecl, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let label = term_to_shape_label::<RDF>(&node).map_err(to_rdf_err)?;

    let is_abstract = rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::SingleBoolPropertyParser::new(
        ShexRVocab::sx_abstract(),
    )
    .optional()
    .parse_focused(rdf)?
    .unwrap_or(false);

    rdf.set_focus(&node);
    let shape_expr = read_node_property(rdf, ShexRVocab::sx_shape_expr(), parse_shape_expr)?;

    Ok(ShapeDecl::new(label, shape_expr, is_abstract))
}

fn parse_shape_expr<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<ShapeExpr, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let type_iri = current_type::<RDF>(rdf)?;

    match type_iri.as_deref() {
        Some(ShexRVocab::SX_SHAPE_AND) => {
            let exprs = read_list_property(rdf, ShexRVocab::sx_shape_exprs(), parse_shape_expr)?.unwrap_or_default();
            Ok(ShapeExpr::and(exprs))
        },
        Some(ShexRVocab::SX_SHAPE_OR) => {
            let exprs = read_list_property(rdf, ShexRVocab::sx_shape_exprs(), parse_shape_expr)?.unwrap_or_default();
            Ok(ShapeExpr::or(exprs))
        },
        Some(ShexRVocab::SX_SHAPE_NOT) => {
            let inner = read_node_property(rdf, ShexRVocab::sx_shape_expr(), parse_shape_expr)?;
            Ok(ShapeExpr::shape_not(inner))
        },
        Some(ShexRVocab::SX_NODE_CONSTRAINT) => parse_node_constraint(rdf).map(ShapeExpr::node_constraint),
        Some(ShexRVocab::SX_SHAPE) => parse_shape(rdf).map(ShapeExpr::shape),
        Some(ShexRVocab::SX_SHAPE_EXTERNAL) => Ok(ShapeExpr::external()),
        _ => {
            // Not one of the inline shape-expression constructs (or no
            // `rdf:type` triple at all): this node is a reference to a
            // separately-declared `sx:ShapeDecl` (or an external/unresolved
            // label), matching the ShExR convention that a `ShapeDecl`
            // node's `rdf:type` is `sx:ShapeDecl`, never one of the above.
            let label = term_to_shape_label::<RDF>(&node).map_err(to_rdf_err)?;
            Ok(ShapeExpr::shape_ref(label))
        },
    }
}

fn current_type<RDF: FocusRDF>(rdf: &mut RDF) -> Result<Option<String>, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let type_term = SingleValuePropertyParser::new(rudof_rdf::rdf_core::vocabs::RdfVocab::rdf_type())
        .optional()
        .parse_focused(rdf)?;
    rdf.set_focus(&node);
    match type_term {
        None => Ok(None),
        Some(t) => Ok(term_to_iri::<RDF>(&t).ok().map(|iri| iri.as_str().to_string())),
    }
}

fn parse_shape<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<Shape, RDFError> {
    use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{IrisPropertyParser, SingleBoolPropertyParser};

    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;

    let closed = SingleBoolPropertyParser::new(ShexRVocab::sx_closed())
        .optional()
        .parse_focused(rdf)?;

    rdf.set_focus(&node);
    let extra: Option<Vec<IriRef>> = {
        let iris = IrisPropertyParser::new(ShexRVocab::sx_extra()).parse_focused(rdf)?;
        if iris.is_empty() {
            None
        } else {
            Some(iris.into_iter().map(IriRef::iri).collect())
        }
    };

    rdf.set_focus(&node);
    let expression = read_opt_node_property(rdf, ShexRVocab::sx_expression(), parse_triple_expr)?;

    rdf.set_focus(&node);
    let sem_acts = read_list_property(rdf, ShexRVocab::sx_sem_acts(), parse_sem_act)?;

    rdf.set_focus(&node);
    let annotations = read_list_property(rdf, ShexRVocab::sx_annotation_prop(), parse_annotation)?;

    rdf.set_focus(&node);
    let extends = read_list_property(rdf, ShexRVocab::sx_extends(), |rdf| {
        let term = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
        term_to_shape_label::<RDF>(&term).map_err(to_rdf_err)
    })?;

    Ok(Shape::new(closed, extra, expression)
        .with_sem_acts(sem_acts)
        .with_annotations(annotations)
        .with_extends(extends))
}

fn parse_triple_expr<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<TripleExpr, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let id = term_to_triple_expr_label::<RDF>(&node).ok();
    let type_iri = current_type::<RDF>(rdf)?;

    match type_iri.as_deref() {
        Some(ShexRVocab::SX_EACH_OF) => parse_triple_expr_group(rdf, id, TripleExpr::each_of),
        Some(ShexRVocab::SX_ONE_OF) => parse_triple_expr_group(rdf, id, TripleExpr::one_of),
        Some(ShexRVocab::SX_TRIPLE_CONSTRAINT) => parse_triple_constraint(rdf, id),
        other => Err(RDFError::DefaultError {
            msg: format!("Expected a triple expression (TripleConstraint/EachOf/OneOf) at {node}, found: {other:?}"),
        }),
    }
}

fn parse_triple_expr_group<RDF: FocusRDF + 'static>(
    rdf: &mut RDF,
    id: Option<TripleExprLabel>,
    make: impl FnOnce(Vec<TripleExpr>) -> TripleExpr,
) -> Result<TripleExpr, RDFError> {
    use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::SingleIntegerPropertyParser;

    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let exprs = read_list_property(rdf, ShexRVocab::sx_expressions(), parse_triple_expr)?.unwrap_or_default();

    rdf.set_focus(&node);
    let min = SingleIntegerPropertyParser::new(ShexRVocab::sx_min())
        .optional()
        .parse_focused(rdf)?
        .map(|n| n as i32);

    rdf.set_focus(&node);
    let max = SingleIntegerPropertyParser::new(ShexRVocab::sx_max())
        .optional()
        .parse_focused(rdf)?
        .map(|n| n as i32);

    rdf.set_focus(&node);
    let sem_acts = read_list_property(rdf, ShexRVocab::sx_sem_acts(), parse_sem_act)?;

    rdf.set_focus(&node);
    let annotations = read_list_property(rdf, ShexRVocab::sx_annotation_prop(), parse_annotation)?;

    Ok(make(exprs)
        .with_id(id)
        .with_min(min)
        .with_max(max)
        .with_sem_acts(sem_acts)
        .with_annotations(annotations))
}

fn parse_triple_constraint<RDF: FocusRDF + 'static>(
    rdf: &mut RDF,
    id: Option<TripleExprLabel>,
) -> Result<TripleExpr, RDFError> {
    use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
        SingleBoolPropertyParser, SingleIntegerPropertyParser,
    };

    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;

    let predicate = SingleIriPropertyParser::new(ShexRVocab::sx_predicate())
        .parse_focused(rdf)
        .map(IriRef::iri)?;

    rdf.set_focus(&node);
    let negated = SingleBoolPropertyParser::new(ShexRVocab::sx_negated())
        .optional()
        .parse_focused(rdf)?;

    rdf.set_focus(&node);
    let inverse = SingleBoolPropertyParser::new(ShexRVocab::sx_inverse())
        .optional()
        .parse_focused(rdf)?;

    rdf.set_focus(&node);
    let min = SingleIntegerPropertyParser::new(ShexRVocab::sx_min())
        .optional()
        .parse_focused(rdf)?
        .map(|n| n as i32);

    rdf.set_focus(&node);
    let max = SingleIntegerPropertyParser::new(ShexRVocab::sx_max())
        .optional()
        .parse_focused(rdf)?
        .map(|n| n as i32);

    rdf.set_focus(&node);
    let value_expr = read_opt_node_property(rdf, ShexRVocab::sx_value_expr(), parse_shape_expr)?;

    rdf.set_focus(&node);
    let sem_acts = read_list_property(rdf, ShexRVocab::sx_sem_acts(), parse_sem_act)?;

    rdf.set_focus(&node);
    let annotations = read_list_property(rdf, ShexRVocab::sx_annotation_prop(), parse_annotation)?;

    Ok(
        TripleExpr::triple_constraint(negated, inverse, predicate, value_expr, min, max)
            .with_id(id)
            .with_sem_acts(sem_acts)
            .with_annotations(annotations),
    )
}

fn parse_node_constraint<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<NodeConstraint, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let mut nc = NodeConstraint::new();

    if let Some(node_kind) = parse_node_kind(rdf)? {
        nc = nc.with_node_kind(node_kind);
    }

    rdf.set_focus(&node);
    if let Some(iri) = SingleIriPropertyParser::new(ShexRVocab::sx_datatype())
        .optional()
        .parse_focused(rdf)?
    {
        nc = nc.with_datatype(IriRef::iri(iri));
    }

    rdf.set_focus(&node);
    let facets = parse_facets(rdf)?;
    if !facets.is_empty() {
        nc = nc.with_xsfacets(facets);
    }

    rdf.set_focus(&node);
    if let Some(values) = read_list_property(rdf, ShexRVocab::sx_values(), parse_value_set_value)? {
        nc = nc.with_values(values);
    }

    Ok(nc)
}

fn parse_node_kind<RDF: FocusRDF>(rdf: &mut RDF) -> Result<Option<NodeKind>, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let value = SingleValuePropertyParser::new(ShexRVocab::sx_node_kind())
        .optional()
        .parse_focused(rdf)?;
    rdf.set_focus(&node);
    match value {
        None => Ok(None),
        Some(term) => {
            let iri = term_to_iri::<RDF>(&term)?;
            let nk = match iri.as_str() {
                ShexRVocab::SX_IRI => NodeKind::Iri,
                ShexRVocab::SX_LITERAL => NodeKind::Literal,
                ShexRVocab::SX_BNODE => NodeKind::BNode,
                ShexRVocab::SX_NON_LITERAL => NodeKind::NonLiteral,
                other => {
                    return Err(RDFError::DefaultError {
                        msg: format!("Unexpected sx:nodeKind value: {other}"),
                    });
                },
            };
            Ok(Some(nk))
        },
    }
}

fn parse_facets<RDF: FocusRDF>(rdf: &mut RDF) -> Result<Vec<XsFacet>, RDFError> {
    use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
        SingleIntegerPropertyParser, SingleStringPropertyParser,
    };

    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let mut facets = Vec::new();

    if let Some(n) = SingleIntegerPropertyParser::new(ShexRVocab::sx_length())
        .optional()
        .parse_focused(rdf)?
    {
        facets.push(XsFacet::length(n as usize));
    }
    rdf.set_focus(&node);
    if let Some(n) = SingleIntegerPropertyParser::new(ShexRVocab::sx_minlength())
        .optional()
        .parse_focused(rdf)?
    {
        facets.push(XsFacet::min_length(n as usize));
    }
    rdf.set_focus(&node);
    if let Some(n) = SingleIntegerPropertyParser::new(ShexRVocab::sx_maxlength())
        .optional()
        .parse_focused(rdf)?
    {
        facets.push(XsFacet::max_length(n as usize));
    }
    rdf.set_focus(&node);
    if let Some(pattern) = SingleStringPropertyParser::new(ShexRVocab::sx_pattern())
        .optional()
        .parse_focused(rdf)?
    {
        rdf.set_focus(&node);
        let flags = SingleStringPropertyParser::new(ShexRVocab::sx_flags())
            .optional()
            .parse_focused(rdf)?;
        facets.push(match flags {
            Some(f) => XsFacet::pattern_flags(&pattern, &f),
            None => XsFacet::pattern(&pattern),
        });
    }
    rdf.set_focus(&node);
    if let Some(n) = numeric_property(rdf, ShexRVocab::sx_mininclusive())? {
        facets.push(XsFacet::min_inclusive(n));
    }
    rdf.set_focus(&node);
    if let Some(n) = numeric_property(rdf, ShexRVocab::sx_minexclusive())? {
        facets.push(XsFacet::min_exclusive(n));
    }
    rdf.set_focus(&node);
    if let Some(n) = numeric_property(rdf, ShexRVocab::sx_maxinclusive())? {
        facets.push(XsFacet::max_inclusive(n));
    }
    rdf.set_focus(&node);
    if let Some(n) = numeric_property(rdf, ShexRVocab::sx_maxexclusive())? {
        facets.push(XsFacet::max_exclusive(n));
    }
    rdf.set_focus(&node);
    if let Some(n) = SingleIntegerPropertyParser::new(ShexRVocab::sx_totaldigits())
        .optional()
        .parse_focused(rdf)?
    {
        facets.push(XsFacet::totaldigits(n as usize));
    }
    rdf.set_focus(&node);
    if let Some(n) = SingleIntegerPropertyParser::new(ShexRVocab::sx_fractiondigits())
        .optional()
        .parse_focused(rdf)?
    {
        facets.push(XsFacet::fractiondigits(n as usize));
    }

    Ok(facets)
}

fn numeric_property<RDF: FocusRDF>(
    rdf: &mut RDF,
    property: IriS,
) -> Result<Option<rudof_rdf::rdf_core::term::literal::NumericLiteral>, RDFError> {
    match SingleValuePropertyParser::new(property).optional().parse_focused(rdf)? {
        None => Ok(None),
        Some(term) => Ok(Some(term_to_number::<RDF>(&term)?)),
    }
}

/// Reads a literal-valued property as a plain string (used for `sx:stem`
/// when it's a literal rather than `[a sx:Wildcard]`).
fn string_property<RDF: FocusRDF>(rdf: &mut RDF) -> Result<String, RDFError> {
    let term = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_string::<RDF>(&term)
}

fn is_wildcard<RDF: FocusRDF>(rdf: &mut RDF) -> Result<bool, RDFError> {
    Ok(current_type::<RDF>(rdf)?.as_deref() == Some(ShexRVocab::SX_WILDCARD))
}

fn parse_value_set_value<RDF: FocusRDF + 'static>(rdf: &mut RDF) -> Result<ValueSetValue, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let type_iri = current_type::<RDF>(rdf)?;

    match type_iri.as_deref() {
        Some(ShexRVocab::SX_IRI_STEM) => {
            let stem = read_stem_iri(rdf)?;
            Ok(ValueSetValue::IriStem { stem })
        },
        Some(ShexRVocab::SX_IRI_STEM_RANGE) => {
            let stem = read_stem_or_wildcard(rdf, |rdf| {
                let iri = iri_from_current_focus(rdf)?;
                Ok(IriRefOrWildcard::IriRef(iri))
            })?
            .unwrap_or(IriRefOrWildcard::Wildcard);
            rdf.set_focus(&node);
            let exclusions = read_list_property(rdf, ShexRVocab::sx_exclusion(), parse_iri_exclusion)?;
            Ok(ValueSetValue::IriStemRange { stem, exclusions })
        },
        Some(ShexRVocab::SX_LITERAL_STEM) => {
            let stem = read_stem_string(rdf)?;
            Ok(ValueSetValue::LiteralStem { stem })
        },
        Some(ShexRVocab::SX_LITERAL_STEM_RANGE) => {
            let stem = read_stem_or_wildcard(rdf, |rdf| string_property(rdf).map(StringOrWildcard::String))?
                .unwrap_or(StringOrWildcard::Wildcard);
            rdf.set_focus(&node);
            let exclusions = read_list_property(rdf, ShexRVocab::sx_exclusion(), parse_literal_exclusion)?;
            Ok(ValueSetValue::LiteralStemRange { stem, exclusions })
        },
        Some(ShexRVocab::SX_LANGUAGE) => {
            let lang = read_lang_property(rdf)?;
            Ok(ValueSetValue::Language { language_tag: lang })
        },
        Some(ShexRVocab::SX_LANGUAGE_STEM) => {
            let s = read_stem_string(rdf)?;
            let stem = if s.is_empty() {
                LangOrWildcard::Wildcard
            } else {
                LangOrWildcard::Lang(parse_lang(&s)?)
            };
            Ok(ValueSetValue::LanguageStem { stem })
        },
        Some(ShexRVocab::SX_LANGUAGE_STEM_RANGE) => {
            // A wildcard stem can be spelled either as `[a sx:Wildcard]`
            // (checked by `read_stem_or_wildcard` itself) or, in a few
            // corpus fixtures, as a bare empty string (see
            // `common::is_wildcard_vs_empty_string` for the write-direction
            // counterpart of this same ambiguity).
            let stem = read_stem_or_wildcard(rdf, |rdf| {
                let s = string_property(rdf)?;
                if s.is_empty() {
                    Ok(LangOrWildcard::Wildcard)
                } else {
                    Ok(LangOrWildcard::Lang(parse_lang(&s)?))
                }
            })?
            .unwrap_or(LangOrWildcard::Wildcard);
            rdf.set_focus(&node);
            let exclusions = read_list_property(rdf, ShexRVocab::sx_exclusion(), parse_language_exclusion)?;
            Ok(ValueSetValue::LanguageStemRange { stem, exclusions })
        },
        _ => {
            // A plain value: an IRI or a literal, no wrapper node.
            term_to_object_value::<RDF>(&node)
                .map(ValueSetValue::ObjectValue)
                .map_err(to_rdf_err)
        },
    }
}

/// Reads `sx:stem`, moving focus onto it and delegating to `parse` unless
/// the stem is the structured `[a sx:Wildcard]` marker (`None` for that
/// case, letting the caller substitute the right `Wildcard` variant).
fn read_stem_or_wildcard<RDF: FocusRDF, T>(
    rdf: &mut RDF,
    parse: impl FnOnce(&mut RDF) -> Result<T, RDFError>,
) -> Result<Option<T>, RDFError> {
    read_opt_node_property(rdf, ShexRVocab::sx_stem(), |rdf| {
        if is_wildcard(rdf)? {
            Ok(None)
        } else {
            parse(rdf).map(Some)
        }
    })
    .map(Option::flatten)
}

/// Reads the current focus node's own text as an `IriRef` (a stem value,
/// which — unlike a "real" IRI reference elsewhere — is a plain string
/// literal, e.g. `sx:stem "http://a.example/v"`, per the ShExR convention).
fn iri_from_current_focus<RDF: FocusRDF>(rdf: &mut RDF) -> Result<IriRef, RDFError> {
    let s = string_property(rdf)?;
    Ok(IriRef::iri(IriS::new_unchecked(&s)))
}

/// Navigates into `sx:stem` and reads it as an `IriRef` (see
/// [`iri_from_current_focus`]).
fn read_stem_iri<RDF: FocusRDF>(rdf: &mut RDF) -> Result<IriRef, RDFError> {
    read_node_property(rdf, ShexRVocab::sx_stem(), iri_from_current_focus)
}

/// Navigates into `sx:stem` and reads it as a plain string.
fn read_stem_string<RDF: FocusRDF>(rdf: &mut RDF) -> Result<String, RDFError> {
    read_node_property(rdf, ShexRVocab::sx_stem(), string_property)
}

fn read_lang_property<RDF: FocusRDF>(rdf: &mut RDF) -> Result<Lang, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let s = SingleValuePropertyParser::new(ShexRVocab::sx_language_tag())
        .parse_focused(rdf)
        .and_then(|term| rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_string::<RDF>(&term))?;
    rdf.set_focus(&node);
    parse_lang(&s)
}

fn parse_lang(s: &str) -> Result<Lang, RDFError> {
    Lang::new(s).map_err(|e| RDFError::DefaultError {
        msg: format!("Invalid language tag {s}: {e}"),
    })
}

fn parse_iri_exclusion<RDF: FocusRDF>(rdf: &mut RDF) -> Result<IriExclusion, RDFError> {
    if current_type::<RDF>(rdf)?.as_deref() == Some(ShexRVocab::SX_IRI_STEM) {
        let iri = read_stem_iri(rdf)?;
        Ok(IriExclusion::IriStem(iri))
    } else {
        let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
        let iri = term_to_iri::<RDF>(&node)?;
        Ok(IriExclusion::Iri(IriRef::iri(iri)))
    }
}

fn parse_literal_exclusion<RDF: FocusRDF>(rdf: &mut RDF) -> Result<LiteralExclusion, RDFError> {
    if current_type::<RDF>(rdf)?.as_deref() == Some(ShexRVocab::SX_LITERAL_STEM) {
        let s = read_stem_string(rdf)?;
        Ok(LiteralExclusion::LiteralStem(s))
    } else {
        let s = string_property(rdf)?;
        Ok(LiteralExclusion::Literal(s))
    }
}

fn parse_language_exclusion<RDF: FocusRDF>(rdf: &mut RDF) -> Result<LanguageExclusion, RDFError> {
    if current_type::<RDF>(rdf)?.as_deref() == Some(ShexRVocab::SX_LANGUAGE_STEM) {
        let s = read_stem_string(rdf)?;
        Ok(LanguageExclusion::LanguageStem(parse_lang(&s)?))
    } else {
        let s = string_property(rdf)?;
        Ok(LanguageExclusion::Language(parse_lang(&s)?))
    }
}

fn parse_sem_act<RDF: FocusRDF>(rdf: &mut RDF) -> Result<SemAct, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let name = SingleIriPropertyParser::new(ShexRVocab::sx_name()).parse_focused(rdf)?;
    rdf.set_focus(&node);
    use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::SingleStringPropertyParser;
    let code = SingleStringPropertyParser::new(ShexRVocab::sx_code())
        .optional()
        .parse_focused(rdf)?;
    Ok(SemAct::new(IriRef::iri(name), code))
}

fn parse_annotation<RDF: FocusRDF>(rdf: &mut RDF) -> Result<Annotation, RDFError> {
    let node = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
    let predicate = SingleIriPropertyParser::new(ShexRVocab::sx_predicate()).parse_focused(rdf)?;
    rdf.set_focus(&node);
    let object_term = SingleValuePropertyParser::new(ShexRVocab::sx_object()).parse_focused(rdf)?;
    let object = term_to_object_value::<RDF>(&object_term).map_err(to_rdf_err)?;
    Ok(Annotation::new(IriRef::iri(predicate), object))
}
