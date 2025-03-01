use crate::compiled::annotation::Annotation;
use crate::compiled::compiled_schema::CompiledSchema;
use crate::compiled::object_value::ObjectValue;
use crate::compiled::sem_act::SemAct;
use crate::compiled::shape::Shape;
use crate::compiled::shape_expr::ShapeExpr;
use crate::compiled::shape_label::ShapeLabel;
use crate::compiled::value_set::ValueSet;
use crate::compiled::value_set_value::ValueSetValue;
use crate::ShapeExprLabel;
use crate::{ast, ast::Schema as SchemaJson, CompiledSchemaError, ShapeLabelIdx};
use crate::{CResult, Cond, Node, Pred};
use iri_s::IriS;
use lazy_static::lazy_static;
use prefixmap::IriRef;
use rbe::{rbe::Rbe, Component, MatchCond, Max, Min, RbeTable};
use rbe::{Cardinality, Pending, RbeError, SingleCond};
use srdf::literal::Literal;
use srdf::Object;
use tracing::debug;

use super::node_constraint::NodeConstraint;

lazy_static! {
    static ref XSD_STRING: IriRef = IriRef::Iri(IriS::new_unchecked(
        "http://www.w3.org/2001/XMLSchema#string"
    ));
    static ref RDF_LANG_STRING: IriRef = IriRef::Iri(IriS::new_unchecked(
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"
    ));
}

#[derive(Debug, Default)]
pub struct SchemaJsonCompiler {
    shape_decls_counter: usize,
}

