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
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

pub(crate) mod constraint_error;
pub mod core;

pub(crate) trait ConstraintComponent<S: SRDFBasic> {
    fn evaluate(
        &self,
        executor: &dyn SHACLExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError>;
}

pub trait DefaultConstraintComponent<S: SRDF> {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError>;
}

pub trait SparqlConstraintComponent<S: QuerySRDF> {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError>;
}

impl<S: SRDF + 'static> From<&Component> for Box<dyn DefaultConstraintComponent<S>> {
    fn from(value: &Component) -> Self {
        match value.to_owned() {
            Component::Class(node) => Box::new(Class::new(node)),
            Component::Datatype(iri_ref) => Box::new(Datatype::new(iri_ref)),
            Component::NodeKind(node_kind) => Box::new(Nodekind::new(node_kind)),
            Component::MinCount(min_count) => Box::new(MinCount::new(min_count)),
            Component::MaxCount(max_count) => Box::new(MaxCount::new(max_count)),
            Component::MinExclusive(literal) => Box::new(MinExclusive::new(literal)),
            Component::MaxExclusive(literal) => Box::new(MaxExclusive::new(literal)),
            Component::MinInclusive(literal) => Box::new(MinInclusive::new(literal)),
            Component::MaxInclusive(literal) => Box::new(MaxInclusive::new(literal)),
            Component::MinLength(min_length) => Box::new(MinLength::new(min_length)),
            Component::MaxLength(max_lenth) => Box::new(MaxLength::new(max_lenth)),
            Component::Pattern { pattern, flags } => Box::new(Pattern::new(pattern, flags)),
            Component::UniqueLang(lang) => Box::new(UniqueLang::new(lang)),
            Component::LanguageIn { langs } => Box::new(LanguageIn::new(langs)),
            Component::Equals(iri_ref) => Box::new(Equals::new(iri_ref)),
            Component::Disjoint(iri_ref) => Box::new(Disjoint::new(iri_ref)),
            Component::LessThan(iri_ref) => Box::new(LessThan::new(iri_ref)),
            Component::LessThanOrEquals(iri) => Box::new(LessThanOrEquals::new(iri)),
            Component::Or { shapes } => Box::new(Or::new(shapes)),
            Component::And { shapes } => Box::new(And::new(shapes)),
            Component::Not { shape } => Box::new(Not::new(shape)),
            Component::Xone { shapes } => Box::new(Xone::new(shapes)),
            Component::Closed {
                is_closed,
                ignored_properties,
            } => Box::new(Closed::new(is_closed, ignored_properties)),
            Component::Node { shape } => Box::new(Node::new(shape)),
            Component::HasValue { value } => Box::new(HasValue::new(value)),
            Component::In { values } => Box::new(In::new(values)),
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => Box::new(QualifiedValue::new(
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            )),
        }
    }
}

impl<S: QuerySRDF + 'static> From<&Component> for Box<dyn SparqlConstraintComponent<S>> {
    fn from(value: &Component) -> Self {
        match value.to_owned() {
            Component::Class(node) => Box::new(Class::new(node)),
            Component::Datatype(iri_ref) => Box::new(Datatype::new(iri_ref)),
            Component::NodeKind(node_kind) => Box::new(Nodekind::new(node_kind)),
            Component::MinCount(min_count) => Box::new(MinCount::new(min_count)),
            Component::MaxCount(max_count) => Box::new(MaxCount::new(max_count)),
            Component::MinExclusive(literal) => Box::new(MinExclusive::new(literal)),
            Component::MaxExclusive(literal) => Box::new(MaxExclusive::new(literal)),
            Component::MinInclusive(literal) => Box::new(MinInclusive::new(literal)),
            Component::MaxInclusive(literal) => Box::new(MaxInclusive::new(literal)),
            Component::MinLength(min_length) => Box::new(MinLength::new(min_length)),
            Component::MaxLength(max_lenth) => Box::new(MaxLength::new(max_lenth)),
            Component::Pattern { pattern, flags } => Box::new(Pattern::new(pattern, flags)),
            Component::UniqueLang(lang) => Box::new(UniqueLang::new(lang)),
            Component::LanguageIn { langs } => Box::new(LanguageIn::new(langs)),
            Component::Equals(iri_ref) => Box::new(Equals::new(iri_ref)),
            Component::Disjoint(iri_ref) => Box::new(Disjoint::new(iri_ref)),
            Component::LessThan(iri_ref) => Box::new(LessThan::new(iri_ref)),
            Component::LessThanOrEquals(iri) => Box::new(LessThanOrEquals::new(iri)),
            Component::Or { shapes } => Box::new(Or::new(shapes)),
            Component::And { shapes } => Box::new(And::new(shapes)),
            Component::Not { shape } => Box::new(Not::new(shape)),
            Component::Xone { shapes } => Box::new(Xone::new(shapes)),
            Component::Closed {
                is_closed,
                ignored_properties,
            } => Box::new(Closed::new(is_closed, ignored_properties)),
            Component::Node { shape } => Box::new(Node::new(shape)),
            Component::HasValue { value } => Box::new(HasValue::new(value)),
            Component::In { values } => Box::new(In::new(values)),
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => Box::new(QualifiedValue::new(
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            )),
        }
    }
}
