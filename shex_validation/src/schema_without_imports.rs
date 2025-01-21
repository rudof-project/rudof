use iri_s::IriS;
use prefixmap::IriRef;
use serde_derive::{Deserialize, Serialize};
use shex_ast::{IriOrStr, Schema, SchemaJsonError, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel};
use shex_compact::ShExParser;
use std::collections::{hash_map::Entry, HashMap};
use url::Url;

use crate::{ResolveMethod, SchemaWithoutImportsError, ShExFormat};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct SchemaWithoutImports {
    source_schema: Box<Schema>,

    local_shapes_counter: usize,

    imported_schemas: Vec<IriOrStr>,

    #[serde(skip)]
    shapes_map: HashMap<ShapeExprLabel, (ShapeExpr, IriS)>,
}

impl SchemaWithoutImports {
    pub fn resolve_iriref(&self, iri_ref: &IriRef) -> IriS {
        self.source_schema.resolve_iriref(iri_ref)
    }

    /// Return the number of shapes declared in the current schema without counting the ones from imported schemas
    pub fn local_shapes_count(&self) -> usize {
        self.local_shapes_counter
    }

    /// Return the total number of shapes declared included the ones from imported schemas
    pub fn total_shapes_count(&self) -> usize {
        self.shapes_map.len()
    }

    /// Returns an iterator of the shape expressions that the schema contains
    /// For exach shape expression, it returns the label and a pair that contains the shape expression and an optional IRI that points to the source where this shape expression has been imported
    /// If `None` it means this is a local shape expression defined in the current schema
    pub fn shapes(&self) -> impl Iterator<Item = (&ShapeExprLabel, &(ShapeExpr, IriS))> {
        self.shapes_map.iter()
    }
    /// Get the list of imported schemas
    pub fn imported_schemas(&self) -> impl Iterator<Item = &IriOrStr> {
        self.imported_schemas.iter()
    }

    /// Resolve the imports declared in a schema
    pub fn resolve_imports(
        schema: &Schema,
        base: &Option<IriS>,
        resolve_method: Option<&ResolveMethod>,
    ) -> Result<SchemaWithoutImports, SchemaWithoutImportsError> {
        let resolve_method = match resolve_method {
            None => ResolveMethod::default(),
            Some(m) => m.clone(),
        };
        let mut visited = Vec::new();
        let mut pending: Vec<_> = schema.imports();
        let mut map = HashMap::new();
        let mut local_shapes_counter = 0;
        if let Some(shapes) = schema.shapes() {
            for decl in shapes {
                local_shapes_counter += 1;
                Self::add_shape_decl(&mut map, decl, &schema.source_iri())?;
            }
        }
        Self::resolve_imports_visited(&mut pending, &mut visited, base, &resolve_method, &mut map)?;
        Ok(SchemaWithoutImports {
            source_schema: Box::new(schema.clone()),
            local_shapes_counter,
            imported_schemas: visited.clone(),
            shapes_map: map,
        })
    }

    pub fn add_shape_decl(
        map: &mut HashMap<ShapeExprLabel, (ShapeExpr, IriS)>,
        decl: ShapeDecl,
        source_iri: &IriS,
    ) -> Result<(), SchemaWithoutImportsError> {
        let id = decl.id.clone();
        match map.entry(decl.id) {
            Entry::Occupied(entry) => {
                let (old_shape_expr, maybe_iri) = entry.get();
                return Err(SchemaWithoutImportsError::DuplicatedShapeDecl {
                    label: id,
                    old_shape_expr: Box::new(old_shape_expr.clone()),
                    imported_from: maybe_iri.clone(),
                    shape_expr2: Box::new(decl.shape_expr),
                });
            }
            Entry::Vacant(v) => {
                v.insert((decl.shape_expr.clone(), source_iri.clone()));
            }
        }
        Ok(())
    }

