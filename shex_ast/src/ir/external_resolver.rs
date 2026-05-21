//! Pluggable resolution of EXTERNAL shape expressions.
//!
//! The ShEx specification states that an EXTERNAL shape conforms when
//! "implementation-specific mechanisms not defined in this specification
//! indicate success". `rudof` exposes this extension point as a chain of
//! resolvers that can:
//!
//! 1. Rewrite the AST before compilation: substituting an EXTERNAL `ShapeDecl`
//!    with the real definition.
//! 2. Answer a verdict at validation time for any EXTERNAL that survived
//!    rewriting.
//!
//! Resolvers are consulted by an [`ExternalShapeResolverRegistry`], which is
//! held inside `shex_validation::ValidatorConfig::external_resolvers`.

use crate::ast;
use crate::ir::{schema_ir::SchemaIR, shape_label::ShapeLabel};
use crate::node::Node;
use crate::{ShapeExprLabel, ShapeLabelIdx};
use rudof_iri::IriS;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

/// Context exposed to a resolver at validation time.
pub struct ExternalResolveCtx<'a> {
    pub node: &'a Node,
    pub shape_idx: ShapeLabelIdx,
    pub shape_label: Option<&'a ShapeLabel>,
    pub schema: &'a SchemaIR,
}

/// A resolver's verdict at validation time. Resolvers that rewrite the AST
/// typically `Abstain` at validation time, since substituted shapes no longer
/// appear as EXTERNAL.
#[derive(Debug, Clone)]
pub enum ExternalResolution {
    Conformant { rationale: String },
    NonConformant { rationale: String },
    Abstain,
}

/// Outcome surfaced to the engine after the registry dispatches through its
/// resolver chain. Carries the resolver's name so the resulting `Reason`/
/// `ValidatorError` can attribute the decision.
#[derive(Debug, Clone)]
pub enum DispatchOutcome {
    Conformant { resolver: String, rationale: String },
    NonConformant { resolver: String, rationale: String },
    Abstain,
}

/// Pluggable resolution of EXTERNAL shape expressions.
pub trait ExternalShapeResolver: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;

    /// AST-rewrite pass. Default: identity (no rewriting).
    fn rewrite_ast(&self, schema: ast::Schema) -> ast::Schema {
        schema
    }

    /// Runtime fallback for `ShapeExpr::External {}` that survived rewriting.
    /// Default: `Abstain`, deferring to the next resolver in the registry.
    fn resolve(&self, _ctx: &ExternalResolveCtx<'_>) -> ExternalResolution {
        ExternalResolution::Abstain
    }
}

/// Ordered chain of resolvers plus a guaranteed terminator
/// (`RejectAllExternalResolver` by default).
#[derive(Debug, Clone)]
pub struct ExternalShapeResolverRegistry {
    resolvers: Vec<Arc<dyn ExternalShapeResolver>>,
}

impl Default for ExternalShapeResolverRegistry {
    /// Default registry: a single [`RejectAllExternalResolver`]. Clients
    /// prepend other resolvers via [`Self::with_resolver`], so the rejecting
    /// terminator always sits at the end of the chain.
    fn default() -> Self {
        Self {
            resolvers: vec![Arc::new(RejectAllExternalResolver::default())],
        }
    }
}

impl ExternalShapeResolverRegistry {
    /// Build an empty registry. Production callers should prefer
    /// [`Self::default`] which installs `RejectAllExternalResolver` as a
    /// terminator. An empty registry surfaces [`DispatchOutcome::Abstain`].
    pub fn empty() -> Self {
        Self { resolvers: vec![] }
    }

