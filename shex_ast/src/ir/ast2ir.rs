use super::node_constraint::NodeConstraint;
use crate::ir::annotation::Annotation;
use crate::ir::object_value::ObjectValue;
use crate::ir::schema_ir::SchemaIR;
use crate::ir::sem_act::SemAct;
use crate::ir::shape::Shape;
use crate::ir::shape_expr::ShapeExpr;
use crate::ir::shape_label::ShapeLabel;
use crate::ir::value_set::ValueSet;
use crate::ir::value_set_value::ValueSetValue;
use crate::{CResult, Cond, Node, Pred, ResolveMethod, ShExFormat, ShExParser, ir};
use crate::{SchemaIRError, ShapeLabelIdx, ast, ast::Schema as SchemaAST};
use crate::{ShapeExprLabel, ast::iri_exclusion::IriExclusion};
use core::panic;
use iri_s::IriS;
use iri_s::error::IriSError;
use prefixmap::{IriRef, PrefixMap};
use rbe::{Cardinality, Pending, RbeError, SingleCond};
use rbe::{Component, MatchCond, Max, Min, RbeTable, rbe::Rbe};
use rdf::rdf_core::term::{Object, literal::{ConcreteLiteral, NumericLiteral}};
use tracing::{debug, trace};

#[derive(Debug, Default)]
/// AST2IR compile a Schema in AST (JSON) to IR (Intermediate Representation).
pub struct AST2IR {
    shape_decls_counter: usize,
    resolve_method: ResolveMethod,
}

impl AST2IR {
    pub fn new(resolve_method: &ResolveMethod) -> Self {
        Self {
            resolve_method: resolve_method.clone(),
            shape_decls_counter: 0,
        }
    }

    pub fn compile(
        &mut self,
        schema_ast: &SchemaAST,
        source_iri: &IriS,
        base: &Option<IriS>,
        compiled_schema: &mut SchemaIR,
    ) -> CResult<()> {
        let mut visited = Vec::new();
        trace!(
            "Compiling schema from {source_iri}. Base: {}",
            base.as_ref().map(|b| b.as_str()).unwrap_or("None")
        );
        let (local_shapes, _total_shapes) =
            self.compile_visited(schema_ast, source_iri, compiled_schema, &mut visited, base)?;
        compiled_schema.set_local_shapes_counter(local_shapes);
        compiled_schema.set_imported_schemas(visited);
        compiled_schema.increment_total_shapes(local_shapes);
        compiled_schema.build_dependency_graph();
        compiled_schema.build_inheritance_graph();
        Ok(())
    }

    // Returns the number of local shapes compiled
    fn compile_visited(
        &mut self,
        schema_ast: &SchemaAST,
        source_iri: &IriS,
        compiled_schema: &mut SchemaIR,
        visited_sources: &mut Vec<IriS>,
        base: &Option<IriS>,
    ) -> CResult<(usize, usize)> {
        let mut total_imported = 0;
        let imports = schema_ast
            .imports_resolved(base)
            .map_err(|e| Box::new(SchemaIRError::ImportIriError { error: e.to_string() }))?;
        for import_iri in imports {
            // trace!("Resolving import {import:?} with base: {base:?}");
            // let import_iri = cnv_iri_ref(&import, &compiled_schema.prefixmap())?;
            trace!("Import IRI resolved to {import_iri}");
            if !visited_sources.contains(&import_iri) {
                visited_sources.push(import_iri.clone());
                // For imported schemas, the base is the source IRI of the schema that imports them
                let imported_schema = self.resolve(&import_iri, Some(source_iri))?;
                let (_local, total) =
                    self.compile_visited(&imported_schema, &import_iri, compiled_schema, visited_sources, base)?;
                compiled_schema.increment_total_shapes(total);
                total_imported += total;
            }
        }
        trace!("Compiling schema to IR");
        compiled_schema.set_prefixmap(schema_ast.prefixmap());
        trace!("Collecting shape labels...");
        let local = self.collect_shape_labels(schema_ast, compiled_schema, source_iri)?;
        trace!("Collecting shape expressions...");
        self.collect_shape_exprs(schema_ast, compiled_schema, source_iri)?;
        trace!(
            "Schema compilation completed with {} shapes. {local}/{total_imported}",
            compiled_schema.shapes_counter()
        );
        Ok((local, total_imported + local))
    }

    fn resolve(&self, iri: &IriS, base: Option<&IriS>) -> CResult<SchemaAST> {
        let new_schema = match &self.resolve_method {
            ResolveMethod::RotatingFormats(formats) => find_schema_rotating_formats(iri, formats.clone(), base),
            ResolveMethod::ByGuessingExtension => todo!(),
            ResolveMethod::ByContentNegotiation => todo!(),
        }?;
        Ok(new_schema)
    }

    pub fn compile_import(&self, import: &IriRef, compiled_schema: &mut SchemaIR) -> CResult<()> {
        let iri = cnv_iri_ref(import, &compiled_schema.prefixmap())?;
        debug!("Importing schema from {iri}");
        // TODO
        Ok(())
    }

    pub fn collect_shape_labels(
        &mut self,
        schema_ast: &SchemaAST,
        compiled_schema: &mut SchemaIR,
        source_iri: &IriS,
    ) -> CResult<usize> {
        let mut shape_labels_counter = 0;
        match &schema_ast.shapes() {
            None => {},
            Some(shape_decls) => {
                for shape_decl in shape_decls {
                    let label = self.shape_expr_label_to_shape_label(&shape_decl.id)?;
                    let idx = compiled_schema.add_shape(label, ShapeExpr::Empty, source_iri);
                    if shape_decl.is_abstract {
                        compiled_schema.add_abstract_shape(idx);
                    }
                    self.shape_decls_counter += 1;
                    shape_labels_counter += 1;
                }
            },
        }
        if let Some(shape_expr_start) = &schema_ast.start() {
            let start_label = ShapeLabel::Start;
            let idx = compiled_schema.add_shape(start_label.clone(), ShapeExpr::Empty, source_iri);
            let start_compiled = self.compile_shape_expr(shape_expr_start, &idx, compiled_schema, source_iri)?;
            compiled_schema.replace_shape(&idx, start_compiled);
            self.shape_decls_counter += 1;
            shape_labels_counter += 1;
        }
        Ok(shape_labels_counter)
    }

    pub fn collect_shape_exprs(
        &mut self,
        schema_ast: &SchemaAST,
        compiled_schema: &mut SchemaIR,
        source_iri: &IriS,
    ) -> CResult<()> {
        match &schema_ast.shapes() {
            None => Ok(()),
            Some(sds) => {
                for sd in sds {
                    let idx = self.get_shape_label_idx(&sd.id, compiled_schema)?;
                    let se = self.compile_shape_decl(sd, &idx, compiled_schema, source_iri)?;
                    compiled_schema.replace_shape(&idx, se)
                }
                Ok(())
            },
        }
    }

