use indoc::formatdoc;
use shacl_ast::property_shape::PropertyShape;
use srdf::QuerySRDF;
use srdf::SHACLPath;

use crate::helper::sparql::select;
use crate::shape::FocusNode;
use crate::shape::ValueNode;
use crate::validate_error::ValidateError;

use super::Result;
use super::ValidatorRunner;

pub struct QueryValidatorRunner;

impl<S: QuerySRDF + 'static> ValidatorRunner<S> for QueryValidatorRunner {
    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &S::Term, focus_nodes: &mut FocusNode<S>) -> Result<()> {
        if S::term_is_bnode(node) {
            return Err(ValidateError::TargetNodeBlankNode);
        }
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn target_class(
        &self,
        store: &S,
        class: &S::Term,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        if S::term_is_iri(class) {
            let query = formatdoc! {"
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

                SELECT DISTINCT ?this
                WHERE {{
                    ?this rdf:type/rdfs:subClassOf* {} .
                }}
            ", class};
            focus_nodes.extend(select(store, query, "this")?);
            Ok(())
        } else {
            Err(ValidateError::TargetClassNotIri)
        }
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn target_object_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};
        focus_nodes.extend(select(store, query, "this")?);
        Ok(())
    }

    fn implicit_target_class(
        &self,
        _store: &S,
        _shape: &S::Term,
        _focus_nodes: &mut FocusNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn predicate(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _predicate: &S::IRI,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
        _value_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        todo!()
    }
}
