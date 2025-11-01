use super::pg::{Context, TokenKind};
/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type QUOTED_STRING = String;
pub fn quoted_string(_ctx: &Ctx, token: Token) -> QUOTED_STRING {
    token.value.into()
}
pub type IDENTIFIER = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> IDENTIFIER {
    token.value.into()
}
pub type NUMBER = String;
pub fn number(_ctx: &Ctx, token: Token) -> NUMBER {
    token.value.into()
}
pub type Pg = Declarations;
pub fn pg_declarations(_ctx: &Ctx, declarations: Declarations) -> Pg {
    declarations
}
pub type Declarations = Declaration1;
pub fn declarations_declaration1(
    _ctx: &Ctx,
    declaration1: Declaration1,
) -> Declarations {
    declaration1
}
pub type Declaration1 = Vec<Declaration>;
pub fn declaration1_c1(
    _ctx: &Ctx,
    mut declaration1: Declaration1,
    declaration: Declaration,
) -> Declaration1 {
    declaration1.push(declaration);
    declaration1
}
pub fn declaration1_declaration(_ctx: &Ctx, declaration: Declaration) -> Declaration1 {
    vec![declaration]
}
#[derive(Debug, Clone)]
pub enum Declaration {
    Node(Node),
    Edge(Edge),
}
pub fn declaration_node(_ctx: &Ctx, node: Node) -> Declaration {
    Declaration::Node(node)
}
pub fn declaration_edge(_ctx: &Ctx, edge: Edge) -> Declaration {
    Declaration::Edge(edge)
}
#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub labels_record: LabelsRecord,
}
pub fn node_c1(_ctx: &Ctx, id: Id, labels_record: LabelsRecord) -> Node {
    Node { id, labels_record }
}
#[derive(Debug, Clone)]
pub struct Edge {
    pub id: Id,
    pub source: IDENTIFIER,
    pub labels_record: LabelsRecord,
    pub target: IDENTIFIER,
}
pub fn edge_c1(
    _ctx: &Ctx,
    id: Id,
    source: IDENTIFIER,
    labels_record: LabelsRecord,
    target: IDENTIFIER,
) -> Edge {
    Edge {
        id,
        source,
        labels_record,
        target,
    }
}
pub type Id = IDENTIFIER;
pub fn id_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> Id {
    identifier
}
#[derive(Debug, Clone)]
pub struct LabelsRecord {
    pub labels_opt: LabelsOpt,
    pub record_opt: RecordOpt,
}
pub fn labels_record_c1(
    _ctx: &Ctx,
    labels_opt: LabelsOpt,
    record_opt: RecordOpt,
) -> LabelsRecord {
    LabelsRecord {
        labels_opt,
        record_opt,
    }
}
pub type LabelsOpt = Option<Labels>;
pub fn labels_opt_labels(_ctx: &Ctx, labels: Labels) -> LabelsOpt {
    Some(labels)
}
pub fn labels_opt_empty(_ctx: &Ctx) -> LabelsOpt {
    None
}
pub type RecordOpt = Option<Record>;
pub fn record_opt_record(_ctx: &Ctx, record: Record) -> RecordOpt {
    Some(record)
}
pub fn record_opt_empty(_ctx: &Ctx) -> RecordOpt {
    None
}
pub type Labels = IDENTIFIER1;
pub fn labels_identifier1(_ctx: &Ctx, identifier1: IDENTIFIER1) -> Labels {
    identifier1
}
pub type IDENTIFIER1 = Vec<IDENTIFIER>;
pub fn identifier1_c1(
    _ctx: &Ctx,
    mut identifier1: IDENTIFIER1,
    identifier: IDENTIFIER,
) -> IDENTIFIER1 {
    identifier1.push(identifier);
    identifier1
}
pub fn identifier1_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> IDENTIFIER1 {
    vec![identifier]
}
pub type Record = Properties;
pub fn record_properties(_ctx: &Ctx, properties: Properties) -> Record {
    properties
}
pub type Properties = Property1;
pub fn properties_property1(_ctx: &Ctx, property1: Property1) -> Properties {
    property1
}
pub type Property1 = Vec<Property>;
pub fn property1_c1(
    _ctx: &Ctx,
    mut property1: Property1,
    property: Property,
) -> Property1 {
    property1.push(property);
    property1
}
pub fn property1_property(_ctx: &Ctx, property: Property) -> Property1 {
    vec![property]
}
#[derive(Debug, Clone)]
pub struct Property {
    pub key: key,
    pub values: Values,
}
pub fn property_c1(_ctx: &Ctx, key: key, values: Values) -> Property {
    Property { key, values }
}
pub type key = IDENTIFIER;
pub fn key_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> key {
    identifier
}
#[derive(Debug, Clone)]
pub enum Values {
    SingleValue(SingleValue),
    ListValue(ListValues),
}
pub fn values_single_value(_ctx: &Ctx, single_value: SingleValue) -> Values {
    Values::SingleValue(single_value)
}
pub fn values_list_value(_ctx: &Ctx, list_values: ListValues) -> Values {
    Values::ListValue(list_values)
}
pub type ListValues = SingleValue0;
pub fn list_values_single_value0(_ctx: &Ctx, single_value0: SingleValue0) -> ListValues {
    single_value0
}
pub type SingleValue1 = Vec<SingleValue>;
pub fn single_value1_c1(
    _ctx: &Ctx,
    mut single_value1: SingleValue1,
    single_value: SingleValue,
) -> SingleValue1 {
    single_value1.push(single_value);
    single_value1
}
pub fn single_value1_single_value(
    _ctx: &Ctx,
    single_value: SingleValue,
) -> SingleValue1 {
    vec![single_value]
}
pub type SingleValue0 = Option<SingleValue1>;
pub fn single_value0_single_value1(
    _ctx: &Ctx,
    single_value1: SingleValue1,
) -> SingleValue0 {
    Some(single_value1)
}
pub fn single_value0_empty(_ctx: &Ctx) -> SingleValue0 {
    None
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