    fn shape_expr_label_to_shape_label(&self, id: &ShapeExprLabel) -> CResult<ShapeLabel> {
        match id {
            ShapeExprLabel::IriRef { value } => {
                let shape_label = iri_ref_2_shape_label(value)?;
                Ok(shape_label)
            },
            ShapeExprLabel::BNode { value } => Ok(ShapeLabel::BNode(value.clone())),
            ShapeExprLabel::Start => Ok(ShapeLabel::Start),
        }
    }

    fn get_shape_label_idx(&self, id: &ShapeExprLabel, compiled_schema: &mut SchemaIR) -> CResult<ShapeLabelIdx> {
        let label = self.shape_expr_label_to_shape_label(id)?;
        compiled_schema.get_shape_label_idx(&label)
    }

    fn compile_shape_decl(
        &self,
        sd: &ast::ShapeDecl,
        idx: &ShapeLabelIdx,
        compiled_schema: &mut SchemaIR,
        source_iri: &IriS,
    ) -> CResult<ShapeExpr> {
        let se = self.compile_shape_expr(&sd.shape_expr, idx, compiled_schema, source_iri)?;
        Ok(se)
    }

    fn ref2idx(&self, sref: &ast::ShapeExprLabel, compiled_schema: &mut SchemaIR) -> CResult<ShapeLabelIdx> {
        let idx = self.get_shape_label_idx(sref, compiled_schema)?;
        Ok(idx)
    }

