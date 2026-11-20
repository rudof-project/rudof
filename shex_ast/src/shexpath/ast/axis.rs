//! The axes of a step.

/// The axes other than `child::`.
///
/// `nonChildAxis ::= "thisShapeExpr::" | "thisTripleExpr::" | "self::" | "parent::" | "ancestor::"`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    /// `thisShapeExpr::`: the item and, if it is a `ShapeAnd`, `ShapeOr` or `ShapeNot`, the shape
    /// expressions it combines, transitively.
    ThisShapeExpr,
    /// `thisTripleExpr::`: the item and, if it is an `EachOf` or `OneOf`, the triple expressions
    /// it groups, transitively, an inclusion among them.
    ThisTripleExpr,
    /// `self::`: the item itself.
    Itself,
    /// `parent::`: the item that has this one as a property value or array member.
    Parent,
    /// `ancestor::`: the parent, its parent, and so on up to the schema.
    Ancestor,
    /// Every item below this one, at any depth. Used by the `$` shortcut; it has no spelling.
    Descendant,
}

impl Axis {
    /// The axes that have a spelling, with it.
    pub const SPELLED: &'static [(Axis, &'static str)] = &[
        (Axis::ThisShapeExpr, "thisShapeExpr::"),
        (Axis::ThisTripleExpr, "thisTripleExpr::"),
        (Axis::Itself, "self::"),
        (Axis::Parent, "parent::"),
        (Axis::Ancestor, "ancestor::"),
    ];
}
