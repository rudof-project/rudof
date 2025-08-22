use std::{fmt, result};

use iri_s::IriS;
use prefixmap::IriRef;
use prefixmap::{Deref, DerefError};
use serde::ser::SerializeMap;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize, Serializer,
};
use srdf::RDFS_LABEL_STR;

use super::object_value::ObjectValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}

impl Annotation {
    pub fn new(predicate: IriRef, object: ObjectValue) -> Annotation {
        Annotation { predicate, object }
    }

    pub fn rdfs_label(str: &str) -> Annotation {
        Annotation {
            predicate: IriRef::iri(IriS::new_unchecked(RDFS_LABEL_STR)),
            object: ObjectValue::str(str),
        }
    }

    pub fn rdfs_comment(str: &str) -> Annotation {
        Annotation {
            predicate: IriRef::prefixed("rdfs", "comment"),
            object: ObjectValue::str(str),
        }
    }

    pub fn predicate(&self) -> IriRef {
        self.predicate.clone()
    }

    pub fn object(&self) -> ObjectValue {
        self.object.clone()
    }
}

impl Deref for Annotation {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        let new_pred = self.predicate.deref(base, prefixmap)?;
        let new_obj = self.object.deref(base, prefixmap)?;
        Ok(Annotation {
            predicate: new_pred,
            object: new_obj,
        })
    }
}

impl Serialize for Annotation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", "Annotation")?;
        map.serialize_entry("predicate", &self.predicate)?;
        map.serialize_entry("object", &self.object)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Annotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Predicate,
            Object,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`type` or `predicate` or `object` for annotation")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "predicate" => Ok(Field::Predicate),
                            "object" => Ok(Field::Object),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AnnotationVisitor;

        impl<'de> Visitor<'de> for AnnotationVisitor {
            type Value = Annotation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Annotation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Annotation, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<String> = None;
                let mut predicate: Option<IriRef> = None;
                let mut object: Option<ObjectValue> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Predicate => {
                            if predicate.is_some() {
                                return Err(de::Error::duplicate_field("predicate"));
                            }
                            let iri: IriRef = map.next_value()?;
                            predicate = Some(iri);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;
                            if value != "Annotation" {
                                return Err(de::Error::custom(format!(
                                    "Expected type `Annotation`, found: {value}"
                                )));
                            }
                            type_ = Some("Annotation".to_string());
                        }
                        Field::Object => {
                            if object.is_some() {
                                return Err(de::Error::duplicate_field("object"));
                            }
                            object = Some(map.next_value()?);
                        }
                    }
                }
                match (predicate, object) {
                    (None, None) => {
                        Err(de::Error::custom("Missing fields `predicate` and `object`"))
                    }
                    (Some(_), None) => Err(de::Error::custom("Missing field `object`")),
                    (None, Some(_)) => Err(de::Error::custom("Missing field `predicate`")),
                    (Some(predicate), Some(object)) => Ok(Annotation { predicate, object }),
                }
            }
        }

        const FIELDS: &[&str] = &["type", "predicate", "object"];
        deserializer.deserialize_struct("Annotation", FIELDS, AnnotationVisitor)
    }
}