    /// Prepend a resolver so it is consulted before all previously-registered
    /// resolvers (in particular, before the default `RejectAllExternalResolver`).
    pub fn with_resolver<R: ExternalShapeResolver + 'static>(mut self, r: R) -> Self {
        self.resolvers.insert(0, Arc::new(r));
        self
    }

    /// Same as [`Self::with_resolver`], but accepts an `Arc` directly so the
    /// caller can share a single resolver across registries.
    pub fn with_resolver_arc(mut self, r: Arc<dyn ExternalShapeResolver>) -> Self {
        self.resolvers.insert(0, r);
        self
    }

    pub fn resolvers(&self) -> &[Arc<dyn ExternalShapeResolver>] {
        &self.resolvers
    }

    /// Apply every resolver's `rewrite_ast` in registry order.
    pub fn rewrite_ast(&self, mut schema: ast::Schema) -> ast::Schema {
        for r in &self.resolvers {
            schema = r.rewrite_ast(schema);
        }
        schema
    }

    /// Consult resolvers in order; the first non-`Abstain` answer wins.
    pub fn dispatch(&self, ctx: &ExternalResolveCtx<'_>) -> DispatchOutcome {
        for r in &self.resolvers {
            match r.resolve(ctx) {
                ExternalResolution::Abstain => continue,
                ExternalResolution::Conformant { rationale } => {
                    return DispatchOutcome::Conformant {
                        resolver: r.name().to_string(),
                        rationale,
                    };
                },
                ExternalResolution::NonConformant { rationale } => {
                    return DispatchOutcome::NonConformant {
                        resolver: r.name().to_string(),
                        rationale,
                    };
                },
            }
        }
        DispatchOutcome::Abstain
    }
}

/// Always-reject resolver. Registered by default; rejects any EXTERNAL that
/// no other resolver claimed.
#[derive(Debug, Clone, Default)]
pub struct RejectAllExternalResolver;

impl ExternalShapeResolver for RejectAllExternalResolver {
    fn name(&self) -> &str {
        "reject-all"
    }

    fn resolve(&self, _ctx: &ExternalResolveCtx<'_>) -> ExternalResolution {
        ExternalResolution::NonConformant {
            rationale: "EXTERNAL shape rejected: no resolver supplied a definition".to_string(),
        }
    }
}

/// File-backed resolver. Loads a separate ShEx schema once at construction and,
/// during the AST-rewrite pass, substitutes any `ShapeDecl` whose body is
/// `ShapeExpr::External` and whose label exists in the externs file with the
/// externs shape expression. At validation time it abstains: once a label has
/// been substituted, the engine sees a regular shape, not EXTERNAL.
#[derive(Debug, Clone)]
pub struct SchemaExternalResolver {
    name: String,
    externs: ast::Schema,
}

impl SchemaExternalResolver {
    /// Parse a ShEx schema file (e.g. `.shex`, `.shextern`) and use it as the
    /// source of EXTERNAL definitions. Uses the standard ShExC parser.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ExternalResolverError> {
        let path = path.as_ref();
        let source_iri: IriS = path.try_into().map_err(|e: rudof_iri::error::IriSError| ExternalResolverError::PathToIri {
            path: path.to_path_buf(),
            error: e.to_string(),
        })?;
        let externs = crate::ShExParser::parse_buf(path, Some(source_iri))
            .map_err(|e| ExternalResolverError::Parse {
                path: path.to_path_buf(),
                error: e.to_string(),
            })?;
        Ok(Self {
            name: format!("schema:{}", path.display()),
            externs,
        })
    }

    /// Construct from an already-parsed `ast::Schema`. Useful in tests.
    pub fn from_schema(name: impl Into<String>, externs: ast::Schema) -> Self {
        Self {
            name: name.into(),
            externs,
        }
    }

    pub fn externs(&self) -> &ast::Schema {
        &self.externs
    }
}

impl ExternalShapeResolver for SchemaExternalResolver {
    fn name(&self) -> &str {
        &self.name
    }

    fn rewrite_ast(&self, mut schema: ast::Schema) -> ast::Schema {
        let lookups = match self.externs.shapes() {
            Some(s) => s,
            None => return schema,
        };
        if let Some(decls) = schema.shapes_mut() {
            for decl in decls.iter_mut() {
                if matches!(decl.shape_expr, ast::ShapeExpr::External) {
                    if let Some(matching) = lookup_decl(&lookups, &decl.id) {
                        decl.shape_expr = matching.shape_expr.clone();
                    }
                }
            }
        }
        schema
    }
}

fn lookup_decl<'a>(
    decls: &'a [ast::ShapeDecl],
    label: &ShapeExprLabel,
) -> Option<&'a ast::ShapeDecl> {
    decls.iter().find(|d| &d.id == label)
}

#[derive(Debug, Error)]
pub enum ExternalResolverError {
    #[error("Could not convert path {path:?} into IRI: {error}")]
    PathToIri { path: std::path::PathBuf, error: String },