    fn compile_shape_expr(
        &self,
        se: &ast::ShapeExpr,
        idx: &ShapeLabelIdx,
        compiled_schema: &mut SchemaIR,
        source_iri: &IriS,
    ) -> CResult<ShapeExpr> {
        let result: ShapeExpr = match se {
            ast::ShapeExpr::Ref(se_ref) => {
                let new_idx = self.ref2idx(se_ref, compiled_schema)?;
                let se: ShapeExpr = ShapeExpr::Ref { idx: new_idx };
                Ok::<ShapeExpr, SchemaIRError>(se)
            },
            ast::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                tracing::debug!("Compiling ShapeOr with {ses:?}");
                let mut cnv = Vec::new();
                for sew in ses {
                    let internal_idx = compiled_schema.new_index(source_iri);
                    let se = self.compile_shape_expr(&sew.se, &internal_idx, compiled_schema, source_iri)?;
                    compiled_schema.replace_shape(&internal_idx, se.clone());
                    cnv.push(internal_idx);
                }
                let result = ShapeExpr::ShapeOr { exprs: cnv };
                compiled_schema.replace_shape(idx, result.clone());
                tracing::debug!("ShapeOr result: {result:?}");
                Ok(result)
            },
            ast::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                tracing::debug!("Compiling ShapeAnd with {ses:?}");
                let mut cnv = Vec::new();
                for sew in ses {
                    let internal_idx = compiled_schema.new_index(source_iri);
                    let se = self.compile_shape_expr(&sew.se, &internal_idx, compiled_schema, source_iri)?;
                    compiled_schema.replace_shape(&internal_idx, se.clone());
                    cnv.push(internal_idx);
                }
                let result = ShapeExpr::ShapeAnd { exprs: cnv };
                compiled_schema.replace_shape(idx, result.clone());
                tracing::debug!("ShapeAnd result: {result:?}");
                Ok(result)
            },
            ast::ShapeExpr::ShapeNot { shape_expr: sew } => {
                trace!("Compiling ShapeNot with {sew:?} and index {idx}");
                let internal_idx = compiled_schema.new_index(source_iri);
                let se = self.compile_shape_expr(&sew.se, &internal_idx, compiled_schema, source_iri)?;
                compiled_schema.replace_shape(&internal_idx, se.clone());
                let not_se = ShapeExpr::ShapeNot { expr: internal_idx };
                compiled_schema.replace_shape(idx, not_se.clone());
                Ok(not_se)
            },
            ast::ShapeExpr::Shape(shape) => {
                let new_extra = self.cnv_extra(&shape.extra, &compiled_schema.prefixmap())?;
                let rbe_table = match &shape.expression {
                    None => RbeTable::new(),
                    Some(tew) => {
                        let mut table = RbeTable::new();
                        let rbe = self.triple_expr2rbe(&tew.te, compiled_schema, &mut table, source_iri)?;
                        table.with_rbe(rbe);
                        table
                    },
                };
                let preds = Self::get_preds_shape(shape);
                // let references = self.get_references_shape(shape, compiled_schema);
                let extends = shape
                    .extends()
                    .iter()
                    .map(|s| self.ref2idx(s, compiled_schema))
                    .collect::<CResult<Vec<_>>>()?;

                let shape = Shape::new(
                    Self::cnv_closed(&shape.closed),
                    new_extra,
                    rbe_table,
                    Self::cnv_sem_acts(&shape.sem_acts),
                    Self::cnv_annotations(&shape.annotations),
                    preds,
                    extends,
                    // references,
                );
                Ok(ShapeExpr::Shape(Box::new(shape)))
            },
            ast::ShapeExpr::NodeConstraint(nc) => {
                let (cond, display) = Self::cnv_node_constraint(
                    self,
                    &nc.node_kind(),
                    &nc.datatype(),
                    &nc.xs_facet(),
                    &nc.values(),
                    &compiled_schema.prefixmap(),
                )?;
                let node_constraint = NodeConstraint::new(nc.clone(), cond, display);
                Ok(ShapeExpr::NodeConstraint(node_constraint))
            },
            ast::ShapeExpr::External => Ok(ShapeExpr::External {}),
        }?;
        //compiled_schema.replace_shape(idx, result.clone());
        trace!("Result of compilation: {result}");
        Ok(result)
    }

    fn cnv_node_constraint(
        &self,
        nk: &Option<ast::NodeKind>,
        dt: &Option<IriRef>,
        xs_facet: &Option<Vec<ast::XsFacet>>,
        values: &Option<Vec<ast::ValueSetValue>>,
        prefixmap: &PrefixMap,
    ) -> CResult<(Cond, String)> {
        let maybe_value_set = match values {
            Some(vs) => {
                let value_set = create_value_set(vs, prefixmap)?;
                Some(value_set)
            },
            None => None,
        };
        node_constraint2match_cond(nk, dt, xs_facet, &maybe_value_set, prefixmap)
    }

    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed,
        }
    }

    fn cnv_extra(&self, extra: &Option<Vec<IriRef>>, prefixmap: &PrefixMap) -> CResult<Vec<Pred>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = cnv_iri_ref(iri, prefixmap)?;
                vs.push(Pred::new(nm));
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_sem_acts(sem_acts: &Option<Vec<ast::SemAct>>) -> Vec<SemAct> {
        if let Some(_vs) = sem_acts {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn cnv_annotations(annotations: &Option<Vec<ast::Annotation>>) -> Vec<Annotation> {
        if let Some(_anns) = annotations {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn triple_expr2rbe(
        &self,
        triple_expr: &ast::TripleExpr,
        compiled_schema: &mut SchemaIR,
        current_table: &mut RbeTable<Pred, Node, ShapeLabelIdx>,
        source_iri: &IriS,
    ) -> CResult<Rbe<Component>> {
        match triple_expr {
            ast::TripleExpr::EachOf {
                id: _,
                expressions,
                min,
                max,
                sem_acts: _,
                annotations: _,
            } => {
                let mut cs = Vec::new();
                for e in expressions {
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table, source_iri)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::and(cs), card))
            },
            ast::TripleExpr::OneOf {
                id: _,
                expressions,
                min,
                max,
                sem_acts: _,
                annotations: _,
            } => {
                let mut cs = Vec::new();
                for e in expressions {
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table, source_iri)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::or(cs), card))
            },
            ast::TripleExpr::TripleConstraint {
                id: _,
                negated: _,
                inverse: _,
                predicate,
                value_expr,
                min,
                max,
                sem_acts: _,
                annotations: _,
            } => {
                let min = self.cnv_min(min)?;
                let max = self.cnv_max(max)?;
                let iri = Self::cnv_predicate(predicate)?;
                let (cond, _display) = self.value_expr2match_cond(value_expr, compiled_schema, source_iri)?;
                let c = current_table.add_component(iri, &cond);
                trace!("triple_expr2rbe: TripleConstraint: added component {c:?} to RBE table");
                Ok(Rbe::symbol(c, min.value, max))
            },
            ast::TripleExpr::Ref(r) => Err(Box::new(SchemaIRError::Todo {
                msg: format!("TripleExprRef {r:?}"),
            })),
        }
    }

    fn cnv_predicate(predicate: &IriRef) -> CResult<Pred> {
        match predicate {
            IriRef::Iri(iri) => Ok(Pred::from(iri.clone())),
            IriRef::Prefixed { prefix, local } => Err(Box::new(SchemaIRError::Internal {
                msg: format!("Cannot convert prefixed {prefix}:{local} to predicate without context"),
            })),
        }
    }

    fn cnv_min_max(&self, min: &Option<i32>, max: &Option<i32>) -> CResult<Cardinality> {
        let min = self.cnv_min(min)?;
        let max = self.cnv_max(max)?;
        Ok(Cardinality::from(min, max))
    }

    fn mk_card_group(rbe: Rbe<Component>, card: Cardinality) -> Rbe<Component> {
        match &card {
            c if c.is_1_1() => rbe,
            c if c.is_star() => Rbe::Star { value: Box::new(rbe) },
            c if c.is_plus() => Rbe::Plus { value: Box::new(rbe) },
            _c => Rbe::Repeat {
                value: Box::new(rbe),
                card,
            },
        }
    }

    fn cnv_min(&self, min: &Option<i32>) -> CResult<Min> {
        match min {
            Some(min) if *min < 0 => Err(Box::new(SchemaIRError::MinLessZero { min: *min })),
            Some(min) => Ok(Min::from(*min)),
            None => Ok(Min::from(1)),
        }
    }

    fn cnv_max(&self, max: &Option<i32>) -> CResult<Max> {
        match *max {
            Some(-1) => Ok(Max::Unbounded),
            Some(max) if max < -1 => Err(Box::new(SchemaIRError::MaxIncorrect { max })),
            Some(max) => Ok(Max::from(max)),
            None => Ok(Max::from(1)),
        }
    }

    fn value_expr2match_cond(
        &self,
        ve: &Option<Box<ast::ShapeExpr>>,
        compiled_schema: &mut SchemaIR,
        source_iri: &IriS,
    ) -> CResult<(Cond, String)> {
        if let Some(se) = ve.as_deref() {
            match se {
                ast::ShapeExpr::NodeConstraint(nc) => self.cnv_node_constraint(
                    &nc.node_kind(),
                    &nc.datatype(),
                    &nc.xs_facet(),
                    &nc.values(),
                    &compiled_schema.prefixmap(),
                ),

                ast::ShapeExpr::Ref(sref) => {
                    let idx = self.ref2idx(sref, compiled_schema)?;
                    Ok((mk_cond_ref(idx), format!("ShapeRef {sref}")))
                },
                ast::ShapeExpr::Shape { .. } => {
                    // TODO: avoid recompiling the same shape expression?
                    // I think this code should be reviewed....
                    let idx = compiled_schema.new_index(source_iri);
                    let se = self.compile_shape_expr(se, &idx, compiled_schema, source_iri)?;
                    compiled_schema.replace_shape(&idx, se.clone());
                    trace!("Returning SHAPE cond with idx {idx}");
                    Ok((mk_cond_ref(idx), format!("Shape {idx}")))
                },
                ast::ShapeExpr::ShapeAnd { shape_exprs } => {
                    let mut ands = Vec::new();
                    for shape_expr in shape_exprs {
                        let idx_se = compiled_schema.new_index(source_iri);
                        let se = self.compile_shape_expr(&shape_expr.se, &idx_se, compiled_schema, source_iri)?;
                        compiled_schema.replace_shape(&idx_se, se.clone());
                        ands.push(idx_se);
                    }
                    let and_se = ShapeExpr::ShapeAnd { exprs: ands.clone() };
                    let idx_and = compiled_schema.new_index(source_iri);
                    compiled_schema.replace_shape(&idx_and, and_se);
                    trace!("Returning AND cond with idx {idx_and}");
                    let display = format!(
                        "AND({})",
                        ands.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
                    );
                    Ok((mk_cond_ref(idx_and), display))
                },
                ast::ShapeExpr::ShapeOr { shape_exprs } => {
                    let mut ors = Vec::new();
                    for se in shape_exprs {
                        let idx_se = compiled_schema.new_index(source_iri);
                        let se = self.compile_shape_expr(&se.se, &idx_se, compiled_schema, source_iri)?;
                        compiled_schema.replace_shape(&idx_se, se.clone());
                        ors.push(idx_se);
                    }
                    let or_se = ShapeExpr::ShapeOr { exprs: ors.clone() };
                    let display = format!(
                        "OR({})",
                        ors.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
                    );
                    let idx_or = compiled_schema.new_index(source_iri);
                    compiled_schema.replace_shape(&idx_or, or_se);
                    Ok((mk_cond_ref(idx_or), display))
                },
                ast::ShapeExpr::ShapeNot { shape_expr } => {
                    let idx_shape_expr = compiled_schema.new_index(source_iri);
                    trace!(
                        "value_expr2matchcond: Compiling ShapeNot with {shape_expr:?}, idx_shape_expr {idx_shape_expr}"
                    );
                    let se = self.compile_shape_expr(&shape_expr.se, &idx_shape_expr, compiled_schema, source_iri)?;
                    compiled_schema.replace_shape(&idx_shape_expr, se.clone());
                    let display = format!("NOT {idx_shape_expr}");
                    let not_se = ShapeExpr::ShapeNot { expr: idx_shape_expr };
                    let idx_not = compiled_schema.new_index(source_iri);
                    compiled_schema.replace_shape(&idx_not, not_se);
                    trace!("Returning NOT cond with idx {idx_not}");
                    Ok((mk_cond_ref(idx_not), display))
                },
                ast::ShapeExpr::External => todo("value_expr2match_cond: ShapeExternal"),
            }
        } else {
            Ok((MatchCond::single(SingleCond::new().with_name(".")), ".".to_string()))
        }
    }

    fn get_preds_shape(shape: &ast::Shape) -> Vec<Pred> {
        match shape.triple_expr() {
            None => Vec::new(),
            Some(te) => Self::get_preds_triple_expr(&te),
        }
    }

    fn get_preds_triple_expr(te: &ast::TripleExpr) -> Vec<Pred> {
        match te {
            ast::TripleExpr::EachOf { expressions, .. } => expressions
                .iter()
                .flat_map(|te| Self::get_preds_triple_expr(&te.te))
                .collect(),
            ast::TripleExpr::OneOf { expressions, .. } => expressions
                .iter()
                .flat_map(|te| Self::get_preds_triple_expr(&te.te))
                .collect(),
            ast::TripleExpr::TripleConstraint { predicate, .. } => {
                let pred = iri_ref2iri_s(predicate);
                vec![Pred::new(pred)]
            },
            ast::TripleExpr::Ref(_) => todo!(),
        }
    }
}

