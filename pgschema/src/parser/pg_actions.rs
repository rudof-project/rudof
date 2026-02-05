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
pub type Pg = Statements;
pub fn pg_statements(_ctx: &Ctx, statements: Statements) -> Pg {
    statements
}
pub type Statements = Statement1;
pub fn statements_statement1(_ctx: &Ctx, statement1: Statement1) -> Statements {
    statement1
}
pub type Statements1 = Vec<Statement>;
pub fn statement1_c1(
    _ctx: &Ctx,
    mut statement1: Statement1,
    statement: Statement,
) -> Statement1 {
    statement1.push(statement);
    statement1
}
#[derive(Debug, Clone)]
pub enum Statement {
    Node(Node),
    Edge(Edge),
}
pub fn declaration_node(_ctx: &Ctx, node: Node) -> Statement {
    Statement::Node(node)
}
pub fn declaration_edge(_ctx: &Ctx, edge: Edge) -> Statement {
    Statement::Edge(edge)
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
    pub source: IDENTIFIER,
    pub id: Option<Id>,
    pub labels_record: LabelsRecord,
    pub target: IDENTIFIER,
}
pub fn edge_c1(
    _ctx: &Ctx,
    source: IDENTIFIER,
    id: Option<Id>,
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
pub type IDENTIFIER1 = Vec<identifier>;
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
pub type key = identifier;
pub fn key_identifier(_ctx: &Ctx, identifier: identifier) -> key {
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
pub fn statement1_statement(_ctx: &Ctx, statement: Statement) -> Statement1 {
    vec![statement]
}
pub type Statement1 = Vec<Statement>;
pub type NodeId = Id;
pub fn node_id_id(_ctx: &Ctx, id: Id) -> NodeId {
    id
}
pub type EdgeId = Id;
pub fn edge_id_id(_ctx: &Ctx, id: Id) -> EdgeId {
    id
}
#[derive(Debug, Clone)]
pub enum identifier {
    IDENTIFIER(IDENTIFIER),
    QUOTED_STRING(QUOTED_STRING),
}
impl identifier {
    pub fn as_str(&self) -> &str {
        match self {
            identifier::IDENTIFIER(s) => s.as_str(),
            identifier::QUOTED_STRING(s) => {
                s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(s)
            }
        }
    }
}
pub fn identifier_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> identifier {
    identifier::IDENTIFIER(identifier)
}
pub fn identifier_quoted_string(_ctx: &Ctx, quoted_string: QUOTED_STRING) -> identifier {
    identifier::QUOTED_STRING(quoted_string)
}
pub fn statement_node(_ctx: &Ctx, node: Node) -> Statement {
    Statement::Node(node)
}
pub fn statement_edge(_ctx: &Ctx, edge: Edge) -> Statement {
    Statement::Edge(edge)
}
pub type identifier1 = Vec<identifier>;
pub fn identifier1_identifier(_ctx: &Ctx, identifier: identifier) -> identifier1 {
    vec![identifier]
}
pub fn labels_identifier1(_ctx: &Ctx, identifier1: identifier1) -> Labels {
    identifier1
}
pub type EdgeIdOpt = Option<EdgeId>;
pub fn edge_id_opt_edge_id(_ctx: &Ctx, edge_id: EdgeId) -> EdgeIdOpt {
    Some(edge_id)
}
pub fn edge_id_opt_empty(_ctx: &Ctx) -> EdgeIdOpt {
    None
}
pub type Labels = identifier1;
#[derive(Debug, Clone)]
pub enum Conj {
    AMPERSAND,
    COMMA,
}
pub fn conj_ampersand(_ctx: &Ctx) -> Conj {
    Conj::AMPERSAND
}
pub fn conj_comma(_ctx: &Ctx) -> Conj {
    Conj::COMMA
}
pub fn identifier1_c1(
    _ctx: &Ctx,
    mut identifier1: identifier1,
    identifier: identifier,
) -> identifier1 {
    identifier1.push(identifier);
    identifier1
}
