use super::pgs::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type IDENTIFIER = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> IDENTIFIER {
    token.value.into()
}
pub type NUMBER = String;
pub fn number(_ctx: &Ctx, token: Token) -> NUMBER {
    token.value.into()
}
pub type QUOTED_STRING = String;
pub fn quoted_string(_ctx: &Ctx, token: Token) -> QUOTED_STRING {
    token.value.into()
}
pub type Pgs = CreateType1;
pub fn pgs_create_type1(_ctx: &Ctx, create_type1: CreateType1) -> Pgs {
    create_type1
}
pub type CreateType1 = Vec<CreateType>;
pub fn create_type1_c1(
    _ctx: &Ctx,
    mut create_type1: CreateType1,
    create_type: CreateType,
) -> CreateType1 {
    create_type1.push(create_type);
    create_type1
}
pub fn create_type1_create_type(_ctx: &Ctx, create_type: CreateType) -> CreateType1 {
    vec![create_type]
}
#[derive(Debug, Clone)]
pub enum CreateType {
    CreateNodeType(CreateNodeType),
    CreateEdgeType(CreateEdgeType),
    CreateGraphType(CreateGraphType),
}
pub fn create_type_create_node_type(
    _ctx: &Ctx,
    create_node_type: CreateNodeType,
) -> CreateType {
    CreateType::CreateNodeType(create_node_type)
}
pub fn create_type_create_edge_type(
    _ctx: &Ctx,
    create_edge_type: CreateEdgeType,
) -> CreateType {
    CreateType::CreateEdgeType(create_edge_type)
}
pub fn create_type_create_graph_type(
    _ctx: &Ctx,
    create_graph_type: CreateGraphType,
) -> CreateType {
    CreateType::CreateGraphType(create_graph_type)
}
pub type CreateNodeType = NodeType;
pub fn create_node_type_node_type(_ctx: &Ctx, node_type: NodeType) -> CreateNodeType {
    node_type
}
pub type CreateEdgeType = EdgeType;
pub fn create_edge_type_edge_type(_ctx: &Ctx, edge_type: EdgeType) -> CreateEdgeType {
    edge_type
}
pub type CreateGraphType = GraphType;
pub fn create_graph_type_graph_type(
    _ctx: &Ctx,
    graph_type: GraphType,
) -> CreateGraphType {
    graph_type
}
#[derive(Debug, Clone)]
pub struct NodeType {
    pub type_name_opt: TypeNameOpt,
    pub label_property_spec: LabelPropertySpec,
}
pub fn node_type_c1(
    _ctx: &Ctx,
    type_name_opt: TypeNameOpt,
    label_property_spec: LabelPropertySpec,
) -> NodeType {
    NodeType {
        type_name_opt,
        label_property_spec,
    }
}
pub type TypeNameOpt = Option<TypeName>;
pub fn type_name_opt_type_name(_ctx: &Ctx, type_name: TypeName) -> TypeNameOpt {
    Some(type_name)
}
pub fn type_name_opt_empty(_ctx: &Ctx) -> TypeNameOpt {
    None
}
#[derive(Debug, Clone)]
pub struct EdgeType {
    pub source: EndpointType,
    pub type_name_opt: TypeNameOpt,
    pub label_property_spec: LabelPropertySpec,
    pub target: EndpointType,
}
pub fn edge_type_c1(
    _ctx: &Ctx,
    source: EndpointType,
    type_name_opt: TypeNameOpt,
    label_property_spec: LabelPropertySpec,
    target: EndpointType,
) -> EdgeType {
    EdgeType {
        source,
        type_name_opt,
        label_property_spec,
        target,
    }
}
#[derive(Debug, Clone)]
pub struct GraphType {
    pub type_name: TypeName,
    pub graph_type_mode_opt: GraphTypeModeOpt,
    pub graph_type_elements_opt: GraphTypeElementsOpt,
}
pub fn graph_type_c1(
    _ctx: &Ctx,
    type_name: TypeName,
    graph_type_mode_opt: GraphTypeModeOpt,
    graph_type_elements_opt: GraphTypeElementsOpt,
) -> GraphType {
    GraphType {
        type_name,
        graph_type_mode_opt,
        graph_type_elements_opt,
    }
}
pub type GraphTypeModeOpt = Option<GraphTypeMode>;
pub fn graph_type_mode_opt_graph_type_mode(
    _ctx: &Ctx,
    graph_type_mode: GraphTypeMode,
) -> GraphTypeModeOpt {
    Some(graph_type_mode)
}
pub fn graph_type_mode_opt_empty(_ctx: &Ctx) -> GraphTypeModeOpt {
    None
}
pub type GraphTypeElementsOpt = Option<GraphTypeElements>;
pub fn graph_type_elements_opt_graph_type_elements(
    _ctx: &Ctx,
    graph_type_elements: GraphTypeElements,
) -> GraphTypeElementsOpt {
    Some(graph_type_elements)
}
pub fn graph_type_elements_opt_empty(_ctx: &Ctx) -> GraphTypeElementsOpt {
    None
}
#[derive(Debug, Clone)]
pub enum GraphTypeMode {
    STRICT,
    LOOSE,
}
pub fn graph_type_mode_strict(_ctx: &Ctx) -> GraphTypeMode {
    GraphTypeMode::STRICT
}
pub fn graph_type_mode_loose(_ctx: &Ctx) -> GraphTypeMode {
    GraphTypeMode::LOOSE
}
#[derive(Debug, Clone)]
pub enum GraphTypeElements {
    TypeName(TypeName),
    NodeType(NodeType),
    EdgeType(EdgeType),
}
pub fn graph_type_elements_type_name(
    _ctx: &Ctx,
    type_name: TypeName,
) -> GraphTypeElements {
    GraphTypeElements::TypeName(type_name)
}
pub fn graph_type_elements_node_type(
    _ctx: &Ctx,
    node_type: NodeType,
) -> GraphTypeElements {
    GraphTypeElements::NodeType(node_type)
}
pub fn graph_type_elements_edge_type(
    _ctx: &Ctx,
    edge_type: EdgeType,
) -> GraphTypeElements {
    GraphTypeElements::EdgeType(edge_type)
}
pub type TypeName = IDENTIFIER;
pub fn type_name_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> TypeName {
    identifier
}
pub type EndpointType = LabelPropertySpec;
pub fn endpoint_type_label_property_spec(
    _ctx: &Ctx,
    label_property_spec: LabelPropertySpec,
) -> EndpointType {
    label_property_spec
}
#[derive(Debug, Clone)]
pub struct LabelPropertySpec {
    pub label_spec_opt: LabelSpecOpt,
    pub property_spec_opt: PropertySpecOpt,
}
pub fn label_property_spec_c1(
    _ctx: &Ctx,
    label_spec_opt: LabelSpecOpt,
    property_spec_opt: PropertySpecOpt,
) -> LabelPropertySpec {
    LabelPropertySpec {
        label_spec_opt,
        property_spec_opt,
    }
}
pub type LabelSpecOpt = Option<LabelSpec>;
pub fn label_spec_opt_label_spec(_ctx: &Ctx, label_spec: LabelSpec) -> LabelSpecOpt {
    Some(label_spec)
}
pub fn label_spec_opt_empty(_ctx: &Ctx) -> LabelSpecOpt {
    None
}
pub type PropertySpecOpt = Option<PropertySpec>;
pub fn property_spec_opt_property_spec(
    _ctx: &Ctx,
    property_spec: PropertySpec,
) -> PropertySpecOpt {
    Some(property_spec)
}
pub fn property_spec_opt_empty(_ctx: &Ctx) -> PropertySpecOpt {
    None
}
pub type LabelSpec = Labels;
pub fn label_spec_labels(_ctx: &Ctx, labels: Labels) -> LabelSpec {
    labels
}
#[derive(Debug, Clone)]
pub struct Labels {
    pub single_label: SingleLabel,
    pub more_labels_opt: MoreLabelsOpt,
}
pub fn labels_c1(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> Labels {
    Labels {
        single_label,
        more_labels_opt,
    }
}
pub type MoreLabelsOpt = Option<MoreLabels>;
pub fn more_labels_opt_more_labels(
    _ctx: &Ctx,
    more_labels: MoreLabels,
) -> MoreLabelsOpt {
    Some(more_labels)
}
pub fn more_labels_opt_empty(_ctx: &Ctx) -> MoreLabelsOpt {
    None
}
#[derive(Debug, Clone)]
pub struct AndLabels {
    pub single_label: SingleLabel,
    pub more_labels_opt: Box<MoreLabelsOpt>,
}
#[derive(Debug, Clone)]
pub struct OrLabels {
    pub single_label: SingleLabel,
    pub more_labels_opt: Box<MoreLabelsOpt>,
}
#[derive(Debug, Clone)]
pub enum MoreLabels {
    AndLabels(AndLabels),
    OrLabels(OrLabels),
}
pub fn more_labels_and_labels(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> MoreLabels {
    MoreLabels::AndLabels(AndLabels {
        single_label,
        more_labels_opt: Box::new(more_labels_opt),
    })
}
pub fn more_labels_or_labels(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> MoreLabels {
    MoreLabels::OrLabels(OrLabels {
        single_label,
        more_labels_opt: Box::new(more_labels_opt),
    })
}
#[derive(Debug, Clone)]
pub enum SingleLabel {
    SingleLabel(IDENTIFIER),
    TypeName(IDENTIFIER),
}
pub fn single_label_single_label(_ctx: &Ctx, identifier: IDENTIFIER) -> SingleLabel {
    SingleLabel::SingleLabel(identifier)
}
pub fn single_label_type_name(_ctx: &Ctx, identifier: IDENTIFIER) -> SingleLabel {
    SingleLabel::TypeName(identifier)
}
pub type PropertySpec = Properties;
pub fn property_spec_properties(_ctx: &Ctx, properties: Properties) -> PropertySpec {
    properties
}
#[derive(Debug, Clone)]
pub struct EachOf {
    pub left: Box<Properties>,
    pub right: Box<Properties>,
}
#[derive(Debug, Clone)]
pub struct OneOf {
    pub left: Box<Properties>,
    pub right: Box<Properties>,
}
#[derive(Debug, Clone)]
pub struct BaseProperty {
    pub optionalopt: OPTIONALOpt,
    pub property: Property,
}
#[derive(Debug, Clone)]
pub enum Properties {
    EachOf(EachOf),
    OneOf(OneOf),
    Paren(Box<Properties>),
    BaseProperty(BaseProperty),
}
pub fn properties_each_of(
    _ctx: &Ctx,
    left: Properties,
    right: Properties,
) -> Properties {
    Properties::EachOf(EachOf {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn properties_one_of(_ctx: &Ctx, left: Properties, right: Properties) -> Properties {
    Properties::OneOf(OneOf {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn properties_paren(_ctx: &Ctx, properties: Properties) -> Properties {
    Properties::Paren(Box::new(properties))
}
pub fn properties_base_property(
    _ctx: &Ctx,
    optionalopt: OPTIONALOpt,
    property: Property,
) -> Properties {
    Properties::BaseProperty(BaseProperty {
        optionalopt,
        property,
    })
}
pub type OPTIONALOpt = Option<OptionalOptNoO>;
#[derive(Debug, Clone)]
pub enum OptionalOptNoO {
    OPTIONAL,
}
pub fn optionalopt_optional(_ctx: &Ctx) -> OPTIONALOpt {
    Some(OptionalOptNoO::OPTIONAL)
}
pub fn optionalopt_empty(_ctx: &Ctx) -> OPTIONALOpt {
    None
}
#[derive(Debug, Clone)]
pub struct Property {
    pub key: key,
    pub type_spec: TypeSpec,
}
pub fn property_c1(_ctx: &Ctx, key: key, type_spec: TypeSpec) -> Property {
    Property { key, type_spec }
}
pub type key = IDENTIFIER;
pub fn key_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> key {
    identifier
}
#[derive(Debug, Clone)]
pub struct TypeSpec {
    pub simple_type: SimpleType,
    pub more_types_opt: MoreTypesOpt,
}
pub fn type_spec_c1(
    _ctx: &Ctx,
    simple_type: SimpleType,
    more_types_opt: MoreTypesOpt,
) -> TypeSpec {
    TypeSpec {
        simple_type,
        more_types_opt,
    }
}
pub type MoreTypesOpt = Option<MoreTypes>;
pub fn more_types_opt_more_types(_ctx: &Ctx, more_types: MoreTypes) -> MoreTypesOpt {
    Some(more_types)
}
pub fn more_types_opt_empty(_ctx: &Ctx) -> MoreTypesOpt {
    None
}
#[derive(Debug, Clone)]
pub struct IntersectionType {
    pub simple_type: SimpleType,
    pub more_types_opt: Box<MoreTypesOpt>,
}
#[derive(Debug, Clone)]
pub struct UnionType {
    pub simple_type: SimpleType,
    pub more_types_opt: Box<MoreTypesOpt>,
}
#[derive(Debug, Clone)]
pub enum MoreTypes {
    IntersectionType(IntersectionType),
    UnionType(UnionType),
}
pub fn more_types_intersection_type(
    _ctx: &Ctx,
    simple_type: SimpleType,
    more_types_opt: MoreTypesOpt,
) -> MoreTypes {
    MoreTypes::IntersectionType(IntersectionType {
        simple_type,
        more_types_opt: Box::new(more_types_opt),
    })
}
pub fn more_types_union_type(
    _ctx: &Ctx,
    simple_type: SimpleType,
    more_types_opt: MoreTypesOpt,
) -> MoreTypes {
    MoreTypes::UnionType(UnionType {
        simple_type,
        more_types_opt: Box::new(more_types_opt),
    })
}
#[derive(Debug, Clone)]
pub struct StringSpec {
    pub card_opt: CardOpt,
    pub check_opt: CheckOpt,
}
#[derive(Debug, Clone)]
pub struct Integer {
    pub card_opt: CardOpt,
    pub check_opt: CheckOpt,
}
#[derive(Debug, Clone)]
pub struct Date {
    pub card_opt: CardOpt,
    pub check_opt: CheckOpt,
}
#[derive(Debug, Clone)]
pub struct Bool {
    pub card_opt: CardOpt,
    pub check_opt: CheckOpt,
}
#[derive(Debug, Clone)]
pub enum SimpleType {
    StringSpec(StringSpec),
    Integer(Integer),
    Date(Date),
    Bool(Bool),
    Any(CheckOpt),
    Cond(Cond),
}
pub fn simple_type_string_spec(
    _ctx: &Ctx,
    card_opt: CardOpt,
    check_opt: CheckOpt,
) -> SimpleType {
    SimpleType::StringSpec(StringSpec { card_opt, check_opt })
}
pub fn simple_type_integer(
    _ctx: &Ctx,
    card_opt: CardOpt,
    check_opt: CheckOpt,
) -> SimpleType {
    SimpleType::Integer(Integer { card_opt, check_opt })
}
pub fn simple_type_date(
    _ctx: &Ctx,
    card_opt: CardOpt,
    check_opt: CheckOpt,
) -> SimpleType {
    SimpleType::Date(Date { card_opt, check_opt })
}
pub fn simple_type_bool(
    _ctx: &Ctx,
    card_opt: CardOpt,
    check_opt: CheckOpt,
) -> SimpleType {
    SimpleType::Bool(Bool { card_opt, check_opt })
}
pub fn simple_type_any(_ctx: &Ctx, check_opt: CheckOpt) -> SimpleType {
    SimpleType::Any(check_opt)
}
pub fn simple_type_cond(_ctx: &Ctx, cond: Cond) -> SimpleType {
    SimpleType::Cond(cond)
}
pub type CardOpt = Option<Card>;
pub fn card_opt_card(_ctx: &Ctx, card: Card) -> CardOpt {
    Some(card)
}
pub fn card_opt_empty(_ctx: &Ctx) -> CardOpt {
    None
}
pub type CheckOpt = Option<Check>;
pub fn check_opt_check(_ctx: &Ctx, check: Check) -> CheckOpt {
    Some(check)
}
pub fn check_opt_empty(_ctx: &Ctx) -> CheckOpt {
    None
}
pub type Check = Cond;
pub fn check_cond(_ctx: &Ctx, cond: Cond) -> Check {
    cond
}
#[derive(Debug, Clone)]
pub struct And {
    pub left: Box<Cond>,
    pub right: Box<Cond>,
}
#[derive(Debug, Clone)]
pub struct OR {
    pub left: Box<Cond>,
    pub right: Box<Cond>,
}
#[derive(Debug, Clone)]
pub enum Cond {
    TRUE,
    FALSE,
    GT(SingleValue),
    GE(SingleValue),
    LT(SingleValue),
    LE(SingleValue),
    EQ(SingleValue),
    Regex(QUOTED_STRING),
    And(And),
    OR(OR),
    Not(Box<Cond>),
    ParenCond(Box<Cond>),
}
pub fn cond_true(_ctx: &Ctx) -> Cond {
    Cond::TRUE
}
pub fn cond_false(_ctx: &Ctx) -> Cond {
    Cond::FALSE
}
pub fn cond_gt(_ctx: &Ctx, single_value: SingleValue) -> Cond {
    Cond::GT(single_value)
}
pub fn cond_ge(_ctx: &Ctx, single_value: SingleValue) -> Cond {
    Cond::GE(single_value)
}
pub fn cond_lt(_ctx: &Ctx, single_value: SingleValue) -> Cond {
    Cond::LT(single_value)
}
pub fn cond_le(_ctx: &Ctx, single_value: SingleValue) -> Cond {
    Cond::LE(single_value)
}
pub fn cond_eq(_ctx: &Ctx, single_value: SingleValue) -> Cond {
    Cond::EQ(single_value)
}
pub fn cond_regex(_ctx: &Ctx, quoted_string: QUOTED_STRING) -> Cond {
    Cond::Regex(quoted_string)
}
pub fn cond_and(_ctx: &Ctx, left: Cond, right: Cond) -> Cond {
    Cond::And(And {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn cond_or(_ctx: &Ctx, left: Cond, right: Cond) -> Cond {
    Cond::OR(OR {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn cond_not(_ctx: &Ctx, cond: Cond) -> Cond {
    Cond::Not(Box::new(cond))
}
pub fn cond_paren_cond(_ctx: &Ctx, cond: Cond) -> Cond {
    Cond::ParenCond(Box::new(cond))
}
#[derive(Debug, Clone)]
pub struct Range {
    pub number: NUMBER,
    pub max: Max,
}
#[derive(Debug, Clone)]
pub enum Card {
    Optional,
    OneOrMore,
    ZeroOrMore,
    Range(Range),
}
pub fn card_optional(_ctx: &Ctx) -> Card {
    Card::Optional
}
pub fn card_one_or_more(_ctx: &Ctx) -> Card {
    Card::OneOrMore
}
pub fn card_zero_or_more(_ctx: &Ctx) -> Card {
    Card::ZeroOrMore
}
pub fn card_range(_ctx: &Ctx, number: NUMBER, max: Max) -> Card {
    Card::Range(Range { number, max })
}
#[derive(Debug, Clone)]
pub enum Max {
    NUMBER(NUMBER),
    Star,
}
pub fn max_number(_ctx: &Ctx, number: NUMBER) -> Max {
    Max::NUMBER(number)
}
pub fn max_star(_ctx: &Ctx) -> Max {
    Max::Star
}
#[derive(Debug, Clone)]
pub enum SingleValue {
    StringValue(QUOTED_STRING),
    NumberValue(NUMBER),
    DateValue(QUOTED_STRING),
    BooleanValue(BOOL),
}
pub fn single_value_string_value(
    _ctx: &Ctx,
    quoted_string: QUOTED_STRING,
) -> SingleValue {
    SingleValue::StringValue(quoted_string)
}
pub fn single_value_number_value(_ctx: &Ctx, number: NUMBER) -> SingleValue {
    SingleValue::NumberValue(number)
}
pub fn single_value_date_value(_ctx: &Ctx, quoted_string: QUOTED_STRING) -> SingleValue {
    SingleValue::DateValue(quoted_string)
}
pub fn single_value_boolean_value(_ctx: &Ctx, bool: BOOL) -> SingleValue {
    SingleValue::BooleanValue(bool)
}
#[derive(Debug, Clone)]
pub enum BOOL {
    TRUE,
    FALSE,
}
pub fn bool_true(_ctx: &Ctx) -> BOOL {
    BOOL::TRUE
}
pub fn bool_false(_ctx: &Ctx) -> BOOL {
    BOOL::FALSE
}