fn iri_ref2iri_s(iri_ref: &IriRef) -> IriS {
    match iri_ref {
        IriRef::Iri(iri) => iri.clone(),
        IriRef::Prefixed { prefix, local } => {
            panic!("Compiling schema...found prefixed iri: {prefix}:{local}")
        },
    }
}

fn node_constraint2match_cond(
    node_kind: &Option<ast::NodeKind>,
    datatype: &Option<IriRef>,
    xs_facet: &Option<Vec<ast::XsFacet>>,
    values: &Option<ValueSet>,
    prefixmap: &PrefixMap,
) -> CResult<(Cond, String)> {
    let c1: Option<(Cond, String)> = node_kind.as_ref().map(node_kind2match_cond);
    let c2 = datatype
        .as_ref()
        .map(|dt| datatype2match_cond(dt, prefixmap))
        .transpose()?;
    let c3 = xs_facet.as_ref().map(xs_facets2match_cond);
    let c4 = values.as_ref().map(|vs| valueset2match_cond(vs.clone(), prefixmap));
    let os = vec![c1, c2, c3, c4];
    Ok(options2match_cond(os))
}

fn node_kind2match_cond(nodekind: &ast::NodeKind) -> (Cond, String) {
    (mk_cond_nodekind(nodekind.clone()), format!("nodekind({nodekind})"))
}

fn datatype2match_cond(datatype: &IriRef, prefixmap: &PrefixMap) -> CResult<(Cond, String)> {
    let iri = cnv_iri_ref(datatype, prefixmap)?;
    let cond = mk_cond_datatype(&iri, prefixmap);
    let str = cond.to_string();
    Ok((cond, str))
}

fn xs_facets2match_cond(xs_facets: &Vec<ast::XsFacet>) -> (Cond, String) {
    let mut conds = Vec::new();
    for xs_facet in xs_facets {
        conds.push(xs_facet2match_cond(xs_facet))
    }
    (MatchCond::And(conds), format!("xs_facets({xs_facets:?})"))
}

fn xs_facet2match_cond(xs_facet: &ast::XsFacet) -> Cond {
    match xs_facet {
        ast::XsFacet::StringFacet(sf) => string_facet_to_match_cond(sf),
        ast::XsFacet::NumericFacet(nf) => numeric_facet_to_match_cond(nf),
    }
}

fn string_facet_to_match_cond(sf: &ast::StringFacet) -> Cond {
    match sf {
        ast::StringFacet::Length(len) => mk_cond_length(*len),
        ast::StringFacet::MinLength(len) => mk_cond_min_length(*len),
        ast::StringFacet::MaxLength(len) => mk_cond_max_length(*len),
        ast::StringFacet::Pattern(pat) => mk_cond_pattern(pat.regex(), pat.flags()),
    }
}

fn numeric_facet_to_match_cond(nf: &ast::NumericFacet) -> Cond {
    match nf {
        ast::NumericFacet::MinInclusive(min) => mk_cond_min_inclusive(min.clone()),
        ast::NumericFacet::MinExclusive(min) => mk_cond_min_exclusive(min.clone()),
        ast::NumericFacet::MaxInclusive(max) => mk_cond_max_inclusive(max.clone()),
        ast::NumericFacet::MaxExclusive(max) => mk_cond_max_exclusive(max.clone()),
        ast::NumericFacet::TotalDigits(td) => mk_cond_total_digits(*td),
        ast::NumericFacet::FractionDigits(fd) => mk_cond_fraction_digits(*fd),
    }
}

fn valueset2match_cond(vs: ValueSet, prefixmap: &PrefixMap) -> (Cond, String) {
    (
        mk_cond_value_set(vs.clone(), prefixmap),
        format!(
            "[{}]",
            vs.values()
                .iter()
                .map(|v| v.show_qualified(prefixmap))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
}

fn options2match_cond<T: IntoIterator<Item = Option<(Cond, String)>>>(os: T) -> (Cond, String) {
    let vec: Vec<(Cond, String)> = os.into_iter().flatten().collect();
    match &vec[..] {
        [] => (MatchCond::empty(), ".".to_string()),
        [(c, s)] => (c.clone(), s.clone()),
        _ => (
            MatchCond::And(vec.iter().map(|(c, _)| c.clone()).collect()),
            vec.iter().map(|(_, s)| s.clone()).collect::<Vec<_>>().join(" AND "),
        ),
    }
}

fn mk_cond_ref(idx: ShapeLabelIdx) -> Cond {
    MatchCond::ref_(idx)
}

fn mk_cond_datatype(datatype: &IriS, prefixmap: &PrefixMap) -> Cond {
    let dt = datatype.clone();
    MatchCond::single(
        SingleCond::new()
            .with_name(prefixmap.qualify(&dt).to_string().as_str())
            .with_cond(move |value: &Node| match check_node_datatype(value, &dt) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("Datatype error: {err}"),
                }),
            }),
    )
}