    pub fn resolve_imports_visited(
        pending: &mut Vec<IriOrStr>,
        visited: &mut Vec<IriOrStr>,
        base: &Option<IriS>,
        resolve_method: &ResolveMethod,
        map: &mut HashMap<ShapeExprLabel, (ShapeExpr, IriS)>,
    ) -> Result<(), SchemaWithoutImportsError> {
        while let Some(candidate) = pending.pop() {
            if !visited.contains(&candidate) {
                let candidate_iri = resolve_iri_or_str(&candidate, base, resolve_method)?;
                let new_schema = match resolve_method {
                    ResolveMethod::RotatingFormats(formats) => {
                        find_schema_rotating_formats(&candidate_iri, formats.clone(), base)
                    }
                    ResolveMethod::ByGuessingExtension => todo!(),
                    ResolveMethod::ByContentNegotiation => todo!(),
                }?;
                for i in new_schema.imports() {
                    if !visited.contains(&i) {
                        pending.push(i.clone())
                    }
                }
                if let Some(shapes) = new_schema.shapes() {
                    for decl in shapes {
                        Self::add_shape_decl(map, decl, &candidate_iri)?;
                    }
                }
                visited.push(candidate.clone());
            }
        }
        Ok(())
    }

    pub fn count_extends(&self) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for (_, (shape, _)) in self.shapes() {
            let extends_counter = match shape {
                ShapeExpr::Shape(Shape { extends: None, .. }) => Some(0),
                ShapeExpr::Shape(Shape {
                    extends: Some(es), ..
                }) => Some(es.len()),
                _ => None,
            };

            if let Some(ec) = extends_counter {
                match result.entry(ec) {
                    Entry::Occupied(mut v) => {
                        let r = v.get_mut();
                        *r += 1;
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(1);
                    }
                }
            }
        }
        result
    }
}

pub fn find_schema_rotating_formats(
    iri: &IriS,
    formats: Vec<ShExFormat>,
    base: &Option<IriS>,
) -> Result<Schema, SchemaWithoutImportsError> {
    for format in &formats {
        match get_schema_from_iri(iri, format, base) {
            Err(_e) => {
                // we ignore the errors by now...we could collect them in a structure and return more information about the errors
            }
            Ok(schema) => return Ok(schema),
        }
    }
    Err(SchemaWithoutImportsError::SchemaFromIriRotatingFormats {
        iri: iri.clone(),
        formats: format!("{formats:?}"),
    })
}

pub fn resolve_iri_or_str(
    value: &IriOrStr,
    base: &Option<IriS>,
    _resolve_method: &ResolveMethod,
) -> Result<IriS, SchemaWithoutImportsError> {
    match value {
        IriOrStr::IriS(iri) => Ok(iri.clone()),
        IriOrStr::String(str) => match Url::parse(str) {
            Ok(url) => Ok(IriS::new_unchecked(url.as_str())),
            Err(_e) => match base {
                None => todo!(),
                Some(base) => {
                    let iri =
                        base.join(str)
                            .map_err(|e| SchemaWithoutImportsError::ResolvingStrIri {
                                base: Box::new(base.clone()),
                                str: str.clone(),
                                error: Box::new(e),
                            })?;
                    Ok(iri)
                }
            },
        },
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn local_folder_as_iri() -> Result<IriS, SchemaJsonError> {
    use tracing::debug;

    let current_dir = std::env::current_dir().map_err(|e| SchemaJsonError::CurrentDir {
        error: format!("{e}"),
    })?;
    debug!("Current dir: {current_dir:?}");
    let url = Url::from_file_path(&current_dir)
        .map_err(|_e| SchemaJsonError::LocalFolderIriError { path: current_dir })?;
    debug!("url: {url}");
    Ok(IriS::new_unchecked(url.as_str()))
}

#[cfg(target_family = "wasm")]
pub fn local_folder_as_iri() -> Result<IriS, SchemaJsonError> {
    Err(SchemaJsonError::CurrentDir {
        error: String::from("No local folder on web"),
    })
}

pub fn get_schema_from_iri(
    iri: &IriS,
    format: &ShExFormat,
    base: &Option<IriS>,
) -> Result<Schema, SchemaWithoutImportsError> {
    match format {
        ShExFormat::ShExC => {
            let content =
                iri.dereference(base)
                    .map_err(|e| SchemaWithoutImportsError::DereferencingIri {
                        iri: iri.clone(),
                        error: format!("{e}"),
                    })?;
            let schema = ShExParser::parse(content.as_str(), None).map_err(|e| {
                SchemaWithoutImportsError::ShExCError {
                    error: format!("{e}"),
                    content: content.clone(),
                }
            })?;
            Ok(schema)
        }
        ShExFormat::ShExJ => {
            let schema =
                Schema::from_iri(iri).map_err(|e| SchemaWithoutImportsError::ShExJError {
                    iri: iri.clone(),
                    error: format!("{e}"),
                })?;
            Ok(schema)
        }
        ShExFormat::Turtle => {
            todo!()
        }
    }
}
