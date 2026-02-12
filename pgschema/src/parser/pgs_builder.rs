use super::pgs_actions::CreateType;
use rustemo::Parser;

use crate::{
    boolean_expr::BooleanExpr,
    card::{Card as PGCard, Max as PGMax},
    key::Key,
    label_property_spec::LabelPropertySpec as PGLabelPropertySpec,
    parser::{
        pgs::PgsParser,
        pgs_actions::{
            BaseProperty, Card, Cond, LabelPropertySpec, Labels, Max, MoreLabels, MoreTypes, Properties, Property,
            PropertySpec, Range, SimpleType, SingleLabel, SingleValue, TypeSpec,
        },
    },
    pgs::PropertyGraphSchema,
    pgs_error::PgsError,
    property_value_spec::{
        PropertyValue as PGPropertyValue, PropertyValueSpec as PGPropertyValueSpec, TypeSpec as PGTypeSpec,
    },
    value::Value,
    value_type::ValueType,
};

pub struct PgsBuilder {}

impl Default for PgsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PgsBuilder {
    pub fn new() -> Self {
        PgsBuilder {}
    }
    pub fn parse_pgs(&self, input: &str) -> Result<PropertyGraphSchema, PgsError> {
        let pgs_content = PgsParser::new()
            .parse(input)
            .map_err(|e| PgsError::ParserError { error: e.to_string() })?;
        let mut schema = PropertyGraphSchema::new();
        get_create_types(pgs_content, &mut schema)?;
        Ok(schema)
    }
}

fn get_create_types(create_types: Vec<CreateType>, schema: &mut PropertyGraphSchema) -> Result<(), PgsError> {
    for create_type in create_types {
        match create_type {
            CreateType::CreateNodeType(node_type) => {
                let label_property_spec = get_label_property_spec(node_type.label_property_spec)?;
                if let Some(type_name) = node_type.type_name_opt {
                    let _ = schema.add_node_spec(type_name.as_str(), label_property_spec)?;
                } else {
                    let _ = schema.add_blank_node_spec(label_property_spec)?;
                }
            },
            CreateType::CreateEdgeType(edge_type) => {
                let source_spec = get_label_property_spec(edge_type.source)?;
                let target_spec = get_label_property_spec(edge_type.target)?;
                let label_property_spec = get_label_property_spec(edge_type.label_property_spec)?;
                if let Some(type_name) = edge_type.type_name_opt {
                    let _ = schema.add_edge_spec(type_name.as_str(), source_spec, target_spec, label_property_spec)?;
                } else {
                    let _ = schema.add_blank_edge_spec(source_spec, label_property_spec, target_spec);
                }
            },
            CreateType::CreateGraphType(_) => todo!(),
        }
    }
    Ok(())
}

fn get_label_property_spec(label_property_spec: LabelPropertySpec) -> Result<PGLabelPropertySpec, PgsError> {
    if let Some(label_spec) = label_property_spec.label_spec_opt {
        let label_spec = get_labels(label_spec)?;
        if let Some(property_spec) = label_property_spec.property_spec_opt {
            let property_value_spec = get_property_value_spec(property_spec)?;
            Ok(PGLabelPropertySpec::content(
                label_spec,
                PGPropertyValueSpec::closed(property_value_spec),
            ))
        } else {
            Ok(label_spec)
        }
    } else {
        Ok(PGLabelPropertySpec::new())
    }
}

fn get_labels(labels: Labels) -> Result<PGLabelPropertySpec, PgsError> {
    let mut label_spec = get_single_label(labels.single_label)?;
    if let Some(more_labels) = labels.more_labels_opt {
        get_more_labels(more_labels, &mut label_spec)?;
    }
    Ok(label_spec)
}

fn get_single_label(single_label: SingleLabel) -> Result<PGLabelPropertySpec, PgsError> {
    match single_label {
        SingleLabel::SingleLabel(identifier) => Ok(PGLabelPropertySpec::label(identifier)),
        SingleLabel::TypeName(type_name) => Ok(PGLabelPropertySpec::ref_(type_name)),
    }
}

