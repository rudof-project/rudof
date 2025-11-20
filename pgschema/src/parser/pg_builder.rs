use std::collections::HashSet;

use rustemo::Parser;

use crate::{
    key::Key,
    parser::{
        pg::PgParser,
        pg_actions::{
            Edge, LabelsRecord, Node, Property, SingleValue, Statement, Values, identifier,
        },
    },
    pg::PropertyGraph,
    pgs_error::PgsError,
    record::Record,
    type_name::LabelName,
    value::Value,
};

pub struct PgBuilder {}

impl Default for PgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PgBuilder {
    pub fn new() -> Self {
        PgBuilder {}
    }
    pub fn parse_pg(&self, input: &str) -> Result<crate::pg::PropertyGraph, PgsError> {
        let pg_content = PgParser::new()
            .parse(input)
            .map_err(|e| PgsError::PGParserError {
                error: e.to_string(),
            })?;
        let mut pg = PropertyGraph::new();
        get_statements(pg_content, &mut pg)?;
        Ok(pg)
    }
}

fn get_statements(statements: Vec<Statement>, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    for statement in statements {
        get_statement(statement, pg)?;
    }
    Ok(())
}

fn get_statement(statement: Statement, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    match statement {
        Statement::Node(node) => get_node(node, pg),
        Statement::Edge(edge) => get_edge(edge, pg),
    }
    /*    let id = get_id(decl.id)?;
    let either = get_node_or_edge(decl.node_edge)?;
    match either {
        Either::Left((labels, record)) => {
            pg.add_node(id, labels, record);
        }
        Either::Right((source, labels, record, target)) => {
            pg.add_edge(id, source, labels, record, target)?;
        }
    }
    Ok(()) */
}

fn get_edge(edge: Edge, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    println!("Getting edge: {:?}", edge);
    // let id = get_id(edge.id)?;
    let source = get_id(edge.source)?;
    let target = get_id(edge.target)?;
    let (labels, record) = get_labels_record(edge.labels_record)?;
    pg.add_edge(edge.id, source, labels, record, target)
}

fn get_node(node: Node, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    let id = get_id(node.id)?;
    let (labels, record) = get_labels_record(node.labels_record)?;
    pg.add_node(id, labels, record);
    Ok(())
}

fn get_labels_record(
    labels_record: LabelsRecord,
) -> Result<(HashSet<LabelName>, Record), PgsError> {
    let labels = if let Some(labels) = labels_record.labels_opt {
        get_labels(labels).unwrap_or_default()
    } else {
        HashSet::new()
    };
    let record = if let Some(record) = labels_record.record_opt {
        get_properties(record)?
    } else {
        Record::new()
    };
    Ok((labels, record))
}

fn get_id(id: String) -> Result<String, PgsError> {
    Ok(id)
}

fn get_labels(labels: Vec<identifier>) -> Result<HashSet<LabelName>, PgsError> {
    let mut result = HashSet::new();
    for label in labels {
        result.insert(label.as_str().into());
    }
    Ok(result)
}

fn get_properties(property_spec: Vec<Property>) -> Result<Record, PgsError> {
    let mut record = Record::new();
    for property in property_spec {
        let (key, values) = get_property(property)?;
        record.insert_values(Key::new(key.as_str()), values)
    }
    Ok(record)
}

fn get_property(property: Property) -> Result<(String, HashSet<Value>), PgsError> {
    let key = property.key.as_str().to_string();
    let values = get_values(property.values)?;
    Ok((key, values))
}

fn get_values(values: Values) -> Result<HashSet<Value>, PgsError> {
    let mut result = HashSet::new();
    match values {
        Values::SingleValue(value) => {
            let value = get_value(value)?;
            result.insert(value);
        }
        Values::ListValue(values_opt) => {
            if let Some(values) = values_opt {
                for value in values {
                    let value = get_value(value)?;
                    result.insert(value);
                }
            }
        }
    }
    Ok(result)
}

fn get_value(value: SingleValue) -> Result<Value, PgsError> {
    match value {
        SingleValue::StringValue(s) => {
            let cleaned = remove_quotes(s.as_str());
            Ok(Value::str(cleaned))
        }
        SingleValue::NumberValue(str_number_) => {
            let number = str_number_.parse::<i32>().map_err(|_| {
                PgsError::InvalidNumber(format!("Invalid number value: {}", str_number_))
            })?;
            Ok(Value::int(number))
        }
        SingleValue::DateValue(date) => {
            let date_value = Value::date(remove_quotes(date.as_str()))?;
            Ok(date_value)
        }
        SingleValue::BooleanValue(bool) => match bool {
            super::pg_actions::BOOL::TRUE => Ok(Value::true_()),
            super::pg_actions::BOOL::FALSE => Ok(Value::false_()),
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