fn mk_cond_length(len: usize) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("length({len})").as_str())
            .with_cond(move |value: &Node| match check_node_length(value, len) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("Length error: {err}"),
                }),
            }),
    )
}

fn mk_cond_min_inclusive(min: NumericLiteral) -> Cond {
    let min_str = min.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("minInclusive({min_str})").as_str())
            .with_cond(move |value: &Node| match check_node_min_inclusive(value, min.clone()) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MinInclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_min_exclusive(min: NumericLiteral) -> Cond {
    let min_str = min.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("minExclusive({min_str})").as_str())
            .with_cond(move |value: &Node| match check_node_min_exclusive(value, min.clone()) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MinExclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_total_digits(total: usize) -> Cond {
    let total_str = total.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("totalDigits({total_str})").as_str())
            .with_cond(move |value: &Node| match check_node_total_digits(value, total) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MaxExclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_fraction_digits(total: usize) -> Cond {
    let total_str = total.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("fractionDigits({total_str})").as_str())
            .with_cond(move |value: &Node| match check_node_fraction_digits(value, total) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MaxExclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_max_exclusive(max: NumericLiteral) -> Cond {
    let max_str = max.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("maxExclusive({max_str})").as_str())
            .with_cond(move |value: &Node| match check_node_max_exclusive(value, max.clone()) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MaxExclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_max_inclusive(max: NumericLiteral) -> Cond {
    let max_str = max.to_string();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("maxInclusive({max_str})").as_str())
            .with_cond(move |value: &Node| match check_node_max_inclusive(value, max.clone()) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MaxInclusive: {err}"),
                }),
            }),
    )
}

fn mk_cond_min_length(len: usize) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("minLength({len})").as_str())
            .with_cond(move |value: &Node| match check_node_min_length(value, len) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("MinLength error: {err}"),
                }),
            }),
    )
}

fn mk_cond_max_length(len: usize) -> Cond {
    MatchCond::simple(
        format!("maxLength({len})").as_str(),
        move |value: &Node| match check_node_max_length(value, len) {
            Ok(_) => Ok(Pending::new()),
            Err(err) => Err(RbeError::MsgError {
                msg: format!("MaxLength error: {err}"),
            }),
        },
    )
}

fn mk_cond_nodekind(nodekind: ast::NodeKind) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("nodekind({nodekind})").as_str())
            .with_cond(move |value: &Node| match check_node_node_kind(value, &nodekind) {
                Ok(_) => Ok(Pending::empty()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("NodeKind Error: {err}"),
                }),
            }),
    )
}

fn mk_cond_pattern(regex: &str, flags: Option<&str>) -> Cond {
    let regex_str = format!("/{regex}/{}", flags.unwrap_or(""));
    let regex = regex.to_string();
    let flags = flags.map(|f| f.to_string());
    MatchCond::single(SingleCond::new().with_name(regex_str.as_str()).with_cond(
        move |value: &Node| match check_pattern(value, &regex, flags.as_deref()) {
            Ok(_) => Ok(Pending::new()),
            Err(err) => Err(RbeError::MsgError {
                msg: format!("Pattern error: {err}"),
            }),
        },
    ))
}

fn iri_ref_2_shape_label(id: &IriRef) -> CResult<ShapeLabel> {
    match id {
        IriRef::Iri(iri) => Ok(ShapeLabel::Iri(iri.clone())),
        IriRef::Prefixed { prefix, local } => Err(Box::new(SchemaIRError::IriRef2ShapeLabelError {
            prefix: prefix.clone(),
            local: local.clone(),
        })),
    }
}

fn mk_cond_value_set(value_set: ValueSet, prefixmap: &PrefixMap) -> Cond {
    let cloned_prefixmap = prefixmap.clone();
    MatchCond::single(
        SingleCond::new()
            .with_name(value_set.show_qualified(prefixmap).as_str())
            .with_cond(move |node: &Node| {
                if value_set.check_value(node.as_object()) {
                    Ok(Pending::empty())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!(
                            "Values failed: {node} not in {}",
                            value_set.show_qualified(&cloned_prefixmap)
                        ),
                    })
                }
            }),
    )
}

fn create_value_set(values: &Vec<ast::ValueSetValue>, prefixmap: &PrefixMap) -> CResult<ValueSet> {
    let mut vs = ValueSet::new();
    for v in values {
        let cvalue = cnv_value(v, prefixmap)?;
        vs.add_value(cvalue)
    }
    Ok(vs)
}

fn cnv_value(v: &ast::ValueSetValue, prefixmap: &PrefixMap) -> CResult<ValueSetValue> {
    match &v {
        ast::ValueSetValue::IriStem { stem, .. } => {
            let cnv_stem = cnv_iri_ref(stem, prefixmap)?;
            Ok(ValueSetValue::IriStem { stem: cnv_stem })
        },
        ast::ValueSetValue::ObjectValue(ovw) => {
            let ov = cnv_object_value(ovw, prefixmap)?;
            Ok(ValueSetValue::ObjectValue(ov))
        },
        ast::ValueSetValue::Language { language_tag, .. } => Ok(ValueSetValue::Language {
            language_tag: language_tag.clone(),
        }),
        ast::ValueSetValue::LiteralStem { stem, .. } => Ok(ValueSetValue::LiteralStem { stem: stem.to_string() }),
        ast::ValueSetValue::LiteralStemRange { stem, exclusions } => {
            let stem = cnv_string_or_wildcard(stem)?;
            let exclusions = cnv_literal_exclusions(exclusions)?;
            Ok(ValueSetValue::LiteralStemRange { stem, exclusions })
        },
        ast::ValueSetValue::IriStemRange { stem, exclusions } => {
            let stem = cnv_iriref_or_wildcard(stem, prefixmap)?;
            let exclusions = cnv_iri_exclusions(exclusions)?;
            Ok(ValueSetValue::IriStemRange { stem, exclusions })
        },
        ast::ValueSetValue::LanguageStem { stem } => {
            let stem = cnv_lang_or_wildcard(stem)?;
            Ok(ValueSetValue::LanguageStem { stem })
        },
        ast::ValueSetValue::LanguageStemRange { stem, exclusions } => {
            let stem = cnv_lang_or_wildcard(stem)?;
            let exclusions = cnv_lang_exclusions(exclusions)?;
            Ok(ValueSetValue::LanguageStemRange { stem, exclusions })
        },
    }
}