impl SchemaJsonCompiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(
        &mut self,
        schema_json: &SchemaJson,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<()> {
        debug!("Compiling schema_json: {compiled_schema:?}");
        compiled_schema.set_prefixmap(schema_json.prefixmap());
        self.collect_shape_labels(schema_json, compiled_schema)?;
        self.collect_shape_exprs(schema_json, compiled_schema)?;
        Ok(())
    }

    pub fn collect_shape_labels(
        &mut self,
        schema_json: &SchemaJson,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<()> {
        match &schema_json.shapes() {
            None => Ok(()),
            Some(sds) => {
                for sd in sds {
                    let label = self.shape_expr_label_to_shape_label(&sd.id)?;
                    compiled_schema.add_shape(label, ShapeExpr::Empty);
                    self.shape_decls_counter += 1;
                }
                Ok(())
            }
        }
    }

    pub fn collect_shape_exprs(
        &mut self,
        schema_json: &SchemaJson,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<()> {
        match &schema_json.shapes() {
            None => Ok(()),
            Some(sds) => {
                for sd in sds {
                    let idx = self.get_shape_label_idx(&sd.id, compiled_schema)?;
                    let se = self.compile_shape_decl(sd, &idx, compiled_schema)?;
                    compiled_schema.replace_shape(&idx, se)
                }
                Ok(())
            }
        }
    }

    fn shape_expr_label_to_shape_label(&self, id: &ShapeExprLabel) -> CResult<ShapeLabel> {
        match id {
            ShapeExprLabel::IriRef { value } => {
                let shape_label = iri_ref_2_shape_label(value)?;
                Ok(shape_label)
            }
            ShapeExprLabel::BNode { value } => Ok(ShapeLabel::BNode(value.clone())),
            ShapeExprLabel::Start => Ok(ShapeLabel::Start),
        }
    }

    fn get_shape_label_idx(
        &self,
        id: &ShapeExprLabel,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<ShapeLabelIdx> {
        let label = self.shape_expr_label_to_shape_label(id)?;
        compiled_schema.get_shape_label_idx(&label)
    }

    fn compile_shape_decl(
        &self,
        sd: &ast::ShapeDecl,
        idx: &ShapeLabelIdx,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<ShapeExpr> {
        self.compile_shape_expr(&sd.shape_expr, idx, compiled_schema)
    }

    fn ref2idx(
        &self,
        sref: &ast::ShapeExprLabel,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<ShapeLabelIdx> {
        let idx = self.get_shape_label_idx(sref, compiled_schema)?;
        Ok(idx)
    }

    fn compile_shape_expr(
        &self,
        se: &ast::ShapeExpr,
        idx: &ShapeLabelIdx,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<ShapeExpr> {
        match se {
            ast::ShapeExpr::Ref(se_ref) => {
                let idx = self.ref2idx(se_ref, compiled_schema)?;
                Ok(ShapeExpr::Ref { idx })
            }
            ast::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                    cnv.push(se);
                }
                let display = match compiled_schema.find_shape_idx(idx) {
                    None => "internal OR".to_string(),
                    Some((label, _)) => compiled_schema.show_label(label),
                };

                Ok(ShapeExpr::ShapeOr {
                    exprs: cnv,
                    display,
                })
            }
            ast::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                    cnv.push(se);
                }
                let display = match compiled_schema.find_shape_idx(idx) {
                    None => "internal AND".to_string(),
                    Some((label, _)) => compiled_schema.show_label(label),
                };
                Ok(ShapeExpr::ShapeAnd {
                    exprs: cnv,
                    display,
                })
            }
            ast::ShapeExpr::ShapeNot { shape_expr: sew } => {
                let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                let display = match compiled_schema.find_shape_idx(idx) {
                    None => "internal NOT".to_string(),
                    Some((label, _)) => compiled_schema.show_label(label),
                };
                Ok(ShapeExpr::ShapeNot {
                    expr: Box::new(se),
                    display,
                })
            }
            ast::ShapeExpr::Shape(shape) => {
                let new_extra = self.cnv_extra(&shape.extra)?;
                let rbe_table = match &shape.expression {
                    None => RbeTable::new(),
                    Some(tew) => {
                        let mut table = RbeTable::new();
                        let rbe = self.triple_expr2rbe(&tew.te, compiled_schema, &mut table)?;
                        table.with_rbe(rbe);
                        table
                    }
                };
                let preds = Self::get_preds_shape(shape);

                let display = match compiled_schema.find_shape_idx(idx) {
                    None => "internal".to_string(),
                    Some((label, _)) => compiled_schema.show_label(label),
                };

                let shape = Shape::new(
                    Self::cnv_closed(&shape.closed),
                    new_extra,
                    rbe_table,
                    Self::cnv_sem_acts(&shape.sem_acts),
                    Self::cnv_annotations(&shape.annotations),
                    preds,
                    display,
                );
                Ok(ShapeExpr::Shape(shape))
            }
            ast::ShapeExpr::NodeConstraint(nc) => {
                // let node_kind_cnv: Option<NodeKind> = cnv_opt(&nc.node_kind(), cnv_node_kind)?;
                // let datatype_cnv = cnv_opt(&nc.datatype(), cnv_iri_ref)?;
                // let xs_facet_cnv = cnv_opt_vec(&nc.xs_facet(), cnv_xs_facet)?;
                // let values_cnv = cnv_opt_vec(&nc.values(), cnv_value)?;
                let cond = Self::cnv_node_constraint(
                    self,
                    &nc.node_kind(),
                    &nc.datatype(),
                    &nc.xs_facet(),
                    &nc.values(),
                )?;
                let display = match compiled_schema.find_shape_idx(idx) {
                    None => "internal NodeConstraint".to_string(),
                    Some((label, _)) => compiled_schema.show_label(label),
                };
                let node_constraint = NodeConstraint::new(nc.clone(), cond, display);
                Ok(ShapeExpr::NodeConstraint(node_constraint))
            }
            ast::ShapeExpr::External => Ok(ShapeExpr::External {}),
        }
    }

    fn cnv_node_constraint(
        &self,
        nk: &Option<ast::NodeKind>,
        dt: &Option<IriRef>,
        xs_facet: &Option<Vec<ast::XsFacet>>,
        values: &Option<Vec<ast::ValueSetValue>>,
    ) -> CResult<Cond> {
        let maybe_value_set = match values {
            Some(vs) => {
                let value_set = create_value_set(vs)?;
                Some(value_set)
            }
            None => None,
        };
        node_constraint2match_cond(nk, dt, xs_facet, &maybe_value_set)
    }

    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed,
        }
    }

    fn cnv_extra(&self, extra: &Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = cnv_iri_ref(iri)?;
                vs.push(nm);
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
        compiled_schema: &mut CompiledSchema,
        current_table: &mut RbeTable<Pred, Node, ShapeLabelIdx>,
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
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::and(cs), card))
            }
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
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::or(cs), card))
            }
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
                let cond = self.value_expr2match_cond(value_expr, compiled_schema)?;
                let c = current_table.add_component(iri, &cond);
                Ok(Rbe::symbol(c, min.value, max))
            }
            ast::TripleExpr::TripleExprRef(r) => Err(CompiledSchemaError::Todo {
                msg: format!("TripleExprRef {r:?}"),
            }),
        }
    }

    fn cnv_predicate(predicate: &IriRef) -> CResult<Pred> {
        match predicate {
            IriRef::Iri(iri) => Ok(Pred::from(iri.clone())),
            IriRef::Prefixed { prefix, local } => Err(CompiledSchemaError::Internal {
                msg: format!(
                    "Cannot convert prefixed {prefix}:{local} to predicate without context"
                ),
            }),
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
            c if c.is_star() => Rbe::Star {
                value: Box::new(rbe),
            },
            c if c.is_plus() => Rbe::Plus {
                value: Box::new(rbe),
            },
            _c => Rbe::Repeat {
                value: Box::new(rbe),
                card,
            },
        }
    }

    fn cnv_min(&self, min: &Option<i32>) -> CResult<Min> {
        match min {
            Some(min) if *min < 0 => Err(CompiledSchemaError::MinLessZero { min: *min }),
            Some(min) => Ok(Min::from(*min)),
            None => Ok(Min::from(1)),
        }
    }

    fn cnv_max(&self, max: &Option<i32>) -> CResult<Max> {
        match *max {
            Some(-1) => Ok(Max::Unbounded),
            Some(max) if max < -1 => Err(CompiledSchemaError::MaxIncorrect { max }),
            Some(max) => Ok(Max::from(max)),
            None => Ok(Max::from(1)),
        }
    }

    fn value_expr2match_cond(
        &self,
        ve: &Option<Box<ast::ShapeExpr>>,
        compiled_schema: &mut CompiledSchema,
    ) -> CResult<Cond> {
        if let Some(se) = ve.as_deref() {
            match se {
                ast::ShapeExpr::NodeConstraint(nc) => self.cnv_node_constraint(
                    &nc.node_kind(),
                    &nc.datatype(),
                    &nc.xs_facet(),
                    &nc.values(),
                ),

                ast::ShapeExpr::Ref(sref) => {
                    let idx = self.ref2idx(sref, compiled_schema)?;
                    Ok(mk_cond_ref(idx))
                }
                ast::ShapeExpr::Shape { .. } => todo("value_expr2match_cond: Shape"),
                ast::ShapeExpr::ShapeAnd { .. } => todo("value_expr2match_cond: ShapeOr"),
                ast::ShapeExpr::ShapeOr { .. } => todo("value_expr2match_cond: ShapeOr"),
                ast::ShapeExpr::ShapeNot { .. } => todo("value_expr2match_cond: ShapeNot"),
                ast::ShapeExpr::External => todo("value_expr2match_cond: ShapeExternal"),
            }
        } else {
            Ok(MatchCond::single(SingleCond::new().with_name(".")))
        }
    }

    /* #[allow(dead_code)]
    fn shape_expr2match_cond(
        &self,
        _se: schema_json::ShapeExpr,
        _compiled_schema: &mut CompiledSchema,
    ) -> CResult<Cond> {
        todo("shape_expr2match_cond")
    }*/

    fn get_preds_shape(shape: &ast::Shape) -> Vec<IriS> {
        match shape.triple_expr() {
            None => Vec::new(),
            Some(te) => Self::get_preds_triple_expr(&te),
        }
    }

    fn get_preds_triple_expr(te: &ast::TripleExpr) -> Vec<IriS> {
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
                vec![pred]
            }
            ast::TripleExpr::TripleExprRef(_) => todo!(),
        }
    }
}

