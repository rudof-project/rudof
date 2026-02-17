use std::fmt::{Debug, Display};

/// Represents an owned blank node in RDF.
///
/// Blank nodes are anonymous resources in RDF that don't have global identifiers.
/// They are only meaningful within the scope of a single RDF graph and are used
/// to represent resources that don't need to be referenced outside of that context.
pub trait BlankNode: Debug + Display + PartialEq {
    /// Constructs a new blank node with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier for this blank node. Accepts any type convertible to `String`, such as `&str` or `String`.
    fn new(id: impl Into<String>) -> Self;
    /// Returns the identifier of this blank node as a string slice.
    fn id(&self) -> &str;
}

/// A trait for borrowed references to blank node labels.
///
/// This trait is useful for working with blank node identifiers without
/// taking ownership, allowing zero-copy operations when processing RDF data.
pub trait BlankNodeRef<'a> {
    /// Returns a reference to the blank node's label.
    fn label(&self) -> &'a str;
}

/// A lightweight borrowed representation of a blank node.
#[derive(Debug, PartialEq)]
pub struct ConcreteBlankNode<'a> {
    s: &'a str,
}

impl<'a> ConcreteBlankNode<'a> {
    /// Constructs a `ConcreteBlankNode` from a string slice
    /// This is a lightweight constructor that simply wraps the provided
    /// string reference without allocating new memory.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice containing the blank node label
    pub fn from(s: &'a str) -> ConcreteBlankNode<'a> {
        ConcreteBlankNode { s }
    }
}

impl<'a> BlankNodeRef<'a> for ConcreteBlankNode<'a> {
    /// Returns the blank node's label as a string slice.
    ///
    /// The returned reference has the same lifetime `'a` as the
    /// `ConcreteBlankNode` instance
    fn label(&self) -> &'a str {
        self.s
    }
}
