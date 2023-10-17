use std::{fmt, result};

use log::debug;
// use log::debug;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize, Serializer,
};

use super::ValueSetValue;
use crate::{
    IriRef, NodeKind, NumericLiteral, Pattern, StringFacet, ValueSetValueWrapper, XsFacet,
};
use serde::ser::SerializeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeConstraint {
    // #[serde(default, rename = "nodeKind", skip_serializing_if = "Option::is_none")]
    node_kind: Option<NodeKind>,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    datatype: Option<IriRef>,

    // #[serde(default, rename = "xsFacet", skip_serializing_if = "Option::is_none"]
    xs_facet: Option<Vec<XsFacet>>,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    values: Option<Vec<ValueSetValueWrapper>>,
}

impl NodeConstraint {
    pub fn new() -> NodeConstraint {
        NodeConstraint {
            node_kind: None,
            datatype: None,
            xs_facet: None,
            values: None,
        }
    }

    pub fn with_datatype(mut self, datatype: IriRef) -> Self {
        self.datatype = Some(datatype);
        self
    }

    pub fn datatype(&self) -> Option<IriRef> {
        self.datatype.clone()
    }

    pub fn with_node_kind(mut self, node_kind: NodeKind) -> Self {
        self.node_kind = Some(node_kind);
        self
    }

    pub fn node_kind(&self) -> Option<NodeKind> {
        self.node_kind.clone()
    }

    pub fn with_xsfacets(mut self, facets: Vec<XsFacet>) -> Self {
        self.xs_facet = Some(facets);
        self
    }

    pub fn with_pattern(mut self, pat: &str) -> Self {
        match self.xs_facet {
            Some(ref mut facets) => facets.push(XsFacet::pattern(pat)),
            None => self.xs_facet = Some(vec![XsFacet::pattern(pat)]),
        }
        self
    }

    pub fn with_pattern_flags(mut self, pat: &str, flags: &str) -> Self {
        match self.xs_facet {
            Some(ref mut facets) => facets.push(XsFacet::pattern_flags(pat, flags)),
            None => self.xs_facet = Some(vec![XsFacet::pattern_flags(pat, flags)]),
        }
        self
    }

    pub fn xs_facet(&self) -> Option<Vec<XsFacet>> {
        self.xs_facet.clone()
    }

    pub fn with_length(self, len: usize) -> Self {
        self.add_facet(XsFacet::length(len))
    }

    pub fn with_minlength(self, len: usize) -> Self {
        self.add_facet(XsFacet::min_length(len))
    }

    pub fn with_maxlength(self, len: usize) -> Self {
        self.add_facet(XsFacet::max_length(len))
    }

    pub fn with_min_inclusive(self, n: NumericLiteral) -> Self {
        self.add_facet(XsFacet::min_inclusive(n))
    }

    pub fn with_max_inclusive(self, n: NumericLiteral) -> Self {
        self.add_facet(XsFacet::max_inclusive(n))
    }

    pub fn add_facet(mut self, f: XsFacet) -> Self {
        match self.xs_facet {
            Some(ref mut facets) => facets.push(f),
            None => self.xs_facet = Some(vec![f]),
        }
        self
    }

    pub fn with_min_exclusive(self, n: NumericLiteral) -> Self {
        self.add_facet(XsFacet::min_exclusive(n))
    }

    pub fn with_max_exclusive(self, n: NumericLiteral) -> Self {
        self.add_facet(XsFacet::max_exclusive(n))
    }

    pub fn with_totaldigits(self, n: usize) -> Self {
        self.add_facet(XsFacet::totaldigits(n))
    }

    pub fn with_fractiondigits(self, n: usize) -> Self {
        self.add_facet(XsFacet::fractiondigits(n))
    }

    pub fn with_values(mut self, values: Vec<ValueSetValue>) -> Self {
        let mut vs: Vec<ValueSetValueWrapper> = Vec::with_capacity(values.len());
        for v in values {
            vs.push(ValueSetValueWrapper::new(v));
        }
        self.values = Some(vs);
        self
    }

