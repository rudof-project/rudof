use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use indoc::formatdoc;
use iri_s::IriS;
use oxigraph::model::GraphNameRef;
use oxigraph::store::Store;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use srdf::RDFFormat;
use srdf::SHACLPath;

use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::select;
use crate::helper::term::Term;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

use super::ValidatorRunner;

// newtype idiom to encapsulate outer deps
pub(crate) struct OxigraphStore<'a>(&'a Store);

impl AsRef<Store> for OxigraphStore<'_> {
    fn as_ref(&self) -> &Store {
        self.0
    }
}

pub(crate) struct OxigraphRunner<'a> {
    store: OxigraphStore<'a>,
}

impl<'a> ValidatorRunner for OxigraphRunner<'a> {
    fn new(path: &str, rdf_format: RDFFormat, base: Option<&str>) -> super::Result<Self> {
        let store = &Store::new()?;

        store.bulk_loader().load_graph(
            BufReader::new(File::open(path)?),
            match rdf_format {
                RDFFormat::Turtle => oxigraph::io::GraphFormat::Turtle,
                RDFFormat::NTriples => oxigraph::io::GraphFormat::NTriples,
                RDFFormat::RDFXML => oxigraph::io::GraphFormat::RdfXml,
                _ => todo!(),
            },
            GraphNameRef::DefaultGraph,
            base,
        )?;

        Ok(OxigraphRunner {
            store: OxigraphStore(&store),
        })
    }

    fn evaluate(
        &self,
        component: &Component,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> super::Result<()> {
        Ok(component.evaluate(&self.store, value_nodes, report)?)
    }

    fn target_node(&self, node: &Term, focus_nodes: &mut HashSet<Term>) -> super::Result<()> {
        if let Term::BlankNode(_) = node {
            return Err(ValidateError::TargetNodeBlankNode);
        }

        let query = formatdoc! {"
                SELECT DISTINCT ?this
                WHERE {{
                    BIND ({} AS ?this)
                }}
            ",
            node
        };
        focus_nodes.extend(select(&self.store, query)?);
        Ok(())
    }

    fn target_class(&self, class: &Term, focus_nodes: &mut HashSet<Term>) -> super::Result<()> {
        match class {
            Term::IRI(_) => {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

                    SELECT DISTINCT ?this
                    WHERE {{
                        ?this rdf:type/rdfs:subClassOf* {} .
                    }}
                ", class};
                focus_nodes.extend(select(&self.store, query)?);
                Ok(())
            }
            Term::BlankNode(_) => Err(ValidateError::TargetClassBlankNode),
            Term::Literal(_) => Err(ValidateError::TargetClassLiteral),
        }
    }

    fn target_subject_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};
        focus_nodes.extend(select(&self.store, query)?);
        Ok(())
    }

    fn target_object_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};
        focus_nodes.extend(select(&self.store, query)?);
        Ok(())
    }

    fn predicate(&self, predicate: &IriS, value_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        paths: &Vec<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        paths: &Vec<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn inverse(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }
}