fn iri_ref2iri_s(iri_ref: &IriRef) -> IriS {
    match iri_ref {
        IriRef::Iri(iri) => iri.clone(),
        IriRef::Prefixed { prefix, local } => {
            panic!("Compiling schema...found prefixed iri: {prefix}:{local}")
        }
    }
}

fn node_constraint2match_cond(
    node_kind: &Option<ast::NodeKind>,
    datatype: &Option<IriRef>,
    xs_facet: &Option<Vec<ast::XsFacet>>,
    values: &Option<ValueSet>,
) -> CResult<Cond> {
    let c1: Option<Cond> = node_kind.as_ref().map(node_kind2match_cond);
    let c2 = datatype.as_ref().map(datatype2match_cond).transpose()?;
    let c3 = xs_facet.as_ref().map(xs_facets2match_cond);
    let c4 = values.as_ref().map(|vs| valueset2match_cond(vs.clone()));
    let os = vec![c1, c2, c3, c4];
    Ok(options2match_cond(os))
}

fn node_kind2match_cond(nodekind: &ast::NodeKind) -> Cond {
    mk_cond_nodekind(nodekind.clone())
}

fn datatype2match_cond(datatype: &IriRef) -> CResult<Cond> {
    //let iri = cnv_iri_ref(datatype)?;
    Ok(mk_cond_datatype(datatype))
}

