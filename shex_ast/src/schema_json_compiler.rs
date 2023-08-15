use log::debug;
use iri_s::IriS;
use rbe::Cardinality;
use rbe::{MatchCond, Min, Max, RbeTable, rbe::Rbe, Component};
use srdf::Object;
use crate::ValueSetValueWrapper;
use crate::schema_json::{TripleExpr, NodeKind, XsFacet};
use crate::{ShapeLabel, ShapeLabelIdx, compiled_schema::CompiledSchema, SchemaJson, CompiledSchemaError, schema_json, IriRef, compiled_schema::SemAct, compiled_schema::Annotation};
use crate::compiled_schema::ShapeExpr;

type CResult<T> = Result<T, CompiledSchemaError>;

#[derive(Debug)]
pub struct SchemaJsonCompiler {
    shape_decls_counter: usize
}

impl SchemaJsonCompiler {

    pub fn new() -> SchemaJsonCompiler {
        SchemaJsonCompiler {
            shape_decls_counter: 0
        }
    }

    pub fn compile(&mut self, schema_json: &SchemaJson, compiled_schema: &mut CompiledSchema) -> CResult<()> {
        debug!("Compiling schema_json: {compiled_schema:?}");
        self.collect_shape_labels(schema_json, compiled_schema)?;
        // dbg!(compiled_schema);
        debug!("Shape labels collected {compiled_schema:?}");
        self.collect_shape_exprs(schema_json, compiled_schema)?;
        debug!("Shape exprs collected {compiled_schema:?}");
        // dbg!(compiled_schema);
        Ok(())
    }

    pub fn collect_shape_labels(&mut self, schema_json: &SchemaJson, compiled_schema: &mut CompiledSchema) -> CResult<()> {
        match &schema_json.shapes {
            None => Ok(()),
            Some(sds) => {
              for sd in sds {
                let label = self.id_to_shape_label(sd.id.as_str())?;
                compiled_schema.add_shape(label, ShapeExpr::Empty);
                self.shape_decls_counter += 1;
              }
              Ok(())
            }
        }
    }