fn cnv_lang_or_wildcard(stem: &ast::LangOrWildcard) -> CResult<crate::ir::value_set_value::LangOrWildcard> {
    match stem {
        ast::LangOrWildcard::Lang(s) => Ok(crate::ir::value_set_value::LangOrWildcard::Lang(s.clone())),
        ast::LangOrWildcard::Wildcard => Ok(crate::ir::value_set_value::LangOrWildcard::Wildcard {
            type_: "Lang wildcard".to_string(),
        }),
    }
}

fn cnv_string_or_wildcard(stem: &ast::StringOrWildcard) -> CResult<crate::ir::value_set_value::StringOrWildcard> {
    match stem {
        ast::StringOrWildcard::String(s) => Ok(crate::ir::value_set_value::StringOrWildcard::String(s.to_string())),
        ast::StringOrWildcard::Wildcard => {
            Ok(crate::ir::value_set_value::StringOrWildcard::Wildcard { type_: "".to_string() })
        },
    }
}

fn cnv_iriref_or_wildcard(
    stem: &ast::IriRefOrWildcard,
    prefixmap: &PrefixMap,
) -> CResult<crate::ir::value_set_value::IriOrWildcard> {
    match stem {
        ast::IriRefOrWildcard::IriRef(iri) => {
            let cnv_iri = cnv_iri_ref(iri, prefixmap)?;
            Ok(crate::ir::value_set_value::IriOrWildcard::Iri(cnv_iri))
        },
        ast::IriRefOrWildcard::Wildcard => {
            Ok(crate::ir::value_set_value::IriOrWildcard::Wildcard { type_: "".to_string() })
        },
    }
}

/*fn cnv_exclusions(
    exclusions: &Option<Vec<ast::StringOrLiteralStemWrapper>>,
) -> CResult<Option<Vec<crate::ir::value_set_value::StringOrLiteralStem>>> {
    match exclusions {
        None => Ok(None),
        Some(exs) => {
            let mut rs = Vec::new();
            for ex in exs {
                let cnv_ex = cnv_string_or_literal_stem(ex)?;
                rs.push(cnv_ex);
            }
            Ok(Some(rs))
        }
    }
}*/

fn cnv_literal_exclusions(
    exclusions: &Option<Vec<ast::literal_exclusion::LiteralExclusion>>,
) -> CResult<Option<Vec<ir::exclusion::LiteralExclusion>>> {
    match exclusions {
        None => Ok(None),
        Some(exs) => {
            let mut rs = Vec::new();
            for ex in exs {
                let cnv_ex = cnv_literal_exclusion(ex)?;
                rs.push(cnv_ex);
            }
            Ok(Some(rs))
        },
    }
}

fn cnv_iri_exclusions(
    exclusions: &Option<Vec<ast::iri_exclusion::IriExclusion>>,
) -> CResult<Option<Vec<ir::exclusion::IriExclusion>>> {
    match exclusions {
        None => Ok(None),
        Some(exs) => {
            let mut rs = Vec::new();
            for ex in exs {
                let cnv_ex = cnv_iri_exclusion(ex)?;
                rs.push(cnv_ex);
            }
            Ok(Some(rs))
        },
    }
}

fn cnv_lang_exclusions(
    exclusions: &Option<Vec<ast::language_exclusion::LanguageExclusion>>,
) -> CResult<Option<Vec<ir::exclusion::LanguageExclusion>>> {
    match exclusions {
        None => Ok(None),
        Some(exs) => {
            let mut rs = Vec::new();
            for ex in exs {
                let cnv_ex = cnv_language_exclusion(ex)?;
                rs.push(cnv_ex);
            }
            Ok(Some(rs))
        },
    }
}

/*
fn cnv_string_or_literal_exclusions(
    exclusions: &Option<Vec<ast::StringOrLiteralExclusion>>,
) -> CResult<Option<Vec<crate::ir::value_set_value::StringOrLiteralExclusion>>> {
    match exclusions {
        None => Ok(None),
        Some(exs) => {
            let mut rs = Vec::new();
            for ex in exs {
                let cnv_ex = cnv_string_or_literal_exclusion(ex)?;
                rs.push(cnv_ex);
            }
            Ok(Some(rs))
        }
    }
}*/

/*
fn cnv_string_or_literalstem(sl: &ast::StringOrLiteralStemWrapper) -> CResult<StringOrLiteralStem> {
    match sl.inner() {
        ast::StringOrLiteralStem::String(s) => Ok(StringOrLiteralStem::String(s.to_string())),
        ast::StringOrLiteralStem::LiteralStem { stem } => Ok(StringOrLiteralStem::LiteralStem {
            stem: stem.to_string(),
        }),
    }
}*/

fn cnv_literal_exclusion(
    le: &ast::literal_exclusion::LiteralExclusion,
) -> CResult<crate::ir::exclusion::LiteralExclusion> {
    match le {
        ast::literal_exclusion::LiteralExclusion::Literal(s) => {
            Ok(crate::ir::exclusion::LiteralExclusion::Literal(s.to_string()))
        },
        ast::literal_exclusion::LiteralExclusion::LiteralStem(s) => {
            Ok(crate::ir::exclusion::LiteralExclusion::LiteralStem(s.to_string()))
        },
    }
}

fn cnv_iri_exclusion(le: &IriExclusion) -> CResult<crate::ir::exclusion::IriExclusion> {
    match le {
        ast::iri_exclusion::IriExclusion::Iri(s) => {
            let iri_s = iri_ref2iri_s(s);
            Ok(crate::ir::exclusion::IriExclusion::Iri(iri_s))
        },
        ast::iri_exclusion::IriExclusion::IriStem(s) => {
            let iri_s = iri_ref2iri_s(s);
            Ok(crate::ir::exclusion::IriExclusion::IriStem(iri_s))
        },
    }
}

fn cnv_language_exclusion(
    le: &ast::language_exclusion::LanguageExclusion,
) -> CResult<crate::ir::exclusion::LanguageExclusion> {
    match le {
        ast::language_exclusion::LanguageExclusion::Language(s) => {
            Ok(crate::ir::exclusion::LanguageExclusion::Language(s.clone()))
        },
        ast::language_exclusion::LanguageExclusion::LanguageStem(s) => {
            Ok(crate::ir::exclusion::LanguageExclusion::LanguageStem(s.clone()))
        },
    }
}