fn xs_facets2match_cond(xs_facets: &Vec<ast::XsFacet>) -> Cond {
    let mut conds = Vec::new();
    for xs_facet in xs_facets {
        conds.push(xs_facet2match_cond(xs_facet))
    }
    MatchCond::And(conds)
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
        ast::StringFacet::Pattern(_) => todo!(),
    }
}

fn numeric_facet_to_match_cond(nf: &ast::NumericFacet) -> Cond {
    match nf {
        ast::NumericFacet::MinInclusive(_min) => todo!(),
        ast::NumericFacet::MinExclusive(_) => todo!(),
        ast::NumericFacet::MaxInclusive(_) => todo!(),
        ast::NumericFacet::MaxExclusive(_) => todo!(),
        ast::NumericFacet::TotalDigits(_) => todo!(),
        ast::NumericFacet::FractionDigits(_) => todo!(),
    }
}

fn valueset2match_cond(vs: ValueSet) -> Cond {
    mk_cond_value_set(vs)
}

fn options2match_cond<T: IntoIterator<Item = Option<Cond>>>(os: T) -> Cond {
    let vec: Vec<Cond> = os.into_iter().flatten().collect();
    match &vec[..] {
        [] => MatchCond::empty(),
        [c] => c.clone(),
        _ => MatchCond::And(vec),
    }
}

fn mk_cond_ref(idx: ShapeLabelIdx) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("@{idx}").as_str())
            .with_cond(move |value: &Node| {
                let result = Pending::from_pair(value.clone(), idx);
                Ok(result)
            }),
    )
}

fn mk_cond_datatype(datatype: &IriRef) -> Cond {
    let dt = datatype.clone();
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("datatype{dt}").as_str())
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
            .with_name(format!("length{len}").as_str())
            .with_cond(move |value: &Node| match check_node_length(value, len) {
                Ok(_) => Ok(Pending::new()),
                Err(err) => Err(RbeError::MsgError {
                    msg: format!("Length error: {err}"),
                }),
            }),
    )
}

fn mk_cond_min_length(len: usize) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("minLength{len}").as_str())
            .with_cond(
                move |value: &Node| match check_node_min_length(value, len) {
                    Ok(_) => Ok(Pending::new()),
                    Err(err) => Err(RbeError::MsgError {
                        msg: format!("MinLength error: {err}"),
                    }),
                },
            ),
    )
}

fn mk_cond_max_length(len: usize) -> Cond {
    MatchCond::simple(format!("maxLength{len}").as_str(), move |value: &Node| {
        match check_node_max_length(value, len) {
            Ok(_) => Ok(Pending::new()),
            Err(err) => Err(RbeError::MsgError {
                msg: format!("MaxLength error: {err}"),
            }),
        }
    })
}

fn mk_cond_nodekind(nodekind: ast::NodeKind) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("nodekind{nodekind}").as_str())
            .with_cond(
                move |value: &Node| match check_node_node_kind(value, &nodekind) {
                    Ok(_) => Ok(Pending::empty()),
                    Err(err) => Err(RbeError::MsgError {
                        msg: format!("NodeKind Error: {err}"),
                    }),
                },
            ),
    )
}

fn iri_ref_2_shape_label(id: &IriRef) -> CResult<ShapeLabel> {
    match id {
        IriRef::Iri(iri) => Ok(ShapeLabel::Iri(iri.clone())),
        IriRef::Prefixed { prefix, local } => Err(CompiledSchemaError::IriRef2ShapeLabelError {
            prefix: prefix.clone(),
            local: local.clone(),
        }),
    }
}

fn mk_cond_value_set(value_set: ValueSet) -> Cond {
    MatchCond::single(
        SingleCond::new()
            .with_name(format!("{}", value_set).as_str())
            .with_cond(move |node: &Node| {
                if value_set.check_value(node.as_object()) {
                    Ok(Pending::empty())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Values failed: {node} not in {value_set}"),
                    })
                }
            }),
    )
}

fn create_value_set(values: &Vec<ast::ValueSetValue>) -> CResult<ValueSet> {
    let mut vs = ValueSet::new();
    for v in values {
        let cvalue = cnv_value(v)?;
        vs.add_value(cvalue)
    }
    Ok(vs)
}

