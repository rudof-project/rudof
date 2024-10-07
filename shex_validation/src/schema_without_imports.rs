use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};
use shex_ast::{IriOrStr, Schema, SchemaJsonError, ShapeExpr, ShapeExprLabel};
use shex_compact::ShExParser;
use std::collections::{hash_map::Entry, HashMap};
use tracing::debug;
use url::Url;

use crate::{ResolveMethod, SchemaWithoutImportsError, ShExFormat};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct SchemaWithoutImports {
    source_schema: Box<Schema>,

    local_shapes_counter: usize,

    imported_schemas: Vec<IriOrStr>,

    #[serde(skip)]
    shapes_map: HashMap<ShapeExprLabel, (ShapeExpr, Option<IriS>)>,
}

impl SchemaWithoutImports {
    /// Return the number of shapes declared in the current schema without counting the ones from imported schemas
    pub fn local_shapes_count(&self) -> usize {
        self.local_shapes_counter
    }

    /// Return the total number of shapes declared included the ones from imported schemas
    pub fn total_shapes_count(&self) -> usize {
        self.shapes_map.len()
    }
    pub fn shapes(&self) -> impl Iterator<Item = (&ShapeExprLabel, &(ShapeExpr, Option<IriS>))> {
        self.shapes_map.iter()
    }
    /// Get the list of imported schemas
    pub fn imported_schemas(&self) -> impl Iterator<Item = &IriOrStr> {
        self.imported_schemas.iter()
    }

    /// Resolve the imports declared in a schema
    pub fn resolve_imports(
        schema: &Schema,
        resolve_method: Option<&ResolveMethod>,
    ) -> Result<SchemaWithoutImports, SchemaJsonError> {
        let resolve_method = match resolve_method {
            None => ResolveMethod::default(),
            Some(m) => m.clone(),
        };
        let mut visited = Vec::new();
        let base = match &schema.base() {
            Some(b) => b.clone(),
            None => {
                let local_folder = local_folder_as_iri()?;
                local_folder.clone()
            }
        };
        let mut pending: Vec<_> = schema.imports();
        let mut map: HashMap<ShapeExprLabel, (ShapeExpr, Option<IriS>)> = HashMap::new();
        let mut local_shapes_counter = 0;
        if let Some(shapes) = schema.shapes() {
            for decl in shapes {
                let id = decl.id.clone();
                local_shapes_counter += 1;
                match map.entry(decl.id) {
                    Entry::Occupied(entry) => {
                        let (old_shape_expr, maybe_iri) = entry.get();
                        return Err(SchemaJsonError::DuplicatedShapeDecl {
                            label: id,
                            old_shape_expr: Box::new(old_shape_expr.clone()),
                            imported_from: maybe_iri.clone(),
                            shape_expr2: Box::new(decl.shape_expr),
                        });
                    }
                    Entry::Vacant(v) => {
                        v.insert((decl.shape_expr.clone(), None));
                    }
                }
            }
        }
        Self::resolve_imports_visited(
            &mut pending,
            &mut visited,
            &base,
            &resolve_method,
            &mut map,
        )?;
        Ok(SchemaWithoutImports {
            source_schema: Box::new(schema.clone()),
            local_shapes_counter,
            imported_schemas: visited.clone(),
            shapes_map: map,
        })
    }

    pub fn resolve_imports_visited(
        pending: &mut Vec<IriOrStr>,
        visited: &mut Vec<IriOrStr>,
        base: &IriS,
        resolve_method: &ResolveMethod,
        map: &mut HashMap<ShapeExprLabel, (ShapeExpr, Option<IriS>)>,
    ) -> Result<(), SchemaJsonError> {
        while let Some(candidate) = pending.pop() {
            if !visited.contains(&candidate) {
                let candidate_iri = resolve_iri_or_str(&candidate, base, resolve_method)?;
                let new_schema = match resolve_method {
                    ResolveMethod::RotatingFormats(formats) => {
                        find_schema_rotating_formats(&candidate_iri, formats.clone())
                    }
                    ResolveMethod::ByGuessingExtension => todo!(),
                    ResolveMethod::ByContentNegotiation => todo!(),
                }?;
                for i in new_schema.imports() {
                    if !visited.contains(&i) {
                        pending.push(i.clone())
                    }
                }
                /*for shape_decl in new_schema.shapes_map().keys() {
                    match self.shapes_map.entry(shape_decl.)
                }*/
                visited.push(candidate.clone());
            }
        }
        Ok(())
    }
}

pub fn find_schema_rotating_formats(
    iri: &IriS,
    formats: Vec<ShExFormat>,
) -> Result<Schema, SchemaJsonError> {
    for format in &formats {
        match get_schema_from_iri(iri, format) {
            Err(_e) => {
                // we ignore the errors by now...we could collect them in a structure and return more information about the errors
            }
            Ok(schema) => return Ok(schema),
        }
    }
    Err(SchemaJsonError::SchemaFromIriRotatingFormats {
        iri: iri.clone(),
        formats: format!("{formats:?}"),
    })
}

pub fn resolve_iri_or_str(
    value: &IriOrStr,
    base: &IriS,
    resolve_method: &ResolveMethod,
) -> Result<IriS, SchemaJsonError> {
    match value {
        IriOrStr::IriS(iri) => Ok(iri.clone()),
        IriOrStr::String(str) => match Url::parse(str) {
            Ok(url) => Ok(IriS::new_unchecked(url.as_str())),
            Err(_e) => {
                let iri = base
                    .extend(str)
                    .map_err(|e| SchemaJsonError::ResolvingStrIri {
                        base: base.clone(),
                        str: str.clone(),
                        error: e,
                    })?;
                Ok(iri)
            }
        },
    }
}

pub fn local_folder_as_iri() -> Result<IriS, SchemaJsonError> {
    let current_dir = std::env::current_dir().map_err(|e| SchemaJsonError::CurrentDir {
        error: format!("{e}"),
    })?;
    debug!("Current dir: {current_dir:?}");
    let url = Url::from_file_path(&current_dir)
        .map_err(|_e| SchemaJsonError::LocalFolderIriError { path: current_dir })?;
    debug!("url: {url}");
    Ok(IriS::new_unchecked(url.as_str()))
}

pub fn get_schema_from_iri(
    iri: &IriS,
    format: &ShExFormat,
) -> Result<Schema, SchemaWithoutImportsError> {
    match format {
        ShExFormat::ShExC => {
            let content =
                iri.dereference()
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
                Schema::from_iri(&iri).map_err(|e| SchemaWithoutImportsError::ShExJError {
                    iri: iri.clone(),
                    error: format!("{e}"),
                })?;
            Ok(schema)
        }
    }
}