fn get_more_labels(more_labels: MoreLabels, label_spec: &mut PGLabelPropertySpec) -> Result<(), PgsError> {
    match more_labels {
        MoreLabels::AndLabels(and_labels) => {
            let label_spec2 = get_single_label(and_labels.single_label)?;
            *label_spec = PGLabelPropertySpec::and(label_spec.clone(), label_spec2);
            if let Some(more_labels) = *and_labels.more_labels_opt {
                get_more_labels(more_labels, label_spec)?;
                Ok(())
            } else {
                Ok(())
            }
        },
        MoreLabels::OrLabels(or_labels) => {
            let label_spec2 = get_single_label(or_labels.single_label)?;
            *label_spec = PGLabelPropertySpec::or(label_spec.clone(), label_spec2);
            if let Some(more_labels) = *or_labels.more_labels_opt {
                get_more_labels(more_labels, label_spec)?;
                Ok(())
            } else {
                Ok(())
            }
        },
    }
}

fn get_property_value_spec(property_value_spec: Properties) -> Result<PGPropertyValue, PgsError> {
    get_property_value(property_value_spec)
}

fn get_property_value(property_value: Properties) -> Result<PGPropertyValue, PgsError> {
    match property_value {
        Properties::Paren(properties) => get_property_value_spec(*properties),
        PropertySpec::EachOf(each_of) => {
            let left = get_property_value_spec(*each_of.left)?;
            let right = get_property_value_spec(*each_of.right)?;
            Ok(PGPropertyValue::each_of(left, right))
        },
        PropertySpec::OneOf(one_of) => {
            let left = get_property_value_spec(*one_of.left)?;
            let right = get_property_value_spec(*one_of.right)?;
            Ok(PGPropertyValue::one_of(left, right))
        },
        PropertySpec::BaseProperty(property) => get_base_property(property),
    }
}

fn get_base_property(base_property: BaseProperty) -> Result<PGPropertyValue, PgsError> {
    let (key, type_spec) = get_property(base_property.property)?;
    if base_property.optionalopt.is_some() {
        Ok(PGPropertyValue::optional_property(key, type_spec))
    } else {
        Ok(PGPropertyValue::property(key, type_spec))
    }
}

fn get_property(property: Property) -> Result<(Key, PGTypeSpec), PgsError> {
    let key = property.key;
    let type_spec = get_type_spec(property.type_spec)?;
    Ok((Key::new(key.as_str()), type_spec))
}

fn get_card(card: Card) -> Result<PGCard, PgsError> {
    match card {
        Card::ZeroOrMore => Ok(PGCard::ZeroOrMore),
        Card::OneOrMore => Ok(PGCard::OneOrMore),
        Card::Range(range) => get_range(range),
        Card::Optional => Ok(PGCard::ZeroOrOne),
    }
}

fn get_range(range: Range) -> Result<PGCard, PgsError> {
    let min = get_number(range.number)?;
    let max = get_max(range.max)?;
    Ok(PGCard::range(min, max))
}

fn get_max(max: Max) -> Result<PGMax, PgsError> {
    match max {
        Max::NUMBER(n) => {
            let n = get_number(n)?;
            Ok(PGMax::Bounded(n))
        },
        Max::Star => Ok(PGMax::Unbounded),
    }
}

fn get_number(number: String) -> Result<usize, PgsError> {
    number.parse::<usize>().map_err(|_| PgsError::InvalidNumber(number))
}

fn get_type_spec(type_spec: TypeSpec) -> Result<PGTypeSpec, PgsError> {
    let simple_type = get_simple_type(type_spec.simple_type)?;
    if let Some(more_types) = type_spec.more_types_opt {
        get_more_types(more_types, simple_type)
    } else {
        Ok(simple_type)
    }
}

fn get_more_types(more_types: MoreTypes, current: PGTypeSpec) -> Result<PGTypeSpec, PgsError> {
    match more_types {
        MoreTypes::UnionType(union_type) => {
            let right = get_simple_type(union_type.simple_type)?;
            Ok(PGTypeSpec::union(current, right))
        },
        MoreTypes::IntersectionType(intersection_type) => {
            let right = get_simple_type(intersection_type.simple_type)?;
            Ok(PGTypeSpec::intersection(current, right))
        },
    }
}