    pub fn with_values_wrapped(mut self, values: Vec<ValueSetValueWrapper>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn values(&self) -> Option<Vec<ValueSetValue>> {
        match &self.values {
            None => None,
            Some(values) => {
                let mut vs: Vec<ValueSetValue> = Vec::with_capacity(values.len());
                for v in values {
                    vs.push(v.value());
                }
                Some(vs)
            }
        }
    }
}

impl Serialize for NodeConstraint {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NodeConstraint {
                node_kind,
                datatype,
                xs_facet,
                values,
            } => {
                debug!("Serializing NodeConstraint: {self:?}");
                let mut map = serializer.serialize_map(None)?;
                
                // map.serialize_entry("type", "NodeConstraint")?;
                match node_kind {
                    None => (),
                    Some(nk) => {
                        map.serialize_entry("nodeKind", &format!("{nk}").to_lowercase())?;
                    }
                }
                match datatype {
                    None => (),
                    Some(dt) => {
                        map.serialize_entry("datatype", &format!("{dt}"))?;
                    }
                }
                match values {
                    None => (),
                    Some(values) => {
                        map.serialize_entry("values", &values)?;
                    }
                }
                match xs_facet {
                    None => (),
                    Some(facets) => {
                        for f in facets {
                            match f {
                                XsFacet::StringFacet(sf) => match sf {
                                    StringFacet::Length(l) => map.serialize_entry("length", l)?,
                                    StringFacet::MinLength(ml) => {
                                        map.serialize_entry("minlength", ml)?
                                    }
                                    StringFacet::MaxLength(ml) => {
                                        map.serialize_entry("maxlength", ml)?
                                    }
                                    StringFacet::Pattern(Pattern { str, flags: None }) => {
                                        map.serialize_entry("pattern", str)?;
                                    }
                                    StringFacet::Pattern(Pattern {
                                        str,
                                        flags: Some(fs),
                                    }) => {
                                        map.serialize_entry("pattern", str)?;
                                        map.serialize_entry("flags", fs)?;
                                    }
                                },
                                XsFacet::NumericFacet(_) => todo!(),
                            }
                        }
                    }
                }
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for NodeConstraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            NodeKind,
            Datatype,
            Values,
            Length,
            MinLength,
            MaxLength,
            Pattern,
            Flags,
            MinInclusive,
            MaxInclusive,
            MinExclusive,
            MaxExclusive,
            TotalDigits,
            FractionDigits,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                            "`type` or `nodeKind` or `datatype` or some xsfacet or `values` ",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "nodeKind" => Ok(Field::NodeKind),
                            "datatype" => Ok(Field::Datatype),
                            "type" => Ok(Field::Type),
                            "pattern" => Ok(Field::Pattern),
                            "flags" => Ok(Field::Flags),
                            "length" => Ok(Field::Length),
                            "minlength" => Ok(Field::MinLength),
                            "maxlength" => Ok(Field::MaxLength),
                            "mininclusive" => Ok(Field::MinInclusive),
                            "maxinclusive" => Ok(Field::MaxInclusive),
                            "minexclusive" => Ok(Field::MinExclusive),
                            "maxexclusive" => Ok(Field::MaxExclusive),
                            "totaldigits" => Ok(Field::TotalDigits),
                            "fractiondigits" => Ok(Field::FractionDigits),
                            "values" => Ok(Field::Values),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct NodeConstraintVisitor;

        impl<'de> Visitor<'de> for NodeConstraintVisitor {
            type Value = NodeConstraint;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct NodeConstraint")
            }

            fn visit_map<V>(self, mut map: V) -> Result<NodeConstraint, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<String> = None;
                let mut node_kind: Option<NodeKind> = None;
                let mut datatype: Option<IriRef> = None;
                let mut pattern: Option<String> = None;
                let mut length: Option<usize> = None;
                let mut minlength: Option<usize> = None;
                let mut maxlength: Option<usize> = None;
                let mut mininclusive: Option<NumericLiteral> = None;
                let mut maxinclusive: Option<NumericLiteral> = None;
                let mut minexclusive: Option<NumericLiteral> = None;
                let mut maxexclusive: Option<NumericLiteral> = None;
                let mut totaldigits: Option<usize> = None;
                let mut fractiondigits: Option<usize> = None;
                let mut flags: Option<String> = None;
                let mut values: Option<Vec<ValueSetValueWrapper>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::NodeKind => {
                            if node_kind.is_some() {
                                return Err(de::Error::duplicate_field("nodeKind"));
                            }
                            let value = map.next_value()?;
                            node_kind = match value {
                                "iri" => Some(NodeKind::Iri),
                                "bnode" => Some(NodeKind::BNode),
                                "literal" => Some(NodeKind::Literal),
                                "nonliteral" => Some(NodeKind::NonLiteral),
                                _ => {
                                    return Err(de::Error::custom(format!(
                                        "Unexpected value for `nodeKind`: {value}"
                                    )))
                                }
                            }
                        }
                        Field::Datatype => {
                            if datatype.is_some() {
                                return Err(de::Error::duplicate_field("datatype"));
                            }
                            let iri: IriRef = map.next_value()?;
                            datatype = Some(iri);
                        }
                        Field::Values => {
                            if values.is_some() {
                                return Err(de::Error::duplicate_field("values"));
                            }
                            let vs: Vec<ValueSetValueWrapper> = map.next_value()?;
                            values = Some(vs)
                        }
                        Field::Pattern => {
                            if pattern.is_some() {
                                return Err(de::Error::duplicate_field("pattern"));
                            }
                            pattern = Some(map.next_value()?);
                        }
                        Field::Length => {
                            if length.is_some() {
                                return Err(de::Error::duplicate_field("length"));
                            }
                            length = Some(map.next_value()?);
                        }
                        Field::MinLength => {
                            if minlength.is_some() {
                                return Err(de::Error::duplicate_field("minlength"));
                            }
                            minlength = Some(map.next_value()?);
                        }
                        Field::MaxLength => {
                            if maxlength.is_some() {
                                return Err(de::Error::duplicate_field("maxlength"));
                            }
                            maxlength = Some(map.next_value()?);
                        }
                        Field::MinInclusive => {
                            if mininclusive.is_some() {
                                return Err(de::Error::duplicate_field("mininclusive"));
                            }
                            mininclusive = Some(map.next_value()?);
                        }
                        Field::MaxInclusive => {
                            if maxinclusive.is_some() {
                                return Err(de::Error::duplicate_field("maxinclusive"));
                            }
                            maxinclusive = Some(map.next_value()?);
                        }
                        Field::MinExclusive => {
                            if minexclusive.is_some() {
                                return Err(de::Error::duplicate_field("minexclusive"));
                            }
                            minexclusive = Some(map.next_value()?);
                        }
                        Field::MaxExclusive => {
                            if maxexclusive.is_some() {
                                return Err(de::Error::duplicate_field("maxexclusive"));
                            }
                            maxexclusive = Some(map.next_value()?);
                        }
                        Field::TotalDigits => {
                            if totaldigits.is_some() {
                                return Err(de::Error::duplicate_field("totaldigits"));
                            }
                            totaldigits = Some(map.next_value()?);
                        }
                        Field::FractionDigits => {
                            if fractiondigits.is_some() {
                                return Err(de::Error::duplicate_field("fractiondigits"));
                            }
                            fractiondigits = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;
                            if value != "NodeConstraint" {
                                return Err(de::Error::custom(format!(
                                    "Expected NodeConstraint, found: {value}"
                                )));
                            }
                            type_ = Some("NodeConstraint".to_string());
                        }
                        Field::Flags => {
                            if flags.is_some() {
                                return Err(de::Error::duplicate_field("flags"));
                            }
                            flags = Some(map.next_value()?);
                        }
                    }
                }
                let mut nc = NodeConstraint::new();
                if let Some(nk) = node_kind {
                    nc = nc.with_node_kind(nk)
                }
                if let Some(pat) = pattern {
                    if let Some(flags) = flags {
                        nc = nc.with_pattern_flags(&pat, &flags)
                    } else {
                        nc = nc.with_pattern(&pat)
                    }
                }
                if let Some(length) = length {
                    nc = nc.with_length(length)
                }
                if let Some(datatype) = datatype {
                    nc = nc.with_datatype(datatype)
                }
                if let Some(values) = values {
                    nc = nc.with_values_wrapped(values)
                }
                if let Some(minlength) = minlength {
                    nc = nc.with_minlength(minlength)
                }
                if let Some(maxlength) = maxlength {
                    nc = nc.with_maxlength(maxlength)
                }
                if let Some(mininclusive) = mininclusive {
                    nc = nc.with_min_inclusive(mininclusive)
                }
                if let Some(maxinclusive) = maxinclusive {
                    nc = nc.with_max_inclusive(maxinclusive)
                }
                if let Some(minexclusive) = minexclusive {
                    nc = nc.with_min_exclusive(minexclusive)
                }
                if let Some(maxexclusive) = maxexclusive {
                    nc = nc.with_max_exclusive(maxexclusive)
                }
                if let Some(totaldigits) = totaldigits {
                    nc = nc.with_totaldigits(totaldigits)
                }
                if let Some(fractiondigits) = fractiondigits {
                    nc = nc.with_fractiondigits(fractiondigits)
                }
                Ok(nc)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "type",
            "nodeKind",
            "datatype",
            "values",
            "pattern",
            "flags",
            "length",
            "minlength",
            "maxlength",
            "mininclusive",
            "maxinclusive",
            "minexclusive",
            "maxexclusive",
            "totaldigits",
            "fractiondigits",
        ];
        deserializer.deserialize_struct("NodeConstraint", FIELDS, NodeConstraintVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_node_kind_iri() {
        let nc = NodeConstraint::new().with_node_kind(NodeKind::Iri);
        let json_nc = serde_json::to_string(&nc).unwrap();
        assert_eq!(json_nc, "{\"nodeKind\":\"iri\"}");
    }

    #[test]
    fn test_deserialize_node_kind_iri() {
        let str = r#"{ "type":"NodeConstraint","nodeKind": "iri"}"#;
        let deserialized: NodeConstraint = serde_json::from_str(str).unwrap();
        let expected = NodeConstraint::new().with_node_kind(NodeKind::Iri);
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_serialize_pattern() {
        let nc = NodeConstraint::new().with_pattern("o*");
        let json_nc = serde_json::to_string(&nc).unwrap();
        let expected = r#"{"pattern":"o*"}"#;
        assert_eq!(json_nc, expected);
    }

    #[test]
    fn test_deserialize_pattern() {
        let str = r#"{ "type":"NodeConstraint","pattern": "o*"}"#;
        let deserialized: NodeConstraint = serde_json::from_str(str).unwrap();
        let expected = NodeConstraint::new().with_pattern("o*");
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_serialize_pattern_flags() {
        let nc = NodeConstraint::new().with_pattern_flags("o*", "i");
        let json_nc = serde_json::to_string(&nc).unwrap();
        let expected = r#"{"pattern":"o*","flags":"i"}"#;
        assert_eq!(json_nc, expected);
    }
    #[test]
    fn test_deserialize_pattern_flags() {
        let str = r#"{ "type":"NodeConstraint","pattern": "o*", "flags": "i"}"#;
        let deserialized: NodeConstraint = serde_json::from_str(str).unwrap();
        let expected = NodeConstraint::new().with_pattern_flags("o*", "i");
        assert_eq!(deserialized, expected);
    }
}
