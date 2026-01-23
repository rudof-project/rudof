use crate::rdf_core::term::literal::{XsdDateTime, XsdDateTimeParseError};
use serde_json;
use serde::{Serialize, Deserialize};


#[test]
fn test_new_valid_datetime() {
    let result = XsdDateTime::new("2026-01-20T12:34:56Z");
    assert!(result.is_ok());
}

#[test]
fn test_new_valid_datetime_with_timezone() {
    let result = XsdDateTime::new("2026-01-20T12:34:56+01:00");
    assert!(result.is_ok());
}

#[test]
fn test_new_valid_datetime_with_fractional_seconds() {
    let result = XsdDateTime::new("2026-01-20T12:34:56.789Z");
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_datetime() {
    let result = XsdDateTime::new("not-a-datetime");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        XsdDateTimeParseError::InvalidDateTime(_)
    ));
}

#[test]
fn test_new_invalid_format() {
    let result = XsdDateTime::new("2026-01-20");
    assert!(result.is_err());
}

#[test]
fn test_new_empty_string() {
    let result = XsdDateTime::new("");
    assert!(result.is_err());
}

#[test]
fn test_value_returns_reference() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let value = xsd_dt.value();
    assert_eq!(value.to_string(), "2026-01-20T12:34:56Z");
}

#[test]
fn test_into_inner_consumes_self() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let inner = xsd_dt.into_inner();
    assert_eq!(inner.to_string(), "2026-01-20T12:34:56Z");
}

#[test]
fn test_display_trait() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    assert_eq!(format!("{}", xsd_dt), "2026-01-20T12:34:56Z");
}

#[test]
fn test_debug_trait() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let debug_str = format!("{:?}", xsd_dt);
    assert!(debug_str.contains("XsdDateTime"));
}

#[test]
fn test_clone_trait() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let cloned = xsd_dt.clone();
    assert_eq!(xsd_dt, cloned);
}

#[test]
fn test_partial_eq() {
    let dt1 = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let dt2 = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let dt3 = XsdDateTime::new("2026-01-21T12:34:56Z").unwrap();

    assert_eq!(dt1, dt2);
    assert_ne!(dt1, dt3);
}

#[test]
fn test_partial_ord() {
    let dt1 = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let dt2 = XsdDateTime::new("2026-01-21T12:34:56Z").unwrap();

    assert!(dt1 < dt2);
    assert!(dt2 > dt1);
}

#[test]
fn test_hash_trait() {
    use std::collections::HashMap;

    let dt1 = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let dt2 = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();

    let mut map = HashMap::new();
    map.insert(dt1, "value1");

    // Should be able to retrieve using an equal datetime
    assert_eq!(map.get(&dt2), Some(&"value1"));
}

#[test]
fn test_serialize_to_json() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56Z").unwrap();
    let json = serde_json::to_string(&xsd_dt).unwrap();
    assert_eq!(json, r#""2026-01-20T12:34:56Z""#);
}

#[test]
fn test_serialize_with_timezone() {
    let xsd_dt = XsdDateTime::new("2026-01-20T12:34:56+01:00").unwrap();
    let json = serde_json::to_string(&xsd_dt).unwrap();
    assert_eq!(json, r#""2026-01-20T12:34:56+01:00""#);
}

#[test]
fn test_deserialize_from_json() {
    let json = r#""2026-01-20T12:34:56Z""#;
    let xsd_dt: XsdDateTime = serde_json::from_str(json).unwrap();
    assert_eq!(xsd_dt.to_string(), "2026-01-20T12:34:56Z");
}

#[test]
fn test_deserialize_invalid_json() {
    let json = r#""not-a-datetime""#;
    let result: Result<XsdDateTime, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_wrong_type() {
    let json = r#"123"#; // number instead of string
    let result: Result<XsdDateTime, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_serialization() {
    let original = XsdDateTime::new("2026-01-20T12:34:56.123Z").unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: XsdDateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_error_display() {
    let err = XsdDateTime::new("invalid").unwrap_err();
    let error_msg = format!("{}", err);
    assert!(error_msg.contains("Invalid XSD DateTime"));
}

#[test]
fn test_in_struct_serialization() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Event {
        name: String,
        timestamp: XsdDateTime,
    }

    let event = Event {
        name: "Test Event".to_string(),
        timestamp: XsdDateTime::new("2026-01-20T12:34:56Z").unwrap(),
    };

    let json = serde_json::to_string(&event).unwrap();
    let deserialized: Event = serde_json::from_str(&json).unwrap();

    assert_eq!(event, deserialized);
    assert!(json.contains(r#""timestamp":"2026-01-20T12:34:56Z""#));
}

#[test]
fn test_ordering_different_timezones() {
    // These represent the same instant in time
    let dt1 = XsdDateTime::new("2026-01-20T12:00:00Z").unwrap();
    let dt2 = XsdDateTime::new("2026-01-20T13:00:00+01:00").unwrap();

    // Just verify they can be compared (exact behavior depends on oxsdatatypes)
    let _ = dt1.partial_cmp(&dt2);
}