    pub fn collect_shape_exprs(&mut self, schema_json: &SchemaJson, compiled_schema: &mut CompiledSchema) -> CResult<()> {
        match &schema_json.shapes {
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

    fn id_to_shape_label<'a>(&self, id: &str) -> CResult<ShapeLabel> {
        let label = ShapeLabel::from_iri_str(id)?;
        Ok(label)
    }

    fn get_shape_label_idx(&self, id: &str, compiled_schema: &mut CompiledSchema) -> CResult<ShapeLabelIdx> {
        let label = self.id_to_shape_label(id)?;
        compiled_schema.get_shape_label_idx(&label)
    }

    fn compile_shape_decl(&self, 
        sd: &schema_json::ShapeDecl, 
        idx: &ShapeLabelIdx, 
        compiled_schema: &mut CompiledSchema) -> CResult<ShapeExpr> {
        self.compile_shape_expr(&sd.shape_expr, idx, compiled_schema)
    }

    fn ref2idx(&self, sref: &schema_json::Ref, compiled_schema: &mut CompiledSchema) -> CResult<ShapeLabelIdx> {
        match sref {
            schema_json::Ref::IriRef { value} => {
              let idx = self.get_shape_label_idx(&value, compiled_schema)?;
              Ok(idx)
            },
            schema_json::Ref::BNode { value: _} => {
               todo!()
            }
         }
    }

    fn compile_shape_expr(&self, se: &schema_json::ShapeExpr, idx: &ShapeLabelIdx, compiled_schema: &mut CompiledSchema) -> CResult<ShapeExpr> {
        match se {
            schema_json::ShapeExpr::Ref(se_ref) => {
               let idx = self.ref2idx(se_ref, compiled_schema)?;
               Ok(ShapeExpr::Ref { idx })
            },
            schema_json::ShapeExpr::ShapeOr { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                    cnv.push(se);
                }
                Ok(ShapeExpr::ShapeOr { exprs: cnv })
            },
            schema_json::ShapeExpr::ShapeAnd { shape_exprs: ses } => {
                let mut cnv = Vec::new();
                for sew in ses {
                    let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                    cnv.push(se);
                }
                Ok(ShapeExpr::ShapeAnd { exprs: cnv })
            },
            schema_json::ShapeExpr::ShapeNot { shape_expr: sew } => {
                let se = self.compile_shape_expr(&sew.se, idx, compiled_schema)?;
                Ok(ShapeExpr::ShapeNot { expr: Box::new(se) })
            },
            schema_json::ShapeExpr::Shape {
                closed,
                extra,
                expression,
                sem_acts,
                annotations,
            } => {
                let new_extra = self.cnv_extra(extra)?;
                let rbe_table = match expression {
                    None => RbeTable::new(),
                    Some(tew) => {
                        let mut table = RbeTable::new();
                        let rbe = self.triple_expr2rbe(&tew.te, compiled_schema, &mut table)?;
                        table.with_rbe(rbe);
                        table
                    }
                };
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    rbe_table, 
                    sem_acts: Self::cnv_sem_acts(&sem_acts),
                    annotations: Self::cnv_annotations(&annotations),
                })
            }, 
            _ => {
              let se = compiled_schema.get_shape_expr(idx)?;
              match *se {
                // ShapeExpr::Empty => Ok(ShapeExpr::Empty),
                _ => panic!("Internal...should not come here for se={se:?}")
              }
            }
        }
    }

    fn cnv_closed(closed: &Option<bool>) -> bool {
        match closed {
            None => false,
            Some(closed) => *closed
        }
    }

    fn cnv_extra<'a>(&self, extra: &Option<Vec<IriRef>>) -> CResult<Vec<IriS>> {
        if let Some(extra) = extra {
            let mut vs = Vec::new();
            for iri in extra {
                let nm = self.cnv_iri_ref(&iri)?;
                vs.push(nm);
            }
            Ok(vs)
        } else {
            Ok(Vec::new())
        }
    }

    fn cnv_iri_ref<'a>(&self, iri: &IriRef) -> Result<IriS, CompiledSchemaError> {
        let iri = IriS::new(&iri.value.as_str())?;
        Ok(iri)
    }

    fn cnv_sem_acts(sem_acts: &Option<Vec<schema_json::SemAct>>) -> Vec<SemAct> {
        if let Some(_vs) = sem_acts {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn cnv_annotations(annotations: &Option<Vec<schema_json::Annotation>>) -> Vec<Annotation> {
        if let Some(_anns) = annotations {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn triple_expr2rbe(&self, 
        triple_expr: &schema_json::TripleExpr, 
        compiled_schema: &mut CompiledSchema, 
        current_table: &mut RbeTable<IriS, Object, ShapeLabelIdx>) -> CResult<Rbe<Component>> {
        match triple_expr {
            TripleExpr::EachOf { id:_ , expressions, min, max, sem_acts: _, annotations: _ } => { 
                let mut cs = Vec::new();
                for e in expressions {
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::and(cs.into_iter()), card))
            },
            TripleExpr::OneOf { id:_ , expressions, min, max, sem_acts:_ , annotations: _ } => { 
                let mut cs = Vec::new();
                for e in expressions {
                    let c = self.triple_expr2rbe(&e.te, compiled_schema, current_table)?;
                    cs.push(c)
                }
                let card = self.cnv_min_max(min, max)?;
                Ok(Self::mk_card_group(Rbe::or(cs.into_iter()), card))
            },
            TripleExpr::TripleConstraint { id:_, inverse: _, predicate, value_expr, min, max, sem_acts: _, annotations: _ } => {
                let min = self.cnv_min(&min)?;
                let max = self.cnv_max(&max)?;
                let iri = IriS::new(predicate.value.as_str())?;
                let cond = self.value_expr2match_cond(value_expr, compiled_schema)?;
                let c = current_table.add_component(iri, cond);
                Ok(Rbe::symbol(c, min.value, max))
            }
            _ => { todo!() }
        }
    }

    fn cnv_min_max(&self, min: &Option<i32>, max: &Option<i32>) -> CResult<Cardinality> {
        let min = self.cnv_min(&min)?;
        let max = self.cnv_max(&max)?;
        Ok(Cardinality::from(min, max))
    }

    fn mk_card_group(rbe: Rbe<Component>, card: Cardinality) -> Rbe<Component> {
        match &card {
            c if c.is_1_1() => rbe,
            c if c.is_star() => Rbe::Star {
                value: Box::new(rbe)
            },
            c if c.is_plus() => Rbe::Plus {
                value: Box::new(rbe)
            },
            _c  => Rbe::Repeat {
                value: Box::new(rbe),
                card
            }
        }
    }

    fn cnv_min(&self, min: &Option<i32>) -> CResult<Min> {
        match min { 
         Some(min) if *min < 0 => Err(CompiledSchemaError::MinLessZero { min: *min }),
         Some(min) => Ok(Min::from(*min)),
         None => Ok(Min::from(1))
        }
    }

    fn cnv_max(&self, max: &Option<i32>) -> CResult<Max> {
        match *max { 
            Some(-1) => Ok(Max::Unbounded),
            Some(max) if max < -1 => Err(CompiledSchemaError::MaxIncorrect { max }),
            Some(max) => Ok(Max::from(max)),
            None => Ok(Max::from(1))
           }
    }

    fn value_expr2match_cond(&self, ve: &Option<Box<schema_json::ShapeExpr>>, compiled_schema: &mut CompiledSchema) -> CResult<MatchCond<IriS, Object, ShapeLabelIdx>> {
       if let Some(se) = ve.as_deref() {
        match se {
            schema_json::ShapeExpr::NodeConstraint { node_kind, datatype, xs_facet, values } => {
                self.node_constraint2match_cond(node_kind, datatype, xs_facet, values)
            },
            schema_json::ShapeExpr::Ref(sref) => {
                let idx = self.ref2idx(sref, compiled_schema)?;
                Ok(MatchCond::new().with_name(format!("Ref{idx}")))
               },   
            _ => { 
                todo!() 
            }   
        }
       } else {
        Ok(MatchCond::new())
       }
    }


    #[allow(dead_code)]
    fn shape_expr2match_cond(&self, _se: schema_json::ShapeExpr, _compiled_schema: &mut CompiledSchema) -> CResult<MatchCond<IriS, Object, ShapeLabelIdx>> {
        todo!()
    }

    fn node_constraint2match_cond(&self, _node_kind: &Option<NodeKind>, _datatype: &Option<IriRef>, _xs_facet: &Option<Vec<XsFacet>>, _values: &Option<Vec<ValueSetValueWrapper>>) -> CResult<MatchCond<IriS, Object, ShapeLabelIdx>> {
        Ok(MatchCond::new().with_name("node_constraint".to_string()))
    }


}