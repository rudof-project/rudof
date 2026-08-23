mod error;

pub use error::ShExRBuilderError;

use crate::ast::iri_exclusion::IriExclusion;
use crate::ast::language_exclusion::LanguageExclusion;
use crate::ast::literal_exclusion::LiteralExclusion;
use crate::{
    Annotation, IriRefOrWildcard, LangOrWildcard, NodeConstraint, NodeKind, NumericFacet, ObjectValue, Schema, SemAct,
    Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, ShapeExprWrapper, StringFacet, StringOrWildcard, TripleExpr,
    TripleExprLabel, TripleExprWrapper, ValueSetValue, XsFacet,
};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShexRVocab};
use rudof_rdf::rdf_core::{BuildRDF, Rdf};
use std::collections::HashMap;

/// Serializes a ShEx [`Schema`] (the ShExJ-shaped AST) to RDF using the ShExR
/// vocabulary (`sx:`, `http://www.w3.org/ns/shex#`).
///
/// Covers shapes (`Shape`, `ShapeAnd`/`Or`/`Not`, `NodeConstraint`,
/// `ShapeExternal`, plain references, `extends`), triple expressions
/// (`TripleConstraint`, `EachOf`, `OneOf`, references), `extra`/`semActs`/
/// `annotations`, IRI- and blank-node-labelled shapes/triple expressions,
/// value sets (plain values, IRI/literal/language stems and stem ranges with
/// exclusions), and a schema's `start`/`startActs`/`imports`. Not supported:
/// `ShapeExprLabel::Start`. Anything outside that returns
/// [`ShExRBuilderError::Unsupported`] rather than
/// emitting incomplete or incorrect RDF.
pub struct ShExRBuilder;

impl ShExRBuilder {
    /// Serializes `schema` into `writer`, returning the term identifying the
    /// new `sx:Schema` node (always a fresh blank node).
    pub fn schema_to_rdf<RDF: BuildRDF>(schema: &Schema, writer: &mut RDF) -> Result<RDF::Term, ShExRBuilderError> {
        Ctx::new(schema, writer).schema_to_rdf()
    }
}

/// Per-call build state: the schema being serialized, the RDF graph being
/// written to, and a cache of blank nodes already allocated for a given
/// ShExJ blank-node label (`_:b1` etc.) so a label referenced more than once
/// — e.g. a `TripleConstraint`'s `id` and a later `Ref` to that same id —
/// resolves to the *same* RDF blank node rather than two different ones.
struct Ctx<'a, RDF: BuildRDF> {
    schema: &'a Schema,
    writer: &'a mut RDF,
    bnodes: HashMap<String, RDF::Subject>,
}

impl<'a, RDF: BuildRDF> Ctx<'a, RDF> {
    fn new(schema: &'a Schema, writer: &'a mut RDF) -> Self {
        Ctx {
            schema,
            writer,
            bnodes: HashMap::new(),
        }
    }

    fn bnode(&mut self) -> Result<RDF::Subject, ShExRBuilderError> {
        Ok(self
            .writer
            .add_bnode()
            .map_err(error_mapper::<RDF>("Error creating bnode"))?
            .into())
    }

    fn bnode_for_label(&mut self, label: &str) -> Result<RDF::Subject, ShExRBuilderError> {
        if let Some(node) = self.bnodes.get(label) {
            return Ok(node.clone());
        }
        let node = self.bnode()?;
        self.bnodes.insert(label.to_string(), node.clone());
        Ok(node)
    }

    fn add_type(&mut self, node: RDF::Subject, ty: IriS) -> Result<(), ShExRBuilderError> {
        self.writer
            .add_type(node, ty)
            .map_err(error_mapper::<RDF>("Error adding rdf:type"))
    }

    fn add_triple(
        &mut self,
        subj: RDF::Subject,
        pred: IriS,
        obj: impl Into<RDF::Term>,
    ) -> Result<(), ShExRBuilderError> {
        self.writer
            .add_triple(subj, pred, obj.into())
            .map_err(error_mapper::<RDF>("Error adding triple"))
    }

