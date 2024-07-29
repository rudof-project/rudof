use crate::{tap_error::Result, tap_headers::TapHeaders};
use crate::{
    BasicNodeType, DatatypeId, NodeType, PropertyId, ShapeId, TapConfig, TapError, TapReaderState,
    TapShape, TapStatement, Value, ValueConstraint, ValueConstraintType,
};
use csv::{Position, Reader, ReaderBuilder, StringRecord, Terminator, Trim};
use std::fs::File;
// use indexmap::IndexSet;
use std::io::{self};
use std::path::Path;

pub struct TapReader<R> {
    reader: Reader<R>,
    state: TapReaderState,
    config: TapConfig,
}

impl<R: io::Read> TapReader<R> {
    pub fn new(reader: Reader<R>, state: TapReaderState, config: TapConfig) -> Self {
        TapReader {
            reader,
            state,
            config,
        }
    }
    pub fn shapes(&mut self) -> ShapesIter<R> {
        ShapesIter::new(self)
    }

    pub fn read_shape(&mut self) -> Result<bool> {
        if let Some(record) = self.next_record()? {
            let maybe_shape_id = self.get_shape_id(&record)?;
            if let Some(shape_id) = &maybe_shape_id {
                self.state.current_shape().set_shape_id(shape_id);
            }
            if let Some(shapelabel) = self.get_shape_label(&record)? {
                self.state
                    .current_shape()
                    .set_shape_label(shapelabel.as_str());
            }

            let maybe_statement = self.record2statement(&record)?;
            if let Some(statement) = maybe_statement {
                self.state.current_shape().add_statement(statement);
            }
            self.reset_next_record();
            while let Some(record) = self.next_record_with_id(&maybe_shape_id)? {
                let maybe_statement = self.record2statement(&record)?;
                if let Some(statement) = maybe_statement {
                    self.state.current_shape().add_statement(statement);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn next_record(&mut self) -> Result<Option<StringRecord>> {
        if let Some(rcd) = &self.state.get_cached_next_record() {
            Ok(Some((*rcd).clone()))
        } else {
            let mut record = StringRecord::new();
            if self.reader.read_record(&mut record)? {
                Ok(Some(record))
            } else {
                Ok(None)
            }
        }
    }

    fn next_record_with_id(&mut self, shape_id: &Option<ShapeId>) -> Result<Option<StringRecord>> {
        let mut record = StringRecord::new();
        if self.reader.read_record(&mut record)? {
            let new_shape_id = self.get_shape_id(&record)?;
            if is_empty(&new_shape_id) || new_shape_id == *shape_id {
                Ok(Some(record))
            } else {
                self.set_next_record(&record);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn set_next_record(&mut self, rcd: &StringRecord) -> &mut Self {
        self.state.set_next_record(rcd);
        self
    }

    fn reset_next_record(&mut self) -> &mut Self {
        self.state.reset_next_record();
        self
    }

    fn get_shape_id(&mut self, rcd: &StringRecord) -> Result<Option<ShapeId>> {
        if let Some(str) = self.state.headers().shape_id(rcd) {
            let shape_id = ShapeId::new(&str);
            Ok(Some(shape_id))
        } else {
            Ok(None)
        }
    }

    fn get_shape_label(&mut self, rcd: &StringRecord) -> Result<Option<String>> {
        if let Some(str) = self.state.headers().shape_label(rcd) {
            Ok(Some(str.to_string()))
        } else {
            Ok(None)
        }
    }

    fn get_property_id(&self, rcd: &StringRecord) -> Option<PropertyId> {
        if let Some(str) = self.state.headers().property_id(rcd) {
            let property_id = PropertyId::new(&str);
            Some(property_id)
        } else {
            None
        }
    }

    fn record2statement(&self, rcd: &StringRecord) -> Result<Option<TapStatement>> {
        if let Some(property_id) = self.get_property_id(rcd) {
            let mut statement = TapStatement::new(property_id);
            self.read_property_label(&mut statement, rcd);
            self.read_mandatory(&mut statement, rcd)?;
            self.read_repeatable(&mut statement, rcd)?;
            self.read_value_nodetype(&mut statement, rcd)?;
            self.read_value_datatype(&mut statement, rcd);
            self.read_value_shape(&mut statement, rcd);
            self.read_value_constraint(&mut statement, rcd)?;
            self.read_note(&mut statement, rcd);
            Ok(Some(statement))
        } else {
            Ok(None)
        }
    }

    fn read_property_label(&self, statement: &mut TapStatement, rcd: &StringRecord) {
        if let Some(str) = self.state.headers().property_label(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                statement.set_property_label(clean_str);
            }
        }
    }

    fn read_note(&self, statement: &mut TapStatement, rcd: &StringRecord) {
        if let Some(str) = self.state.headers().note(rcd) {
            if !str.is_empty() {
                statement.set_note(&str);
            }
        }
    }

    fn read_value_datatype(&self, statement: &mut TapStatement, rcd: &StringRecord) {
        if let Some(str) = self.state.headers().value_datatype(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                let datatype_id = DatatypeId::new(clean_str);
                statement.set_value_datatype(&datatype_id);
            }
        }
    }

    fn read_value_nodetype(&self, statement: &mut TapStatement, rcd: &StringRecord) -> Result<()> {
        if let Some(str) = self.state.headers().value_nodetype(rcd) {
            let mut current_node_type: Option<NodeType> = None;
            for str in get_strs(&str) {
                let next_node_type = parse_node_type(str)?;
                match &mut current_node_type {
                    Some(node_type) => {
                        current_node_type = Some(node_type.merge_node_type(&next_node_type))
                    }
                    None => current_node_type = Some(next_node_type),
                }
            }
            if let Some(node_type) = current_node_type {
                statement.set_value_nodetype(&node_type);
                Ok(())
            } else {
                Err(TapError::EmptyNodeType)
            }
        } else {
            Ok(())
        }
    }

    fn read_value_shape(&self, statement: &mut TapStatement, rcd: &StringRecord) {
        if let Some(str) = self.state.headers().value_shape(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                let shape_id = ShapeId::new(clean_str);
                statement.set_value_shape(&shape_id);
            }
        }
    }

    fn read_value_constraint(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
    ) -> Result<()> {
        if let Some(str) = self.state.headers().value_constraint(rcd) {
            let value_constraint_type = self.read_value_constraint_type(rcd)?;
            match value_constraint_type {
                ValueConstraintType::PickList => {
                    let values = parse_values(str.as_str(), self.config.picklist_delimiter())?;
                    if !values.is_empty() {
                        statement.set_value_constraint(&ValueConstraint::picklist(values));
                    }
                }
                ValueConstraintType::Pattern => {
                    statement.set_value_constraint(&ValueConstraint::pattern(str.as_str()));
                }
                _ => todo!(),
            }
        };
        Ok(())
    }

    fn read_value_constraint_type(&self, rcd: &StringRecord) -> Result<ValueConstraintType> {
        if let Some(str) = self.state.headers().value_constraint_type(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                match clean_str.to_uppercase().as_str() {
                    "PICKLIST" => Ok(ValueConstraintType::PickList),
                    "PATTERN" => Ok(ValueConstraintType::Pattern),
                    "LANGUAGETAG" => Ok(ValueConstraintType::LanguageTag),
                    "IRISTEM" => Ok(ValueConstraintType::IRIStem),
                    "MINLENGTH" => Ok(ValueConstraintType::MinLength),
                    "MAXLENGTH" => Ok(ValueConstraintType::MaxLength),
                    "MININCLUSIVE" => Ok(ValueConstraintType::MinInclusive),
                    "MINEXCLUSIVE" => Ok(ValueConstraintType::MinExclusive),
                    "MAXINCLUSIVE" => Ok(ValueConstraintType::MinInclusive),
                    "MAXEXCLUSIVE" => Ok(ValueConstraintType::MaxExclusive),
                    _ => Err(TapError::UnexpectedValueConstraintType { value: str.clone() }),
                }
            } else {
                Ok(ValueConstraintType::default())
            }
        } else {
            Ok(ValueConstraintType::default())
        }
    }

    fn read_mandatory(&self, statement: &mut TapStatement, rcd: &StringRecord) -> Result<()> {
        if let Some(str) = self.state.headers().mandatory(rcd) {
            let mandatory = parse_boolean(&str, "mandatory")?;
            statement.set_mandatory(mandatory);
        };
        Ok(())
    }

    fn read_repeatable(&self, statement: &mut TapStatement, rcd: &StringRecord) -> Result<()> {
        if let Some(str) = self.state.headers().repeatable(rcd) {
            let repeatable = parse_boolean(&str, "repeatable")?;
            statement.set_repeatable(repeatable);
        };
        Ok(())
    }
}

fn is_empty(str: &Option<ShapeId>) -> bool {
    match str {
        None => true,
        Some(s) if s.is_empty() => true,
        _ => false,
    }
}

fn parse_node_type(str: &str) -> Result<NodeType> {
    match str.to_uppercase().as_str() {
        "IRI" => Ok(NodeType::Basic(BasicNodeType::IRI)),
        "BNODE" => Ok(NodeType::Basic(BasicNodeType::BNode)),
        "LITERAL" => Ok(NodeType::Basic(BasicNodeType::Literal)),
        _ => Err(TapError::UnexpectedNodeType {
            str: str.to_string(),
        }),
    }
}

fn parse_boolean(str: &str, field: &str) -> Result<bool> {
    match str.trim().to_uppercase().as_str() {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(TapError::ShouldBeBoolean {
            field: field.to_string(),
            value: str.to_string(),
        }),
    }
}

fn parse_values(str: &str, delimiter: &str) -> Result<Vec<Value>> {
    Ok(str.split_terminator(delimiter).map(Value::new).collect())
}

fn strip_whitespace(str: &str) -> Option<&str> {
    let s = str.trim();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn get_strs(str: &str) -> impl Iterator<Item = &str> {
    str.split(|c| c == ' ').filter(|&x| !x.is_empty())
}

/// A borrowed iterator over Shapes
///
/// The lifetime parameter `'r` refers to the lifetime of the underlying `TapReader`.
pub struct ShapesIter<'r, R: 'r> {
    reader: &'r mut TapReader<R>,
}

impl<'r, R: io::Read> ShapesIter<'r, R> {
    fn new(reader: &'r mut TapReader<R>) -> ShapesIter<'r, R> {
        ShapesIter { reader }
    }

    /// Return a mutable reference to the underlying `TapReader`.
    pub fn reader_mut(&mut self) -> &mut TapReader<R> {
        self.reader
    }
}

impl<'r, R: io::Read> Iterator for ShapesIter<'r, R> {
    type Item = Result<TapShape>;

    fn next(&mut self) -> Option<Result<TapShape>> {
        match self.reader.read_shape() {
            Err(err) => Some(Err(err)),
            Ok(true) => Some(Ok(self.reader.state.current_shape().clone())),
            Ok(false) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{TapReaderBuilder, TapShape};

    use super::*;

    #[test]
    fn test_simple() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel
";
        let mut tap_reader = TapReaderBuilder::new()
            .from_reader(data.as_bytes(), TapConfig::default())
            .unwrap();
        let mut expected_shape = TapShape::new();
        expected_shape.set_shape_id(&ShapeId::new("Person"));
        let mut statement = TapStatement::new(PropertyId::new("knows"));
        statement.set_property_label("KnowsLabel");
        expected_shape.add_statement(statement);
        let next_shape = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape, expected_shape);
    }

    #[test]
    fn test_2lines() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel
,,name,NameLabel
";
        let mut tap_reader = TapReaderBuilder::new()
            .from_reader(data.as_bytes(), TapConfig::default())
            .unwrap();
        let mut expected_shape = TapShape::new();
        expected_shape.set_shape_id(&ShapeId::new("Person"));
        let mut statement = TapStatement::new(PropertyId::new("knows"));
        statement.set_property_label("KnowsLabel");
        expected_shape.add_statement(statement);
        let mut statement = TapStatement::new(PropertyId::new("name"));
        statement.set_property_label("NameLabel");
        expected_shape.add_statement(statement);
        let next_shape = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape, expected_shape);
    }

    #[test]
    fn test_2shapes() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel
,,name,NameLabel
Company,CompanyLabel,founder,FounderLabel
";
        let mut tap_reader = TapReaderBuilder::new()
            .from_reader(data.as_bytes(), TapConfig::default())
            .unwrap();
        let mut expected_shape1 = TapShape::new();
        expected_shape1.set_shape_id(&ShapeId::new("Person"));
        let mut statement = TapStatement::new(PropertyId::new("knows"));
        statement.set_property_label("KnowsLabel");
        expected_shape1.add_statement(statement);
        let mut statement = TapStatement::new(PropertyId::new("name"));
        statement.set_property_label("NameLabel");
        expected_shape1.add_statement(statement);
        let next_shape1 = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape1, expected_shape1);

        let mut expected_shape2 = TapShape::new();
        expected_shape2.set_shape_id(&ShapeId::new("Company"));
        let mut statement = TapStatement::new(PropertyId::new("founder"));
        statement.set_property_label("FounderLabel");
        expected_shape2.add_statement(statement);
        let next_shape2 = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape2, expected_shape2);
    }
}