fn cnv_value(v: &ast::ValueSetValue) -> CResult<ValueSetValue> {
    match &v {
        ast::ValueSetValue::IriStem { stem, .. } => {
            let cnv_stem = cnv_iri_ref(stem)?;
            Ok(ValueSetValue::IriStem { stem: cnv_stem })
        }
        ast::ValueSetValue::ObjectValue(ovw) => {
            let ov = cnv_object_value(ovw)?;
            Ok(ValueSetValue::ObjectValue(ov))
        }
        ast::ValueSetValue::Language { language_tag, .. } => Ok(ValueSetValue::Language {
            language_tag: language_tag.to_string(),
        }),
        ast::ValueSetValue::LiteralStem { stem, .. } => Ok(ValueSetValue::LiteralStem {
            stem: stem.to_string(),
        }),
        ast::ValueSetValue::LiteralStemRange { .. } => {
            todo!()
            /*let stem = cnv_string_or_wildcard(&stem)?;
            let exclusions = cnv_opt_vec(&exclusions, cnv_string_or_literalstem)?;
            Ok(ValueSetValue::LiteralStemRange { stem, exclusions })*/
        }
        _ => todo!(),
    }
}

/*
fn cnv_node_kind(_nk: &ast::NodeKind) -> CResult<NodeKind> {
    todo!()
}

fn cnv_xs_facet(_xsf: &ast::XsFacet) -> CResult<XsFacet> {
    todo!()
}

fn cnv_vec<A, B, F>(vs: Vec<A>, func: F) -> CResult<Vec<B>>
where
    F: Fn(&A) -> CResult<B>,
{
    let mut rs = Vec::new();
    for v in vs {
        let b = func(&v)?;
        rs.push(b);
    }
    Ok(rs)
}

fn cnv_opt_vec<A, B, F>(maybe_vs: &Option<Vec<A>>, func: F) -> CResult<Option<Vec<B>>>
where
    F: Fn(&A) -> CResult<B>,
{
    match maybe_vs {
        None => Ok(None),
        Some(vs) => {
            let mut rs = Vec::new();
            for v in vs {
                match func(v) {
                    Err(err) => return Err(err),
                    Ok(result) => {
                        rs.push(result);
                    }
                }
            }
            Ok(Some(rs))
        }
    }
}

fn cnv_opt<A, B, F>(maybe_vs: &Option<A>, func: F) -> CResult<Option<B>>
where
    F: Fn(&A) -> CResult<B>,
{
    match maybe_vs {
        None => Ok(None),
        Some(vs) => match func(vs) {
            Err(err) => Err(err),
            Ok(v) => Ok(Some(v)),
        },
    }
}

fn cnv_string_or_wildcard(_sw: &ast::StringOrWildcard) -> CResult<StringOrWildcard> {
    todo!()
}

fn cnv_string_or_literalstem(
    _sl: &ast::StringOrLiteralStemWrapper,
) -> CResult<StringOrLiteralStem> {
    todo!()
}
*/

fn cnv_object_value(ov: &ast::ObjectValue) -> CResult<ObjectValue> {
    match ov {
        ast::ObjectValue::IriRef(ir) => {
            let iri = cnv_iri_ref(ir)?;
            Ok(ObjectValue::IriRef(iri))
        }
        ast::ObjectValue::Literal(_n) => {
            todo!()
        } /*ast::ObjectValue::ObjectLiteral {
              value, language, ..
          } => Ok(ObjectValue::ObjectLiteral {
              value: value.to_string(),
              language: language.clone(),
          })*/
    }
}

/*fn cnv_lang(lang: &String) -> CResult<Lang> {
    Ok(Lang::new(lang.as_str()))
}*/

/*fn check_node_maybe_node_kind(node: &Node, nodekind: &Option<ast::NodeKind>) -> CResult<()> {
    match nodekind {
        None => Ok(()),
        Some(nk) => check_node_node_kind(node, &nk),
    }
}*/