fn get_simple_type(simple_type: SimpleType) -> Result<PGTypeSpec, PgsError> {
    match simple_type {
        SimpleType::StringSpec(str) => {
            let card = get_card_opt(str.card_opt)?;
            if let Some(cond) = str.check_opt {
                let cond = get_cond(cond)?;
                Ok(PGTypeSpec::cond(ValueType::string(card), cond))
            } else {
                Ok(PGTypeSpec::string(card))
            }
        },
        SimpleType::Integer(integer) => {
            let card = get_card_opt(integer.card_opt)?;
            if let Some(cond) = integer.check_opt {
                let cond = get_cond(cond)?;
                Ok(PGTypeSpec::cond(ValueType::integer(card), cond))
            } else {
                Ok(PGTypeSpec::integer(card))
            }
        },
        SimpleType::Date(date) => {
            let card = get_card_opt(date.card_opt)?;
            if let Some(cond) = date.check_opt {
                let cond = get_cond(cond)?;
                Ok(PGTypeSpec::cond(ValueType::date(card), cond))
            } else {
                Ok(PGTypeSpec::date(card))
            }
        },
        SimpleType::Any(cond) => {
            if let Some(cond) = cond {
                let cond = get_cond(cond)?;
                Ok(PGTypeSpec::cond(ValueType::Any, cond))
            } else {
                Ok(PGTypeSpec::any())
            }
        },
        SimpleType::Cond(cond) => {
            let cond = get_cond(cond)?;
            Ok(PGTypeSpec::cond(ValueType::Any, cond))
        },
        SimpleType::Bool(bool) => {
            let card = get_card_opt(bool.card_opt)?;
            if let Some(cond) = bool.check_opt {
                let cond = get_cond(cond)?;
                Ok(PGTypeSpec::cond(ValueType::bool(card), cond))
            } else {
                Ok(PGTypeSpec::bool(card))
            }
        },
    }
}

fn get_card_opt(card_opt: Option<Card>) -> Result<PGCard, PgsError> {
    if let Some(card) = card_opt {
        get_card(card)
    } else {
        Ok(PGCard::One)
    }
}

fn get_cond(cond: Cond) -> Result<BooleanExpr, PgsError> {
    match cond {
        Cond::TRUE => Ok(BooleanExpr::True),
        Cond::FALSE => Ok(BooleanExpr::False),
        Cond::And(and) => {
            let left = get_cond(*and.left)?;
            let right = get_cond(*and.right)?;
            Ok(BooleanExpr::And(Box::new(left), Box::new(right)))
        },
        Cond::OR(or) => {
            let left = get_cond(*or.left)?;
            let right = get_cond(*or.right)?;
            Ok(BooleanExpr::Or(Box::new(left), Box::new(right)))
        },
        Cond::Not(cond) => {
            let expr = get_cond(*cond)?;
            Ok(BooleanExpr::Not(Box::new(expr)))
        },
        Cond::ParenCond(cond) => {
            let expr = get_cond(*cond)?;
            Ok(expr)
        },
        Cond::GT(single_value) => {
            let value = get_value(single_value)?;
            Ok(BooleanExpr::GreaterThan(value))
        },
        Cond::GE(single_value) => {
            let value = get_value(single_value)?;
            Ok(BooleanExpr::GreaterThanOrEqual(value))
        },
        Cond::LT(single_value) => {
            let value = get_value(single_value)?;
            Ok(BooleanExpr::LessThan(value))
        },
        Cond::LE(single_value) => {
            let value = get_value(single_value)?;
            Ok(BooleanExpr::LessThanOrEqual(value))
        },
        Cond::EQ(single_value) => {
            let value = get_value(single_value)?;
            Ok(BooleanExpr::Equals(value))
        },
        Cond::Regex(pattern) => {
            let cleaned = remove_quotes(pattern.as_str());
            Ok(BooleanExpr::Regex(cleaned.to_string()))
        },
    }
}

fn get_value(value: SingleValue) -> Result<Value, PgsError> {
    match value {
        SingleValue::StringValue(s) => {
            let cleaned = remove_quotes(s.as_str());
            Ok(Value::str(cleaned))
        },
        SingleValue::NumberValue(str_number_) => {
            let number = str_number_
                .parse::<i32>()
                .map_err(|_| PgsError::InvalidNumber(format!("Invalid number value: {}", str_number_)))?;
            Ok(Value::int(number))
        },
        SingleValue::DateValue(date) => {
            let date_value = Value::date(remove_quotes(date.as_str()))?;
            Ok(date_value)
        },
        SingleValue::BooleanValue(bool) => match bool {
            super::pgs_actions::BOOL::TRUE => Ok(Value::true_()),
            super::pgs_actions::BOOL::FALSE => Ok(Value::false_()),
        },
    }
}

// This function has been obtained from:
// https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
fn remove_quotes(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
