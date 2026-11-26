//! The locations of items.

use crate::common::*;
use shex_ast::shexpath::{Location, Segment};

#[test]
fn the_schema_is_at_the_empty_pointer() {
    assert_eq!(Location::root().to_string(), "");
    assert_eq!(issue().root().location(), &Location::root());
}

#[test]
fn a_location_is_a_json_pointer() {
    let location = Location::root().key("shapes").index(0).key("shapeExpr");
    assert_eq!(location.to_string(), "/shapes/0/shapeExpr");
    assert_eq!(
        location.segments(),
        [
            Segment::Key("shapes".to_string()),
            Segment::Index(0),
            Segment::Key("shapeExpr".to_string())
        ]
    );
    assert_eq!(Location::root().key("a/b~c").to_string(), "/a~1b~0c");
}

#[test]
fn the_parent_location_drops_the_last_step() {
    let location = Location::root().key("shapes").index(0);
    assert_eq!(location.parent(), Some(Location::root().key("shapes")));
    assert_eq!(Location::root().parent(), None);
}