fn check_node_node_kind(node: &Node, nk: &ast::NodeKind) -> CResult<()> {
    match (nk, node.as_object()) {
        (ast::NodeKind::Iri, Object::Iri { .. }) => Ok(()),
        (ast::NodeKind::Iri, _) => Err(CompiledSchemaError::NodeKindIri { node: node.clone() }),
        (ast::NodeKind::BNode, Object::BlankNode(_)) => Ok(()),
        (ast::NodeKind::BNode, _) => Err(CompiledSchemaError::NodeKindBNode { node: node.clone() }),
        (ast::NodeKind::Literal, Object::Literal(_)) => Ok(()),
        (ast::NodeKind::Literal, _) => {
            Err(CompiledSchemaError::NodeKindLiteral { node: node.clone() })
        }
        (ast::NodeKind::NonLiteral, Object::BlankNode(_)) => Ok(()),
        (ast::NodeKind::NonLiteral, Object::Iri { .. }) => Ok(()),
        (ast::NodeKind::NonLiteral, _) => {
            Err(CompiledSchemaError::NodeKindNonLiteral { node: node.clone() })
        }
    }
}

/*
fn check_node_maybe_datatype(node: &Node, datatype: &Option<IriRef>) -> CResult<()> {
    match datatype {
        None => Ok(()),
        Some(dt) => check_node_datatype(node, dt),
    }
}
*/

fn check_node_datatype(node: &Node, dt: &IriRef) -> CResult<()> {
    debug!("check_node_datatype: {node:?} datatype: {dt}");
    match node.as_object() {
        Object::Literal(Literal::DatatypeLiteral {
            ref datatype,
            lexical_form,
        }) => {
            if dt == datatype {
                Ok(())
            } else {
                Err(CompiledSchemaError::DatatypeDontMatch {
                    expected: dt.clone(),
                    found: datatype.clone(),
                    lexical_form: lexical_form.clone(),
                })
            }
        }
        Object::Literal(Literal::StringLiteral {
            lexical_form,
            lang: None,
        }) => {
            debug!("StringLiteral...{}", *dt);
            if *dt == *XSD_STRING {
                debug!("datatype cond passes");
                Ok(())
            } else {
                debug!("datatype cond fails: {}!={}", dt, *XSD_STRING);
                Err(CompiledSchemaError::DatatypeDontMatchString {
                    expected: dt.clone(),
                    lexical_form: lexical_form.clone(),
                })
            }
        }
        Object::Literal(Literal::StringLiteral {
            lexical_form,
            lang: Some(lang),
        }) => {
            if *dt == *RDF_LANG_STRING {
                Ok(())
            } else {
                Err(CompiledSchemaError::DatatypeDontMatchLangString {
                    lexical_form: lexical_form.clone(),
                    lang: Box::new(lang.clone()),
                })
            }
        }
        _ => Err(CompiledSchemaError::DatatypeNoLiteral {
            expected: Box::new(dt.clone()),
            node: Box::new(node.clone()),
        }),
    }
}

fn check_node_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_length: {node:?} length: {len}");
    let node_length = node.length();
    if node_length == len {
        Ok(())
    } else {
        Err(CompiledSchemaError::LengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        })
    }
}

fn check_node_min_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_min_length: {node:?} min_length: {len}");
    let node_length = node.length();
    if node_length >= len {
        Ok(())
    } else {
        Err(CompiledSchemaError::MinLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        })
    }
}

fn check_node_max_length(node: &Node, len: usize) -> CResult<()> {
    debug!("check_node_max_length: {node:?} max_length: {len}");
    let node_length = node.length();
    if node_length <= len {
        Ok(())
    } else {
        Err(CompiledSchemaError::MaxLengthError {
            expected: len,
            found: node_length,
            node: format!("{node}"),
        })
    }
}

/*
fn check_node_min_inclusive(node: &Node, min: &NumericLiteral) -> CResult<()> {
    debug!("check_node_min_inclusive: {node:?} min: {min}");
    if let Some(node_number) = node.numeric_value() {
        if node_number.less_than(min) {
            Ok(())
        } else {
            Err(CompiledSchemaError::MinInclusiveError {
                expected: min.clone(),
                found: node_number,
                node: format!("{node}"),
            })
        }
    } else {
        Err(CompiledSchemaError::NonNumeric {
            node: format!("{node}"),
        })
    }
}
*/

/*fn check_node_xs_facets(node: &Object, xs_facets: &Vec<XsFacet>) -> CResult<()> {
    Ok(()) // todo!()
}*/

fn todo<A>(str: &str) -> CResult<A> {
    Err(CompiledSchemaError::Todo {
        msg: str.to_string(),
    })
}

fn cnv_iri_ref(iri: &IriRef) -> Result<IriS, CompiledSchemaError> {
    match iri {
        IriRef::Iri(iri) => Ok(iri.clone()),
        _ => Err(CompiledSchemaError::Internal {
            msg: format!("Cannot convert {iri} to Iri"),
        }),
    }
}