fn cnv_object_value(ov: &ast::ObjectValue, prefixmap: &PrefixMap) -> CResult<ObjectValue> {
    match ov {
        ast::ObjectValue::IriRef(ir) => {
            let iri = cnv_iri_ref(ir, prefixmap)?;
            Ok(ObjectValue::IriRef(iri))
        },
        ast::ObjectValue::Literal(lit) => Ok(ObjectValue::ObjectLiteral(lit.clone())),
    }
}

fn check_pattern(node: &Node, regex: &str, flags: Option<&str>) -> CResult<()> {
    let lexical_form = match node.as_object() {
        Object::Literal(lit) => Ok(lit.lexical_form()),
        Object::BlankNode(b) => Ok(b.clone()),
        _ => Err(Box::new(SchemaIRError::PatternNodeNotLiteral {
            node: node.to_string(),
            regex: regex.to_string(),
            flags: flags.map(|f| f.to_string()),
        })),
    }?;
    if let Ok(re) = regex::Regex::new(regex) {
        if re.is_match(&lexical_form) {
            Ok(())
        } else {
            Err(Box::new(SchemaIRError::PatternError {
                regex: regex.to_string(),
                flags: flags.unwrap_or("").to_string(),
                lexical_form: lexical_form.clone(),
            }))
        }
    } else {
        Err(Box::new(SchemaIRError::InvalidRegex {
            regex: regex.to_string(),
        }))
    }
}

fn check_node_node_kind(node: &Node, nk: &ast::NodeKind) -> CResult<()> {
    match (nk, node.as_object()) {
        (ast::NodeKind::Iri, Object::Iri { .. }) => Ok(()),
        (ast::NodeKind::Iri, _) => Err(Box::new(SchemaIRError::NodeKindIri { node: node.clone() })),
        (ast::NodeKind::BNode, Object::BlankNode(_)) => Ok(()),
        (ast::NodeKind::BNode, _) => Err(Box::new(SchemaIRError::NodeKindBNode { node: node.clone() })),
        (ast::NodeKind::Literal, Object::Literal(_)) => Ok(()),
        (ast::NodeKind::Literal, _) => Err(Box::new(SchemaIRError::NodeKindLiteral { node: node.clone() })),
        (ast::NodeKind::NonLiteral, Object::BlankNode(_)) => Ok(()),
        (ast::NodeKind::NonLiteral, Object::Iri { .. }) => Ok(()),
        (ast::NodeKind::NonLiteral, _) => Err(Box::new(SchemaIRError::NodeKindNonLiteral { node: node.clone() })),
    }
}

fn check_node_datatype(node: &Node, dt: &IriS) -> CResult<()> {
    let object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_datatype: as_checked_object error: {e}"),
        })
    })?;
    trace!("check_node_datatype: {object:?} datatype: {dt}");
    match object {
        Object::Literal(sliteral) => check_literal_datatype(&sliteral, dt, node),
        Object::Iri(_) | Object::BlankNode(_) | Object::Triple { .. } => {
            Err(Box::new(SchemaIRError::DatatypeNoLiteral {
                expected: Box::new(dt.clone()),
                node: Box::new(node.clone()),
            }))
        },
    }
}

// Check that the literal has the expected datatype
// It assumes that the literal has been checked and in case of wrong datatype it is a WrongDatatypeLiteral
fn check_literal_datatype(sliteral: &ConcreteLiteral, expected: &IriS, node: &Node) -> CResult<()> {
    let checked_literal = sliteral.clone().as_checked_literal().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_literal_datatype: as_checked_literal error: {e}"),
        })
    })?;
    match checked_literal {
        ConcreteLiteral::WrongDatatypeLiteral {
            lexical_form,
            datatype,
            error,
        } => Err(Box::new(SchemaIRError::WrongDatatypeLiteralMatch {
            datatype: datatype.clone(),
            error: error.clone(),
            expected: expected.clone(),
            lexical_form: lexical_form.to_string(),
        })),
        _ => {
            let node_dt = checked_literal.datatype();
            let node_dt_iri = node_dt.get_iri().map_err(|e| {
                Box::new(SchemaIRError::CheckLiteralDatatypeCnvIriRef2IriError {
                    iri_ref: node_dt.clone(),
                    error: e.to_string(),
                })
            })?;
            if node_dt_iri == expected {
                Ok(())
            } else {
                Err(Box::new(SchemaIRError::DatatypeDontMatch {
                    expected: expected.clone(),
                    found: node_dt,
                    lexical_form: node.to_string(),
                }))
            }
        },
    }
}

fn check_node_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_length: {node:?} length: {len}");
    let node_length = node.length();
    if node_length == len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::LengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}

