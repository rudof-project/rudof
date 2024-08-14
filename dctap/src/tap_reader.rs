use crate::tap_error::Result;
use crate::{
    BasicNodeType, DatatypeId, NodeType, PropertyId, ShapeId, TapConfig, TapError, TapReaderState,
    TapShape, TapStatement, Value, ValueConstraint, ValueConstraintType,
};
use csv::{Position, Reader, StringRecord};
use tracing::debug;
// use indexmap::IndexSet;
use std::io::{self};

pub struct TapReader<R> {
    reader: Reader<R>,
    state: TapReaderState,
    config: TapConfig,
}

impl<R: io::Read> TapReader<R> {
    pub fn new(reader: Reader<R>, state: TapReaderState, config: &TapConfig) -> Self {
        TapReader {
            reader,
            state,
            config: config.clone(),
        }
    }
    pub fn shapes(&mut self) -> ShapesIter<R> {
        ShapesIter::new(self)
    }

    pub fn read_shape(&mut self) -> Result<bool> {
        if let Some((record, pos)) = self.next_record()? {
            debug!("Read shape: {pos:?}, record: {record:?}");
            let maybe_shape_id = self.get_shape_id(&record, pos.line())?;
            if let Some(shape_id) = &maybe_shape_id {
                self.state.current_shape().set_shape_id(shape_id);
                self.state.current_shape().set_start_line(pos.line())
            }
            if let Some(shapelabel) = self.get_shape_label(&record)? {
                self.state
                    .current_shape()
                    .set_shape_label(shapelabel.as_str());
            }

            debug!("1st record2statement: {pos:?}, record: {record:?}");
            let maybe_statement = self.record2statement(&record, &pos)?;
            if let Some(statement) = maybe_statement {
                self.state.current_shape().add_statement(statement);
            }
            self.reset_next_record();
            while let Some((record, pos)) = self.next_record_with_id(&maybe_shape_id)? {
                debug!("In loop record2statement: {pos:?}, record: {record:?}");
                let maybe_statement = self.record2statement(&record, &pos)?;
                if let Some(statement) = maybe_statement {
                    self.state.current_shape().add_statement(statement);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn next_record(&mut self) -> Result<Option<(StringRecord, Position)>> {
        if let Some((rcd, pos)) = &self.state.get_cached_next_record() {
            debug!("Cached record {rcd:?}, at {pos:?}");
            Ok(Some(((*rcd).clone(), (*pos).clone())))
        } else {
            let mut record = StringRecord::new();
            let pos = self.reader.position().clone();
            if self
                .reader
                .read_record(&mut record)
                .map_err(|e| TapError::CSVError { err: e })?
            {
                Ok(Some((record, pos.clone())))
            } else {
                Ok(None)
            }
        }
    }

    fn next_record_with_id(
        &mut self,
        shape_id: &Option<ShapeId>,
    ) -> Result<Option<(StringRecord, Position)>> {
        let mut record = StringRecord::new();
        let pos = self.position().clone();
        if self.reader.read_record(&mut record)? {
            let new_shape_id = &mut self.get_shape_id(&record, pos.line())?;
            let c: Option<ShapeId> = new_shape_id.clone();
            if same_shape_id(shape_id, c) {
                Ok(Some((record, pos)))
            } else {
                self.set_next_record(&record, &pos);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn set_next_record(&mut self, rcd: &StringRecord, pos: &Position) -> &mut Self {
        self.state.set_next_record(rcd, pos);
        self
    }

    fn reset_next_record(&mut self) -> &mut Self {
        self.state.reset_next_record();
        self
    }

    fn get_shape_id(&mut self, rcd: &StringRecord, line: u64) -> Result<Option<ShapeId>> {
        if let Some(str) = self.state.headers().shape_id(rcd) {
            let shape_id = ShapeId::new(&str, line);
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

    fn get_property_id(&self, rcd: &StringRecord, pos: &Position) -> Option<PropertyId> {
        if let Some(str) = self.state.headers().property_id(rcd) {
            let property_id = PropertyId::new(&str, pos.line());
            Some(property_id)
        } else {
            None
        }
    }

    fn record2statement(&self, rcd: &StringRecord, pos: &Position) -> Result<Option<TapStatement>> {
        if let Some(property_id) = self.get_property_id(rcd, pos) {
            let mut statement = TapStatement::new(property_id);
            self.read_property_label(&mut statement, rcd);
            self.read_mandatory(&mut statement, rcd, pos)?;
            self.read_repeatable(&mut statement, rcd, pos)?;
            self.read_value_nodetype(&mut statement, rcd, pos)?;
            self.read_value_datatype(&mut statement, rcd, pos);
            self.read_value_shape(&mut statement, rcd, pos.line());
            self.read_value_constraint(&mut statement, rcd, pos)?;
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

    fn read_value_datatype(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
        pos: &Position,
    ) {
        if let Some(str) = self.state.headers().value_datatype(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                let datatype_id = DatatypeId::new(clean_str, pos.line());
                statement.set_value_datatype(&datatype_id);
            }
        }
    }

    fn read_value_nodetype(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
        pos: &Position,
    ) -> Result<()> {
        if let Some(str) = self.state.headers().value_nodetype(rcd) {
            let mut current_node_type: Option<NodeType> = None;
            for str in get_strs(&str) {
                let next_node_type = parse_node_type(str, pos)?;
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
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn read_value_shape(&self, statement: &mut TapStatement, rcd: &StringRecord, line: u64) {
        if let Some(str) = self.state.headers().value_shape(rcd) {
            if let Some(clean_str) = strip_whitespace(&str) {
                let shape_id = ShapeId::new(clean_str, line);
                statement.set_value_shape(&shape_id);
            }
        }
    }

    fn read_value_constraint(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
        pos: &Position,
    ) -> Result<()> {
        if let Some(str) = self.state.headers().value_constraint(rcd) {
            let value_constraint_type = self.read_value_constraint_type(rcd, pos)?;
            match value_constraint_type {
                ValueConstraintType::PickList => {
                    let values = parse_values(str.as_str(), *self.config.picklist_delimiter())?;
                    if !values.is_empty() {
                        statement.set_value_constraint(&ValueConstraint::picklist(values));
                    }
                }
                ValueConstraintType::Pattern => {
                    statement.set_value_constraint(&ValueConstraint::pattern(str.as_str()));
                }
                _ => {
                    debug!("Not implemented handling of value constraint type: {value_constraint_type:?}, It is just ignored")
                }
            }
        };
        Ok(())
    }

    fn read_value_constraint_type(
        &self,
        rcd: &StringRecord,
        pos: &Position,
    ) -> Result<ValueConstraintType> {
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
                    _ => {
                        debug!("UnexpectedValueConstraintType: {str}");
                        Ok(ValueConstraintType::Unknown {
                            value: str.clone(),
                            line: pos.line(),
                        })
                    }
                }
            } else {
                Ok(ValueConstraintType::default())
            }
        } else {
            Ok(ValueConstraintType::default())
        }
    }

    fn read_mandatory(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
        pos: &Position,
    ) -> Result<()> {
        if let Some(str) = self.state.headers().mandatory(rcd) {
            let mandatory = parse_boolean(&str, "mandatory", pos)?;
            statement.set_mandatory(mandatory);
        };
        Ok(())
    }

    fn read_repeatable(
        &self,
        statement: &mut TapStatement,
        rcd: &StringRecord,
        pos: &Position,
    ) -> Result<()> {
        if let Some(str) = self.state.headers().repeatable(rcd) {
            let repeatable = parse_boolean(&str, "repeatable", pos)?;
            statement.set_repeatable(repeatable);
        };
        Ok(())
    }

    fn position(&self) -> &Position {
        self.reader.position()
    }
}

/*fn is_empty(str: &Option<ShapeId>) -> bool {
    match str {
        None => true,
        Some(s) if s.is_empty() => true,
        _ => false,
    }
}*/

fn parse_node_type(str: &str, pos: &Position) -> Result<NodeType> {
    match str.to_uppercase().as_str() {
        "URI" => Ok(NodeType::Basic(BasicNodeType::IRI)),
        "IRI" => Ok(NodeType::Basic(BasicNodeType::IRI)),
        "BNODE" => Ok(NodeType::Basic(BasicNodeType::BNode)),
        "LITERAL" => Ok(NodeType::Basic(BasicNodeType::Literal)),
        _ => Err(TapError::UnexpectedNodeType {
            str: str.to_string(),
            pos: pos.clone(),
        }),
    }
}

fn same_shape_id(shape_id: &Option<ShapeId>, new_shape_id: Option<ShapeId>) -> bool {
    match (shape_id, new_shape_id) {
        (None, None) => true,
        (Some(_), None) => true,
        (Some(s1), Some(s2)) => s1.str() == s2.str(),
        (None, Some(_)) => false,
    }
}

fn parse_boolean(str: &str, field: &str, pos: &Position) -> Result<bool> {
    match str.trim().to_uppercase().as_str() {
        "" => Ok(false),
        "YES" => Ok(true),
        "NO" => Ok(false),
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(TapError::ShouldBeBoolean {
            field: field.to_string(),
            value: str.to_string(),
            pos: pos.clone(),
        }),
    }
}

fn parse_values(str: &str, delimiter: char) -> Result<Vec<Value>> {
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
    pub fn _reader_mut(&mut self) -> &mut TapReader<R> {
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
    // use tracing_test::traced_test;

    #[test]
    fn test_simple() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel";
        let mut tap_reader =
            TapReaderBuilder::from_reader(data.as_bytes(), &TapConfig::default()).unwrap();
        let mut expected_shape = TapShape::new(2);
        expected_shape.set_shape_id(&ShapeId::new("Person", 2));
        expected_shape.set_shape_label("PersonLabel");
        let mut statement = TapStatement::new(PropertyId::new("knows", 2));
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
        let mut tap_reader =
            TapReaderBuilder::from_reader(data.as_bytes(), &TapConfig::default()).unwrap();
        let mut expected_shape = TapShape::new(2);
        expected_shape.set_shape_id(&ShapeId::new("Person", 2));
        expected_shape.set_shape_label("PersonLabel");
        let mut statement = TapStatement::new(PropertyId::new("knows", 2));
        statement.set_property_label("KnowsLabel");
        expected_shape.add_statement(statement);
        let mut statement = TapStatement::new(PropertyId::new("name", 3));
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
        let mut tap_reader =
            TapReaderBuilder::from_reader(data.as_bytes(), &TapConfig::default()).unwrap();
        let mut expected_shape1 = TapShape::new(2);
        expected_shape1.set_shape_id(&ShapeId::new("Person", 2));
        expected_shape1.set_shape_label("PersonLabel");
        let mut statement = TapStatement::new(PropertyId::new("knows", 2));
        statement.set_property_label("KnowsLabel");
        expected_shape1.add_statement(statement);
        let mut statement = TapStatement::new(PropertyId::new("name", 3));
        statement.set_property_label("NameLabel");
        expected_shape1.add_statement(statement);
        let next_shape1 = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape1, expected_shape1);

        let mut expected_shape2 = TapShape::new(4);
        expected_shape2.set_shape_id(&ShapeId::new("Company", 4));
        expected_shape2.set_shape_label("CompanyLabel");
        let mut statement = TapStatement::new(PropertyId::new("founder", 4));
        statement.set_property_label("FounderLabel");
        expected_shape2.add_statement(statement);
        let next_shape2 = tap_reader.shapes().next().unwrap().unwrap();
        assert_eq!(next_shape2, expected_shape2);
    }
}
