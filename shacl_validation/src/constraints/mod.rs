use std::collections::HashSet;

use constraint_error::ConstraintError;
use core::cardinality::max_count::MaxCount;
use core::cardinality::min_count::MinCount;
use core::logical::and::And;
use core::logical::not::Not;
use core::logical::or::Or;
use core::logical::xone::Xone;
use core::other::closed::Closed;
use core::other::has_value::HasValue;
use core::other::r#in::In;
use core::property_pair::disjoint::Disjoint;
use core::property_pair::equals::Equals;
use core::property_pair::less_than::LessThan;
use core::property_pair::less_than_or_equals::LessThanOrEquals;
use core::shape_based::node::Node;
use core::shape_based::qualified_value_shape::QualifiedValue;
use core::string_based::language_in::LanguageIn;
use core::string_based::max_length::MaxLength;
use core::string_based::min_length::MinLength;
use core::string_based::pattern::Pattern;
use core::string_based::unique_lang::UniqueLang;
use core::value::class::Class;
use core::value::datatype::Datatype;
use core::value::node_kind::Nodekind;
use core::value_range::max_exclusive::MaxExclusive;
use core::value_range::max_inclusive::MaxInclusive;
use core::value_range::min_exclusive::MinExclusive;
use core::value_range::min_inclusive::MinInclusive;
use shacl_ast::component::Component;
use srdf::{SRDFBasic, SRDF};

use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;
use crate::validation_report::result::ValidationResultBuilder;

pub(crate) mod constraint_error;
pub mod core;

pub trait ConstraintComponent<S> {
    fn evaluate(
        &self,
        store: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError>;

    fn make_validation_result(&self, value_node: Option<&Term>, report: &mut ValidationReport) {
        let mut builder = ValidationResultBuilder::default();

        if let Some(focus_node) = value_node {
            builder.focus_node(focus_node);
        }

        report.add_result(builder.build());
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for Component {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        match self.to_owned() {
            Component::Class(node) => Class::new(node).evaluate(store, value_nodes, report),
            Component::Datatype(iri_ref) => {
                Datatype::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::NodeKind(node_kind) => {
                Nodekind::new(node_kind).evaluate(store, value_nodes, report)
            }
            Component::MinCount(min_count) => {
                MinCount::new(min_count).evaluate(store, value_nodes, report)
            }
            Component::MaxCount(max_count) => {
                MaxCount::new(max_count).evaluate(store, value_nodes, report)
            }
            Component::MinExclusive(literal) => {
                MinExclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MaxExclusive(literal) => {
                MaxExclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MinInclusive(literal) => {
                MinInclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MaxInclusive(literal) => {
                MaxInclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MinLength(min_length) => {
                MinLength::new(min_length).evaluate(store, value_nodes, report)
            }
            Component::MaxLength(max_lenth) => {
                MaxLength::new(max_lenth).evaluate(store, value_nodes, report)
            }
            Component::Pattern { pattern, flags } => {
                Pattern::new(pattern, flags).evaluate(store, value_nodes, report)
            }
            Component::UniqueLang(lang) => {
                UniqueLang::new(lang).evaluate(store, value_nodes, report)
            }
            Component::LanguageIn { langs } => {
                LanguageIn::new(langs).evaluate(store, value_nodes, report)
            }
            Component::Equals(iri_ref) => Equals::new(iri_ref).evaluate(store, value_nodes, report),
            Component::Disjoint(iri_ref) => {
                Disjoint::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::LessThan(iri_ref) => {
                LessThan::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::LessThanOrEquals(iri) => {
                LessThanOrEquals::new(iri).evaluate(store, value_nodes, report)
            }
            Component::Or { shapes } => Or::new(shapes).evaluate(store, value_nodes, report),
            Component::And { shapes } => And::new(shapes).evaluate(store, value_nodes, report),
            Component::Not { shape } => Not::new(shape).evaluate(store, value_nodes, report),
            Component::Xone { shapes } => Xone::new(shapes).evaluate(store, value_nodes, report),
            Component::Closed {
                is_closed,
                ignored_properties,
            } => Closed::new(is_closed, ignored_properties).evaluate(store, value_nodes, report),
            Component::Node { shape } => Node::new(shape).evaluate(store, value_nodes, report),
            Component::HasValue { value } => {
                HasValue::new(value).evaluate(store, value_nodes, report)
            }
            Component::In { values } => In::new(values).evaluate(store, value_nodes, report),
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => QualifiedValue::new(
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            )
            .evaluate(store, value_nodes, report),
        }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Component {
    fn evaluate(
        &self,
        store: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        match self.to_owned() {
            Component::Class(node) => Class::new(node).evaluate(store, value_nodes, report),
            Component::Datatype(iri_ref) => {
                Datatype::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::NodeKind(node_kind) => {
                Nodekind::new(node_kind).evaluate(store, value_nodes, report)
            }
            Component::MinCount(min_count) => {
                MinCount::new(min_count).evaluate(store, value_nodes, report)
            }
            Component::MaxCount(max_count) => {
                MaxCount::new(max_count).evaluate(store, value_nodes, report)
            }
            Component::MinExclusive(literal) => {
                MinExclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MaxExclusive(literal) => {
                MaxExclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MinInclusive(literal) => {
                MinInclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MaxInclusive(literal) => {
                MaxInclusive::new(literal).evaluate(store, value_nodes, report)
            }
            Component::MinLength(min_length) => {
                MinLength::new(min_length).evaluate(store, value_nodes, report)
            }
            Component::MaxLength(max_lenth) => {
                MaxLength::new(max_lenth).evaluate(store, value_nodes, report)
            }
            Component::Pattern { pattern, flags } => {
                Pattern::new(pattern, flags).evaluate(store, value_nodes, report)
            }
            Component::UniqueLang(lang) => {
                UniqueLang::new(lang).evaluate(store, value_nodes, report)
            }
            Component::LanguageIn { langs } => {
                LanguageIn::new(langs).evaluate(store, value_nodes, report)
            }
            Component::Equals(iri_ref) => Equals::new(iri_ref).evaluate(store, value_nodes, report),
            Component::Disjoint(iri_ref) => {
                Disjoint::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::LessThan(iri_ref) => {
                LessThan::new(iri_ref).evaluate(store, value_nodes, report)
            }
            Component::LessThanOrEquals(iri) => {
                LessThanOrEquals::new(iri).evaluate(store, value_nodes, report)
            }
            Component::Or { shapes } => Or::new(shapes).evaluate(store, value_nodes, report),
            Component::And { shapes } => And::new(shapes).evaluate(store, value_nodes, report),
            Component::Not { shape } => Not::new(shape).evaluate(store, value_nodes, report),
            Component::Xone { shapes } => Xone::new(shapes).evaluate(store, value_nodes, report),
            Component::Closed {
                is_closed,
                ignored_properties,
            } => Closed::new(is_closed, ignored_properties).evaluate(store, value_nodes, report),
            Component::Node { shape } => Node::new(shape).evaluate(store, value_nodes, report),
            Component::HasValue { value } => {
                HasValue::new(value).evaluate(store, value_nodes, report)
            }
            Component::In { values } => In::new(values).evaluate(store, value_nodes, report),
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => QualifiedValue::new(
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            )
            .evaluate(store, value_nodes, report),
        }
    }
}