fn check_node_min_inclusive(node: &Node, min: NumericLiteral) -> CResult<()> {
    trace!("check_node_min_inclusive: {node:?} min_inclusive: {min}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_min_inclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_inclusive: as_numeric error".to_string(),
        })
    })?;
    if !node_num.less_than(&min) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinInclusiveError {
            expected: min.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_min_exclusive(node: &Node, min: NumericLiteral) -> CResult<()> {
    trace!("check_node_min_exclusive: {node:?} min_exclusive: {min}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_min_exclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_exclusive: as_numeric error".to_string(),
        })
    })?;
    if !node_num.less_than_or_eq(&min) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinExclusiveError {
            expected: min.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_total_digits(node: &Node, total: usize) -> CResult<()> {
    trace!("check_node_total_digits: {node:?} total: {total}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_total_digits: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_total_digits: as_numeric error".to_string(),
        })
    })?;
    if let Some(num_digits) = node_num.total_digits() {
        trace!("check_node_total_digits: node total digits: {num_digits}");
        if num_digits <= total {
            trace!("check_node_total_digits: OK {num_digits} <= {total} node [{node_num}]");
            Ok(())
        } else {
            trace!("check_node_total_digits: Failed {num_digits} > {total} node [{node_num}]");
            Err(Box::new(SchemaIRError::TotalDigitsError {
                expected: total,
                found: node_num,
                node: node.to_string(),
            }))
        }
    } else {
        trace!("check_node_total_digits: node has no total digits");
        Err(Box::new(SchemaIRError::TotalDigitsError {
            expected: total,
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_fraction_digits(node: &Node, fd: usize) -> CResult<()> {
    trace!("check_node_fraction_digits: {node:?} total: {fd}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_fraction_digits: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_fraction_digits: as_numeric error".to_string(),
        })
    })?;
    if let Some(num_fd) = node_num.fraction_digits() {
        trace!("check_node_fraction_digits: node fraction digits: {num_fd}");
        if num_fd <= fd {
            trace!("check_node_fraction_digits: OK {fd:?} > Fraction digits of {node_num:?} = {num_fd}",);
            Ok(())
        } else {
            trace!("check_node_fraction_digits: Failed {fd} <= fraction digits of {node_num} {num_fd}",);
            Err(Box::new(SchemaIRError::FractionDigitsError {
                expected: fd,
                found: node_num,
                node: node.to_string(),
            }))
        }
    } else {
        trace!("check_node_fraction_digits: node has no fraction digits");
        Err(Box::new(SchemaIRError::FractionDigitsError {
            expected: fd,
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_max_exclusive(node: &Node, max: NumericLiteral) -> CResult<()> {
    trace!("check_node_max_exclusive: {node:?} max_exclusive: {max:?}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_max_exclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_min_exclusive: as_numeric error".to_string(),
        })
    })?;
    if node_num.less_than(&max) {
        trace!("check_node_max_exclusive: OK {node_num:?} < {max:?}");
        Ok(())
    } else {
        trace!("check_node_max_exclusive: Failed {node_num} not less than {max}");
        Err(Box::new(SchemaIRError::MinExclusiveError {
            expected: max.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_max_inclusive(node: &Node, max: NumericLiteral) -> CResult<()> {
    trace!("check_node_max_inclusive: {node:?} max_inclusive: {max}");
    let node_object = node.as_checked_object().map_err(|e| {
        Box::new(SchemaIRError::Internal {
            msg: format!("check_node_max_inclusive: as_checked_object error: {e}"),
        })
    })?;
    let node_num = node_object.numeric_value().ok_or_else(|| {
        Box::new(SchemaIRError::Internal {
            msg: "check_node_max_inclusive: as_numeric error".to_string(),
        })
    })?;
    if node_num.less_than_or_eq(&max) {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MaxInclusiveError {
            expected: max.clone(),
            found: node_num,
            node: node.to_string(),
        }))
    }
}

fn check_node_min_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_min_length: {node:?} min_length: {len}");
    let node_length = node.length();
    if node_length >= len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MinLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}

fn check_node_max_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_max_length: {node:?} max_length: {len}");
    let node_length = node.length();
    if node_length <= len {
        Ok(())
    } else {
        Err(Box::new(SchemaIRError::MaxLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        }))
    }
}

fn todo<A>(str: &str) -> CResult<A> {
    panic!("TODO: {str}");
    /*Err(Box::new(SchemaIRError::Todo {
        msg: str.to_string(),
    }))*/
}

fn cnv_iri_ref(iri: &IriRef, prefixmap: &PrefixMap) -> Result<IriS, Box<SchemaIRError>> {
    match iri {
        IriRef::Iri(iri) => Ok(iri.clone()),
        IriRef::Prefixed { prefix, local } => prefixmap.resolve_prefix_local(prefix, local).map_err(|e| {
            Box::new(SchemaIRError::CnvIriRefError {
                prefix: prefix.clone(),
                local: local.clone(),
                error: e.to_string(),
            })
        }),
    }
}

pub fn find_schema_rotating_formats(
    iri: &IriS,
    formats: Vec<ShExFormat>,
    base: Option<&IriS>,
) -> Result<SchemaAST, Box<SchemaIRError>> {
    let mut errors = Vec::new();
    for format in &formats {
        trace!("Trying format {format} for IRI {iri} with base {base:?}");
        match get_schema_from_iri(iri, format, base) {
            Err(e) => {
                errors.push((format.clone(), e));
            },
            Ok(schema) => return Ok(schema),
        }
    }
    Err(Box::new(SchemaIRError::SchemaFromIriRotatingFormats {
        iri: iri.clone(),
        errors,
    }))
}

pub fn get_schema_from_iri(
    iri: &IriS,
    format: &ShExFormat,
    base: Option<&IriS>,
) -> Result<SchemaAST, Box<SchemaIRError>> {
    let candidates = candidates(iri, base, format).map_err(|e| {
        Box::new(SchemaIRError::CandidatesError {
            iri: iri.clone(),
            error: e.to_string(),
        })
    })?;
    match format {
        ShExFormat::ShExC => {
            let content = find_content_from_iris(candidates, base)?;
            let schema = ShExParser::parse(content.as_str(), None, iri).map_err(|e| {
                Box::new(SchemaIRError::ShExCError {
                    iri: iri.clone(),
                    error: e.to_string(),
                })
            })?;
            Ok(schema)
        },
        ShExFormat::ShExJ => {
            let content = find_content_from_iris(candidates, base)?;
            let schema = SchemaAST::from_reader(content.as_bytes()).map_err(|e| {
                Box::new(SchemaIRError::ShExJError {
                    iri: iri.clone(),
                    error: e.to_string(),
                })
            })?;
            Ok(schema)
        },
        ShExFormat::RDFFormat(_) => {
            todo!()
        },
    }
}

pub fn candidates(iri: &IriS, base: Option<&IriS>, format: &ShExFormat) -> Result<Vec<IriS>, IriSError> {
    let mut candidates = vec![iri.clone()];
    let extended_iris: Result<Vec<IriS>, IriSError> = format
        .extensions()
        .into_iter()
        .map(|ext| {
            let ext_iri = format!("{}.{ext}", iri.as_str());
            IriS::from_str_base_iri(&ext_iri, base)
        })
        .collect();
    let extended = extended_iris?;
    candidates.extend(extended);
    Ok(candidates)
}

fn find_content_from_iris(iris: Vec<IriS>, base: Option<&IriS>) -> Result<String, Box<SchemaIRError>> {
    find_first_ok(iris, |iri| get_content(iri, base))
        .map_err(|errs| Box::new(SchemaIRError::FindingContentFromIrisError { errors: errs }))
        .map(|(content, _)| content)
}

fn get_content(iri: IriS, base: Option<&IriS>) -> Result<String, SchemaIRError> {
    iri.dereference(base).map_err(|e| SchemaIRError::DereferencingIri {
        iri: iri.clone(),
        error: e.to_string(),
    })
}

/// Applies function `f` to each element of `vec` in order, returning the first `Ok` result
/// along with a vector of all errors encountered before that point.
/// If all applications of `f` result in `Err`, returns a vector of all errors.
fn find_first_ok<A, B, E, F>(vec: Vec<A>, f: F) -> Result<(B, Vec<E>), Vec<E>>
where
    F: Fn(A) -> Result<B, E>,
{
    let mut errors = Vec::new();

    for item in vec {
        match f(item) {
            Ok(b) => return Ok((b, errors)),
            Err(e) => errors.push(e),
        }
    }
    // If we get here, all items resulted in errors
    Err(errors)
}
