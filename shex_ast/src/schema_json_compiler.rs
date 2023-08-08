use log::debug;
use iri_s::IriS;

use crate::{ShapeLabel, ShapeLabelIdx, CompiledSchema, SchemaJson, CompiledSchemaError, schema_json, schema, IriRef, compiled_schema::SemAct, compiled_schema::Annotation};
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
                debug!("shape_dcl {idx:?} {se:?}");
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

    fn compile_shape_decl(&self, sd: &schema_json::ShapeDecl, idx: &ShapeLabelIdx, compiled_schema: &mut CompiledSchema) -> CResult<ShapeExpr> {
        self.compile_shape_expr(&sd.shape_expr, idx, compiled_schema)
    }

    fn compile_shape_expr(&self, se: &schema_json::ShapeExpr, idx: &ShapeLabelIdx, compiled_schema: &mut CompiledSchema) -> CResult<ShapeExpr> {
        match se {
            schema_json::ShapeExpr::Ref(se_ref) => {
               match se_ref {
                  schema_json::Ref::IriRef { value} => {
                    let idx = self.get_shape_label_idx(&value, compiled_schema)?;
                    Ok(ShapeExpr::Ref{ idx })
                  },
                  schema_json::Ref::BNode { value} => {
                     todo!()
                  }
               }
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
                /*let expression = match expression {
                    Some(ref e) => {
                      let te = self.cnv_triple_expr(e)?;
                      Some(te)
                    },
                    None => None
                };*/
                Ok(ShapeExpr::Shape {
                    closed: Self::cnv_closed(closed),
                    extra: new_extra,
                    // expression: expression,
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
        if let Some(vs) = sem_acts {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }

    fn cnv_annotations(annotations: &Option<Vec<schema_json::Annotation>>) -> Vec<Annotation> {
        if let Some(anns) = annotations {
            // TODO
            Vec::new()
        } else {
            Vec::new()
        }
    }



}