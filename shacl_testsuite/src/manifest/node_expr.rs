use crate::TestSuiteError;
use crate::manifest::{Manifest, ManifestEntry, ManifestGraph};
use oxrdf::{NamedNode, NamedOrBlankNode, Term};
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::vocabs::{RdfsVocab, ShaclTestVocab, TestManifestVocab};
use rudof_rdf::rdf_core::{Any, NeighsRDF};
use shacl::ast::ASTNodeExpr;
use shacl::ir::{IRNodeExpr, IRSchema};
use shacl::rdf::ShaclParser;
use shacl::validator::RecursionSemantics;
use shacl::validator::engine::NativeEngine;
use std::collections::HashMap;

/// A manifest for `sht:EvalNodeExpr` tests
pub type NodeExprManifest = Manifest<NodeExprTest>;

/// One `sht:EvalNodeExpr` test case
pub struct NodeExprTest {
    pub label: String,
    pub node_expr: Term,
    pub fnode: Option<Term>,
    pub scope: HashMap<String, Term>,
    pub ignore_order: bool,
    pub expected: Vec<Term>,
}

impl ManifestEntry for NodeExprTest {
    fn parse(manifest: &mut ManifestGraph, entry: NamedOrBlankNode) -> Result<Self, TestSuiteError> {
        let label = manifest
            .value(&entry, &RdfsVocab::rdfs_label().into())?
            .map_or_else(|| entry.to_string(), |label| label.to_string());

        let action: NamedOrBlankNode = manifest
            .required_value(&entry, &TestManifestVocab::mf_action().into())?
            .try_into()?;

        let node_expr = manifest.required_value(&action, &ShaclTestVocab::sht_node_expr().into())?;
        let fnode = manifest.value(&action, &ShaclTestVocab::sht_focus_node().into())?;

        let ignore_order = manifest
            .value(&action, &ShaclTestVocab::sht_ignore_order().into())?
            .is_some_and(|t| t.to_string().contains("true"));

        let mut scope = HashMap::new();
        for triple in manifest.store().triples_matching(&action, &Any, &Any)? {
            let predicate = triple.pred().as_str().to_owned();
            if let Some(name) = predicate.strip_prefix(ShaclTestVocab::SHT_SCOPE_PREFIX) {
                scope.insert(name.to_owned(), triple.obj().clone());
            }
        }

        let expected = match manifest.value(&entry, &TestManifestVocab::mf_result().into())? {
            None => Vec::new(),
            Some(res) => manifest.list(res)?,
        };

        Ok(Self {
            label,
            node_expr,
            fnode,
            scope,
            ignore_order,
            expected,
        })
    }
}

impl NodeExprManifest {
    /// Parses, compiles and evaluates test's node expressions
    pub fn evaluate(&self, test: &NodeExprTest) -> Result<Vec<Term>, TestSuiteError> {
        let store = self.graph().store();
        let mut parser = ShaclParser::new(store.clone());
        let ast = parser.parse()?;
        let expr: ASTNodeExpr = parser.parse_node_expr(&test.node_expr)?;
        let mut ir = IRSchema::compile(&ast)?;
        let expr: IRNodeExpr = IRNodeExpr::compile(&expr, &ast, &mut ir)?;

        let focus = test
            .fnode
            .clone()
            .unwrap_or_else(|| Term::NamedNode(NamedNode::new_unchecked("urn:x-rudof:no-focus-node")));

        let mut engine = NativeEngine::new(RecursionSemantics::default());

        let values = expr
            .evaluate(store, &focus, &test.scope, &mut engine, &ir)
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        Ok(values)
    }
}