    fn schema_to_rdf(&mut self) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_schema())?;
        if let Some(start) = self.schema.start() {
            let start_node = self.shape_expr_to_rdf(&start)?;
            self.add_triple(node.clone(), ShexRVocab::sx_start(), start_node)?;
        }
        if let Some(imports) = self.schema.imports() {
            let list = self.list_to_rdf(&imports, Self::import_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_imports(), list)?;
        }
        if let Some(acts) = self.schema.start_actions() {
            let list = self.list_to_rdf(&acts, Self::sem_act_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_start_acts(), list)?;
        }
        if let Some(shapes) = self.schema.shapes() {
            let list = self.list_to_rdf(&shapes, Self::shape_decl_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_shapes(), list)?;
        }
        Ok(node.into())
    }

    fn shape_decl_to_rdf(&mut self, decl: &ShapeDecl) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.label_to_subject(decl.id())?;
        self.add_type(node.clone(), ShexRVocab::sx_shape_decl())?;
        if decl.is_abstract {
            self.add_triple(node.clone(), ShexRVocab::sx_abstract(), Object::boolean(true))?;
        }
        let shape_expr = self.shape_expr_to_rdf(&decl.shape_expr)?;
        self.add_triple(node.clone(), ShexRVocab::sx_shape_expr(), shape_expr)?;
        Ok(node.into())
    }

    fn shape_expr_to_rdf(&mut self, se: &ShapeExpr) -> Result<RDF::Term, ShExRBuilderError> {
        match se {
            ShapeExpr::Shape(shape) => self.shape_to_rdf(shape),
            ShapeExpr::Ref(label) => self.label_to_subject(label).map(Into::into),
            ShapeExpr::NodeConstraint(nc) => self.node_constraint_to_rdf(nc),
            ShapeExpr::ShapeAnd { shape_exprs } => self.shape_algebra_to_rdf(ShexRVocab::sx_shape_and(), shape_exprs),
            ShapeExpr::ShapeOr { shape_exprs } => self.shape_algebra_to_rdf(ShexRVocab::sx_shape_or(), shape_exprs),
            ShapeExpr::ShapeNot { shape_expr } => {
                let node = self.bnode()?;
                self.add_type(node.clone(), ShexRVocab::sx_shape_not())?;
                let inner = self.shape_expr_to_rdf(&shape_expr.se)?;
                self.add_triple(node.clone(), ShexRVocab::sx_shape_expr(), inner)?;
                Ok(node.into())
            },
            ShapeExpr::External => {
                let node = self.bnode()?;
                self.add_type(node.clone(), ShexRVocab::sx_shape_external())?;
                Ok(node.into())
            },
        }
    }

    fn shape_algebra_to_rdf(&mut self, ty: IriS, exprs: &[ShapeExprWrapper]) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ty)?;
        let list = self.list_to_rdf(exprs, |ctx, w| ctx.shape_expr_to_rdf(&w.se))?;
        self.add_triple(node.clone(), ShexRVocab::sx_shape_exprs(), list)?;
        Ok(node.into())
    }

    fn shape_to_rdf(&mut self, shape: &Shape) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_shape())?;
        if shape.closed == Some(true) {
            self.add_triple(node.clone(), ShexRVocab::sx_closed(), Object::boolean(true))?;
        }
        if let Some(extends) = &shape.extends {
            let list = self.list_to_rdf(extends, Self::label_to_subject_term)?;
            self.add_triple(node.clone(), ShexRVocab::sx_extends(), list)?;
        }
        if let Some(extra) = &shape.extra {
            for iri_ref in extra {
                let iri = self.schema.resolve_iriref(iri_ref);
                self.add_triple(node.clone(), ShexRVocab::sx_extra(), iri)?;
            }
        }
        if let Some(te) = shape.triple_expr() {
            let te_node = self.triple_expr_to_rdf(&te)?;
            self.add_triple(node.clone(), ShexRVocab::sx_expression(), te_node)?;
        }
        if let Some(sem_acts) = &shape.sem_acts {
            let list = self.list_to_rdf(sem_acts, Self::sem_act_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_sem_acts(), list)?;
        }
        if let Some(annotations) = &shape.annotations {
            let list = self.list_to_rdf(annotations, Self::annotation_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_annotation_prop(), list)?;
        }
        Ok(node.into())
    }

    fn triple_expr_to_rdf(&mut self, te: &TripleExpr) -> Result<RDF::Term, ShExRBuilderError> {
        match te {
            TripleExpr::Ref(label) => self.triple_expr_label_to_subject(label).map(Into::into),
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => self.triple_expr_group_to_rdf(
                ShexRVocab::sx_each_of(),
                id,
                expressions,
                *min,
                *max,
                sem_acts,
                annotations,
            ),
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => self.triple_expr_group_to_rdf(
                ShexRVocab::sx_one_of(),
                id,
                expressions,
                *min,
                *max,
                sem_acts,
                annotations,
            ),
            TripleExpr::TripleConstraint {
                id,
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let node = match id {
                    Some(label) => self.triple_expr_label_to_subject(label)?,
                    None => self.bnode()?,
                };
                self.add_type(node.clone(), ShexRVocab::sx_triple_constraint())?;
                let predicate_iri = self.schema.resolve_iriref(predicate);
                self.add_triple(node.clone(), ShexRVocab::sx_predicate(), predicate_iri)?;
                if *negated == Some(true) {
                    self.add_triple(node.clone(), ShexRVocab::sx_negated(), Object::boolean(true))?;
                }
                if *inverse == Some(true) {
                    self.add_triple(node.clone(), ShexRVocab::sx_inverse(), Object::boolean(true))?;
                }
                if let Some(se) = value_expr {
                    let value_expr_node = self.shape_expr_to_rdf(se)?;
                    self.add_triple(node.clone(), ShexRVocab::sx_value_expr(), value_expr_node)?;
                }
                if let Some(min) = min {
                    self.add_triple(
                        node.clone(),
                        ShexRVocab::sx_min(),
                        Object::literal(ConcreteLiteral::integer(i128::from(*min))),
                    )?;
                }
                if let Some(max) = max {
                    self.add_triple(
                        node.clone(),
                        ShexRVocab::sx_max(),
                        Object::literal(ConcreteLiteral::integer(i128::from(*max))),
                    )?;
                }
                if let Some(sem_acts) = sem_acts {
                    let list = self.list_to_rdf(sem_acts, Self::sem_act_to_rdf)?;
                    self.add_triple(node.clone(), ShexRVocab::sx_sem_acts(), list)?;
                }
                if let Some(annotations) = annotations {
                    let list = self.list_to_rdf(annotations, Self::annotation_to_rdf)?;
                    self.add_triple(node.clone(), ShexRVocab::sx_annotation_prop(), list)?;
                }
                Ok(node.into())
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn triple_expr_group_to_rdf(
        &mut self,
        ty: IriS,
        id: &Option<TripleExprLabel>,
        expressions: &[TripleExprWrapper],
        min: Option<i32>,
        max: Option<i32>,
        sem_acts: &Option<Vec<SemAct>>,
        annotations: &Option<Vec<Annotation>>,
    ) -> Result<RDF::Term, ShExRBuilderError> {
        let node = match id {
            Some(label) => self.triple_expr_label_to_subject(label)?,
            None => self.bnode()?,
        };
        self.add_type(node.clone(), ty)?;
        let list = self.list_to_rdf(expressions, |ctx, w| ctx.triple_expr_to_rdf(&w.te))?;
        self.add_triple(node.clone(), ShexRVocab::sx_expressions(), list)?;
        if let Some(min) = min {
            self.add_triple(
                node.clone(),
                ShexRVocab::sx_min(),
                Object::literal(ConcreteLiteral::integer(i128::from(min))),
            )?;
        }
        if let Some(max) = max {
            self.add_triple(
                node.clone(),
                ShexRVocab::sx_max(),
                Object::literal(ConcreteLiteral::integer(i128::from(max))),
            )?;
        }
        if let Some(sem_acts) = sem_acts {
            let list = self.list_to_rdf(sem_acts, Self::sem_act_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_sem_acts(), list)?;
        }
        if let Some(annotations) = annotations {
            let list = self.list_to_rdf(annotations, Self::annotation_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_annotation_prop(), list)?;
        }
        Ok(node.into())
    }

    fn node_constraint_to_rdf(&mut self, nc: &NodeConstraint) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_node_constraint())?;
        if let Some(nk) = nc.node_kind() {
            let kind_iri = match nk {
                NodeKind::Iri => ShexRVocab::sx_iri(),
                NodeKind::BNode => ShexRVocab::sx_bnode(),
                NodeKind::NonLiteral => ShexRVocab::sx_non_literal(),
                NodeKind::Literal => ShexRVocab::sx_literal(),
            };
            self.add_triple(node.clone(), ShexRVocab::sx_node_kind(), kind_iri)?;
        }
        if let Some(dt) = nc.datatype() {
            let iri = self.schema.resolve_iriref(&dt);
            self.add_triple(node.clone(), ShexRVocab::sx_datatype(), iri)?;
        }
        if let Some(facets) = nc.xs_facet() {
            for facet in &facets {
                self.facet_to_rdf(&node, facet)?;
            }
        }
        if let Some(values) = nc.values() {
            let list = self.list_to_rdf(&values, Self::value_set_value_to_term)?;
            self.add_triple(node.clone(), ShexRVocab::sx_values(), list)?;
        }
        Ok(node.into())
    }

    fn facet_to_rdf(&mut self, node: &RDF::Subject, facet: &XsFacet) -> Result<(), ShExRBuilderError> {
        match facet {
            XsFacet::StringFacet(StringFacet::Length(n)) => self.add_triple(
                node.clone(),
                ShexRVocab::sx_length(),
                Object::literal(ConcreteLiteral::integer(*n as i128)),
            ),
            XsFacet::StringFacet(StringFacet::MinLength(n)) => self.add_triple(
                node.clone(),
                ShexRVocab::sx_minlength(),
                Object::literal(ConcreteLiteral::integer(*n as i128)),
            ),
            XsFacet::StringFacet(StringFacet::MaxLength(n)) => self.add_triple(
                node.clone(),
                ShexRVocab::sx_maxlength(),
                Object::literal(ConcreteLiteral::integer(*n as i128)),
            ),
            XsFacet::StringFacet(StringFacet::Pattern(p)) => {
                self.add_triple(node.clone(), ShexRVocab::sx_pattern(), Object::str(&p.str))?;
                if let Some(flags) = &p.flags {
                    self.add_triple(node.clone(), ShexRVocab::sx_flags(), Object::str(flags))?;
                }
                Ok(())
            },
            XsFacet::NumericFacet(NumericFacet::TotalDigits(n)) => self.add_triple(
                node.clone(),
                ShexRVocab::sx_totaldigits(),
                Object::literal(ConcreteLiteral::integer(*n as i128)),
            ),
            XsFacet::NumericFacet(NumericFacet::FractionDigits(n)) => self.add_triple(
                node.clone(),
                ShexRVocab::sx_fractiondigits(),
                Object::literal(ConcreteLiteral::integer(*n as i128)),
            ),
            XsFacet::NumericFacet(nf) => {
                let (pred, nl) = match nf {
                    NumericFacet::MinInclusive(nl) => (ShexRVocab::sx_mininclusive(), nl),
                    NumericFacet::MinExclusive(nl) => (ShexRVocab::sx_minexclusive(), nl),
                    NumericFacet::MaxInclusive(nl) => (ShexRVocab::sx_maxinclusive(), nl),
                    NumericFacet::MaxExclusive(nl) => (ShexRVocab::sx_maxexclusive(), nl),
                    NumericFacet::TotalDigits(_) | NumericFacet::FractionDigits(_) => unreachable!(),
                };
                let lit = ConcreteLiteral::lit_datatype(
                    &nl.lexical_form(),
                    &IriRef::iri(IriS::new_unchecked(facet_bound_datatype(nl))),
                );
                self.add_triple(node.clone(), pred, Object::literal(lit))
            },
        }
    }

    fn value_set_value_to_term(&mut self, v: &ValueSetValue) -> Result<RDF::Term, ShExRBuilderError> {
        match v {
            ValueSetValue::ObjectValue(ov) => Ok(self.object_value_to_term(ov)),
            ValueSetValue::IriStem { stem } => {
                let s = self.schema.resolve_iriref(stem).to_string();
                self.typed_string_node(ShexRVocab::sx_iri_stem(), ShexRVocab::sx_stem(), &s)
            },
            ValueSetValue::IriStemRange { stem, exclusions } => self.iri_stem_range_to_rdf(stem, exclusions),
            ValueSetValue::LiteralStem { stem } => {
                self.typed_string_node(ShexRVocab::sx_literal_stem(), ShexRVocab::sx_stem(), stem)
            },
            ValueSetValue::LiteralStemRange { stem, exclusions } => self.literal_stem_range_to_rdf(stem, exclusions),
            ValueSetValue::Language { language_tag } => self.typed_string_node(
                ShexRVocab::sx_language(),
                ShexRVocab::sx_language_tag(),
                language_tag.as_str(),
            ),
            ValueSetValue::LanguageStem { stem } => {
                let s = match stem {
                    LangOrWildcard::Lang(lang) => lang.as_str().to_string(),
                    LangOrWildcard::Wildcard => String::new(),
                };
                self.typed_string_node(ShexRVocab::sx_language_stem(), ShexRVocab::sx_stem(), &s)
            },
            ValueSetValue::LanguageStemRange { stem, exclusions } => self.language_stem_range_to_rdf(stem, exclusions),
        }
    }

    fn object_value_to_term(&self, ov: &ObjectValue) -> RDF::Term {
        match ov {
            ObjectValue::IriRef(iri_ref) => self.schema.resolve_iriref(iri_ref).into(),
            ObjectValue::Literal(lit) => Object::literal(lit.clone()).into(),
        }
    }

    /// A fresh blank node typed `ty`, with a single string-literal `pred`
    /// triple — the shape shared by `IriStem`/`LiteralStem`/`LanguageStem`/
    /// `Language`.
    fn typed_string_node(&mut self, ty: IriS, pred: IriS, value: &str) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ty)?;
        self.add_triple(node.clone(), pred, Object::str(value))?;
        Ok(node.into())
    }

    fn wildcard_node(&mut self) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_wildcard())?;
        Ok(node.into())
    }

    fn iri_stem_range_to_rdf(
        &mut self,
        stem: &IriRefOrWildcard,
        exclusions: &Option<Vec<IriExclusion>>,
    ) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_iri_stem_range())?;
        if let Some(excls) = exclusions {
            let list = self.list_to_rdf(excls, Self::iri_exclusion_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_exclusion(), list)?;
        }
        let stem_term = match stem {
            IriRefOrWildcard::IriRef(iri_ref) => {
                let s = self.schema.resolve_iriref(iri_ref).to_string();
                Object::str(&s).into()
            },
            IriRefOrWildcard::Wildcard => self.wildcard_node()?,
        };
        self.add_triple(node.clone(), ShexRVocab::sx_stem(), stem_term)?;
        Ok(node.into())
    }

    fn iri_exclusion_to_rdf(&mut self, excl: &IriExclusion) -> Result<RDF::Term, ShExRBuilderError> {
        match excl {
            IriExclusion::Iri(iri_ref) => Ok(self.schema.resolve_iriref(iri_ref).into()),
            IriExclusion::IriStem(iri_ref) => {
                let s = self.schema.resolve_iriref(iri_ref).to_string();
                self.typed_string_node(ShexRVocab::sx_iri_stem(), ShexRVocab::sx_stem(), &s)
            },
        }
    }

    fn literal_stem_range_to_rdf(
        &mut self,
        stem: &StringOrWildcard,
        exclusions: &Option<Vec<LiteralExclusion>>,
    ) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_literal_stem_range())?;
        if let Some(excls) = exclusions {
            let list = self.list_to_rdf(excls, Self::literal_exclusion_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_exclusion(), list)?;
        }
        let stem_term = match stem {
            StringOrWildcard::String(s) => Object::str(s).into(),
            StringOrWildcard::Wildcard => self.wildcard_node()?,
        };
        self.add_triple(node.clone(), ShexRVocab::sx_stem(), stem_term)?;
        Ok(node.into())
    }

    fn literal_exclusion_to_rdf(&mut self, excl: &LiteralExclusion) -> Result<RDF::Term, ShExRBuilderError> {
        match excl {
            LiteralExclusion::Literal(s) => Ok(Object::str(s).into()),
            LiteralExclusion::LiteralStem(s) => {
                self.typed_string_node(ShexRVocab::sx_literal_stem(), ShexRVocab::sx_stem(), s)
            },
        }
    }

    fn language_stem_range_to_rdf(
        &mut self,
        stem: &LangOrWildcard,
        exclusions: &Option<Vec<LanguageExclusion>>,
    ) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_language_stem_range())?;
        if let Some(excls) = exclusions {
            let list = self.list_to_rdf(excls, Self::language_exclusion_to_rdf)?;
            self.add_triple(node.clone(), ShexRVocab::sx_exclusion(), list)?;
        }
        let stem_term = match stem {
            LangOrWildcard::Lang(lang) => Object::str(lang.as_str()).into(),
            LangOrWildcard::Wildcard => self.wildcard_node()?,
        };
        self.add_triple(node.clone(), ShexRVocab::sx_stem(), stem_term)?;
        Ok(node.into())
    }

    fn language_exclusion_to_rdf(&mut self, excl: &LanguageExclusion) -> Result<RDF::Term, ShExRBuilderError> {
        match excl {
            LanguageExclusion::Language(lang) => Ok(Object::str(lang.as_str()).into()),
            LanguageExclusion::LanguageStem(lang) => {
                self.typed_string_node(ShexRVocab::sx_language_stem(), ShexRVocab::sx_stem(), lang.as_str())
            },
        }
    }

    fn sem_act_to_rdf(&mut self, sem_act: &SemAct) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_sem_act())?;
        let name_iri = self.schema.resolve_iriref(&sem_act.name());
        self.add_triple(node.clone(), ShexRVocab::sx_name(), name_iri)?;
        if let Some(code) = sem_act.code() {
            self.add_triple(node.clone(), ShexRVocab::sx_code(), Object::str(&code))?;
        }
        Ok(node.into())
    }

    fn annotation_to_rdf(&mut self, annotation: &Annotation) -> Result<RDF::Term, ShExRBuilderError> {
        let node = self.bnode()?;
        self.add_type(node.clone(), ShexRVocab::sx_annotation())?;
        let predicate_iri = self.schema.resolve_iriref(&annotation.predicate());
        self.add_triple(node.clone(), ShexRVocab::sx_predicate(), predicate_iri)?;
        let object = self.object_value_to_term(&annotation.object());
        self.add_triple(node.clone(), ShexRVocab::sx_object(), object)?;
        Ok(node.into())
    }

    /// `imports` entries are kept exactly as written (a plain relative
    /// reference like `"1dot"` stays relative, e.g. `<1dot>`) rather than
    /// resolved against the schema's base IRI — matching the ShExR fixtures,
    /// which don't resolve them either.
    fn import_to_rdf(&mut self, import: &crate::IriOrStr) -> Result<RDF::Term, ShExRBuilderError> {
        let iri = match import {
            crate::IriOrStr::String(s) => IriS::new_unchecked(s),
            crate::IriOrStr::IriRef(iri_ref) => self.schema.resolve_iriref(iri_ref),
        };
        Ok(iri.into())
    }

    fn label_to_subject_term(&mut self, label: &ShapeExprLabel) -> Result<RDF::Term, ShExRBuilderError> {
        self.label_to_subject(label).map(Into::into)
    }

    fn label_to_subject(&mut self, label: &ShapeExprLabel) -> Result<RDF::Subject, ShExRBuilderError> {
        match label {
            ShapeExprLabel::IriRef { value } => Ok(self.schema.resolve_iriref(value).into()),
            ShapeExprLabel::BNode { value } => self.bnode_for_label(value.value()),
            ShapeExprLabel::Start => Err(ShExRBuilderError::Unsupported {
                what: "shape label: Start".to_string(),
            }),
        }
    }

    fn triple_expr_label_to_subject(&mut self, label: &TripleExprLabel) -> Result<RDF::Subject, ShExRBuilderError> {
        match label {
            TripleExprLabel::IriRef { value } => Ok(self.schema.resolve_iriref(value).into()),
            TripleExprLabel::BNode { value } => self.bnode_for_label(value.value()),
        }
    }

    /// Builds an RDF collection (`rdf:first`/`rdf:rest`/`rdf:nil`) out of
    /// `items`, converting each element with `elem`, and returns the term
    /// identifying the head of the list (`rdf:nil` itself if `items` is
    /// empty). Mirrors `shacl::validator::report::result::path_list_to_rdf`.
    fn list_to_rdf<T>(
        &mut self,
        items: &[T],
        elem: impl Fn(&mut Self, &T) -> Result<RDF::Term, ShExRBuilderError>,
    ) -> Result<RDF::Term, ShExRBuilderError> {
        let mut rest: RDF::Term = RdfVocab::rdf_nil().into();
        for item in items.iter().rev() {
            let element = elem(self, item)?;
            let node = self.bnode()?;
            self.add_triple(node.clone(), RdfVocab::rdf_first(), element)?;
            self.add_triple(node.clone(), RdfVocab::rdf_rest(), rest)?;
            rest = node.into();
        }
        Ok(rest)
    }
}