    #[error("Could not parse external shapes file {path:?}: {error}")]
    Parse {
        path: std::path::PathBuf,
        error: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Schema, ShapeDecl, ShapeExpr};
    use crate::ir::actions::semantic_actions_registry::SemanticActionsRegistry;

    fn label(iri: &str) -> ShapeExprLabel {
        ShapeExprLabel::iri_unchecked(iri)
    }

    fn schema_with(decls: Vec<ShapeDecl>) -> Schema {
        Schema::default().with_shapes(Some(decls))
    }

    fn dummy_ctx<'a>(node: &'a Node, schema: &'a SchemaIR) -> ExternalResolveCtx<'a> {
        ExternalResolveCtx {
            node,
            shape_idx: ShapeLabelIdx::default(),
            shape_label: None,
            schema,
        }
    }

    #[test]
    fn default_registry_rejects() {
        let reg = ExternalShapeResolverRegistry::default();
        let names: Vec<_> = reg.resolvers().iter().map(|r| r.name().to_string()).collect();
        assert_eq!(names, vec!["reject-all".to_string()]);

        // The reject-all terminator should answer NonConformant via dispatch.
        let node = Node::iri(IriS::new_unchecked("http://example/n"));
        let schema = SchemaIR::new(SemanticActionsRegistry::default());
        let ctx = dummy_ctx(&node, &schema);
        assert!(matches!(
            reg.dispatch(&ctx),
            DispatchOutcome::NonConformant { .. }
        ));
    }

    #[test]
    fn empty_registry_abstains() {
        let reg = ExternalShapeResolverRegistry::empty();
        let node = Node::iri(IriS::new_unchecked("http://example/n"));
        let schema = SchemaIR::new(SemanticActionsRegistry::default());
        let ctx = dummy_ctx(&node, &schema);
        assert!(matches!(reg.dispatch(&ctx), DispatchOutcome::Abstain));
    }

    #[test]
    fn schema_resolver_substitutes_matching_label() {
        let sext = label("http://a.example/Sext");
        let externs_shape = ShapeExpr::empty_shape();
        let externs = schema_with(vec![ShapeDecl::new(sext.clone(), externs_shape.clone(), false)]);

        let main = schema_with(vec![ShapeDecl::new(sext.clone(), ShapeExpr::External, false)]);

        let resolver = SchemaExternalResolver::from_schema("test", externs);
        let rewritten = resolver.rewrite_ast(main);

        let decls = rewritten.shapes().expect("shapes present");
        assert_eq!(decls.len(), 1);
        assert!(!matches!(decls[0].shape_expr, ShapeExpr::External));
        assert_eq!(decls[0].shape_expr, externs_shape);
    }

    #[test]
    fn schema_resolver_leaves_unknown_labels_external() {
        let known = label("http://a.example/Known");
        let unknown = label("http://a.example/Unknown");
        let externs = schema_with(vec![ShapeDecl::new(known, ShapeExpr::empty_shape(), false)]);
        let main = schema_with(vec![ShapeDecl::new(unknown, ShapeExpr::External, false)]);

        let resolver = SchemaExternalResolver::from_schema("test", externs);
        let rewritten = resolver.rewrite_ast(main);

        let decls = rewritten.shapes().unwrap();
        assert!(matches!(decls[0].shape_expr, ShapeExpr::External));
    }

    #[test]
    fn registry_runs_user_resolvers_before_default() {
        let sext = label("http://a.example/Sext");
        let externs = schema_with(vec![ShapeDecl::new(sext.clone(), ShapeExpr::empty_shape(), false)]);
        let resolver = SchemaExternalResolver::from_schema("test", externs);

        let reg = ExternalShapeResolverRegistry::default().with_resolver(resolver);
        let names: Vec<_> = reg.resolvers().iter().map(|r| r.name().to_string()).collect();
        assert_eq!(names, vec!["test".to_string(), "reject-all".to_string()]);

        // rewrite_ast: user resolver substitutes the External; reject-all is a no-op for rewrite.
        let main = schema_with(vec![ShapeDecl::new(sext, ShapeExpr::External, false)]);
        let rewritten = reg.rewrite_ast(main);
        assert!(!matches!(rewritten.shapes().unwrap()[0].shape_expr, ShapeExpr::External));
    }
}