/// XSD datatype for a numeric facet bound (`sx:mininclusive` etc). ShExJ's
/// deserializer stores an integral JSON number in whichever `NumericLiteral`
/// variant its magnitude fits (`Byte`, `Short`, ...) purely as a storage
/// optimization, but ShExR always encodes it as plain `xsd:integer`
/// regardless of that internal width; only `Decimal`/`Double`/`Float` values
/// (i.e. the JSON literal had a decimal point or exponent) keep their own
/// datatype.
fn facet_bound_datatype(nl: &rudof_rdf::rdf_core::term::literal::NumericLiteral) -> &'static str {
    use rudof_rdf::rdf_core::term::literal::NumericLiteral;
    match nl {
        NumericLiteral::Decimal(_) => "http://www.w3.org/2001/XMLSchema#decimal",
        NumericLiteral::Double(_) => "http://www.w3.org/2001/XMLSchema#double",
        NumericLiteral::Float(_) => "http://www.w3.org/2001/XMLSchema#float",
        _ => "http://www.w3.org/2001/XMLSchema#integer",
    }
}

fn error_mapper<RDF: Rdf>(msg: &str) -> impl FnOnce(RDF::Err) -> ShExRBuilderError + '_ {
    move |e| ShExRBuilderError::RDFBuildError {
        msg: format!("{msg}: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_iri::iri;
    use rudof_rdf::rdf_core::NeighsRDF;
    use rudof_rdf::rdf_core::term::Triple as _;
    use rudof_rdf::rdf_impl::OxigraphInMemory;

    fn schema_with_shape(label_iri: &str, shape_expr: ShapeExpr) -> Schema {
        let label = ShapeExprLabel::iri_unchecked(label_iri);
        let mut schema = Schema::new(&iri!("http://default/"));
        schema.add_shape(label, shape_expr, false);
        schema
    }

    #[test]
    fn builds_an_empty_shape_declaration() {
        let schema = schema_with_shape("http://a.example/S1", ShapeExpr::empty_shape());
        let mut graph = OxigraphInMemory::empty();
        ShExRBuilder::schema_to_rdf(&schema, &mut graph).unwrap();

        let triples: Vec<_> = graph.triples().unwrap().collect();
        assert_eq!(triples.len(), 7, "unexpected triples: {triples:#?}");
        let is_type = |t: &&<OxigraphInMemory as Rdf>::Triple, class: &str| {
            t.pred().as_str() == RdfVocab::RDF_TYPE && t.obj().to_string().contains(class)
        };
        assert!(triples.iter().any(|t| is_type(&t, ShexRVocab::SX_SCHEMA)));
        assert!(triples.iter().any(|t| is_type(&t, ShexRVocab::SX_SHAPE_DECL)));
        assert!(triples.iter().any(|t| is_type(&t, ShexRVocab::SX_SHAPE)));
        assert!(triples.iter().any(|t| t.pred().as_str() == ShexRVocab::SX_SHAPES));
        assert!(
            triples
                .iter()
                .any(|t| is_type(&t, ShexRVocab::SX_SHAPE_DECL) && t.subj().to_string().contains("http://a.example/S1"))
        );
        assert!(triples.iter().any(|t| t.pred().as_str() == ShexRVocab::SX_SHAPE_EXPR));
    }

    #[test]
    fn builds_a_single_triple_constraint() {
        let te = TripleExpr::triple_constraint(
            None,
            None,
            IriRef::iri(rudof_iri::IriS::new_unchecked("http://a.example/p1")),
            None,
            None,
            None,
        );
        let shape = ShapeExpr::shape(Shape::new(None, None, Some(te)));
        let schema = schema_with_shape("http://a.example/S1", shape);
        let mut graph = OxigraphInMemory::empty();
        ShExRBuilder::schema_to_rdf(&schema, &mut graph).unwrap();

        let triples: Vec<_> = graph.triples().unwrap().collect();
        assert!(triples.iter().any(|t| t.pred().as_str() == RdfVocab::RDF_TYPE
            && t.obj().to_string().contains(ShexRVocab::SX_TRIPLE_CONSTRAINT)));
        assert!(
            triples.iter().any(|t| t.pred().as_str() == ShexRVocab::SX_PREDICATE
                && t.obj().to_string().contains("http://a.example/p1"))
        );
        assert!(triples.iter().any(|t| t.pred().as_str() == ShexRVocab::SX_EXPRESSION));
    }

    #[test]
    fn unsupported_shape_expr_reports_unsupported_not_a_panic() {
        // `ShapeExprLabel::Start` (the special "START" pseudo-label, distinct
        // from `Schema::start()`) has no ShExR representation and is the one
        // remaining unsupported construct.
        let mut schema = Schema::new(&iri!("http://default/"));
        schema.add_shape(ShapeExprLabel::Start, ShapeExpr::empty_shape(), false);
        let mut graph = OxigraphInMemory::empty();
        let err = ShExRBuilder::schema_to_rdf(&schema, &mut graph).unwrap_err();
        assert!(matches!(err, ShExRBuilderError::Unsupported { .. }));
    }

    #[test]
    fn reuses_the_same_bnode_for_a_repeated_blank_shape_label() {
        // A shape declared under a blank-node label, referenced from another
        // shape's triple constraint, must resolve to the *same* RDF blank
        // node both times (ShExR identity, not two unrelated blank nodes).
        use crate::BNode;
        let mut schema = Schema::new(&iri!("http://default/"));
        schema.add_shape(ShapeExprLabel::bnode(BNode::new("b1")), ShapeExpr::empty_shape(), false);
        let te = TripleExpr::triple_constraint(
            None,
            None,
            IriRef::iri(rudof_iri::IriS::new_unchecked("http://a.example/p1")),
            Some(ShapeExpr::shape_ref(ShapeExprLabel::bnode(BNode::new("b1")))),
            None,
            None,
        );
        schema.add_shape(
            ShapeExprLabel::iri_unchecked("http://a.example/S1"),
            ShapeExpr::shape(Shape::new(None, None, Some(te))),
            false,
        );
        let mut graph = OxigraphInMemory::empty();
        ShExRBuilder::schema_to_rdf(&schema, &mut graph).unwrap();

        let triples: Vec<_> = graph.triples().unwrap().collect();
        let value_expr_target = triples
            .iter()
            .find(|t| t.pred().as_str() == ShexRVocab::SX_VALUE_EXPR)
            .map(|t| t.obj().clone())
            .expect("expected a sx:valueExpr triple");
        assert!(
            triples.iter().any(|t| t.pred().as_str() == RdfVocab::RDF_TYPE
                && t.obj().to_string().contains(ShexRVocab::SX_SHAPE)
                && t.subj().to_string() == value_expr_target.to_string()),
            "the referenced blank shape label should resolve to the same bnode as the declared shape"
        );
    }
}
