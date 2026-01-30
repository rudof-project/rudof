use std::{hash::Hash, collections::HashSet, fmt::Display};
use async_trait::async_trait;

/// An asynchronous trait for querying RDF graphs.
///
/// This trait provides a common interface for performing asynchronous queries
/// on RDF graphs. It abstracts over the specific representation of RDF components,
/// allowing implementations to use different backend storage systems or data structures.
///
/// # Associated Types
///
/// The trait defines several associated types that must be specified by implementors:
///
/// - [`Subject`](Self::Subject): Represents an RDF subject (typically an IRI or blank node)
/// - [`IRI`](Self::IRI): Represents an Internationalized Resource Identifier
/// - [`BNode`](Self::BNode): Represents a blank node
/// - [`Literal`](Self::Literal): Represents a literal value (string, number, etc.)
/// - [`Term`](Self::Term): Represents any RDF term (IRI, blank node, or literal)
/// - [`Err`](Self::Err): The error type returned by operations
///
/// # Thread Safety
///
/// All associated types must implement `Sync + Send` to enable safe concurrent access
/// across thread boundaries, which is essential for asynchronous operations.
#[async_trait]
pub trait AsyncRDF {
    /// The type representing an RDF subject.
    type Subject: Display + Sync + Send;
    /// The type representing an Internationalized Resource Identifier (IRI).
    type IRI: Display + Hash + Eq + Sync + Send;
    /// The type representing an RDF blank node.
    type BNode: Display + Sync + Send;
    /// The type representing an RDF literal value.
    type Literal: Display + Sync + Send;
    /// The type representing any RDF term.
    type Term: Display + Sync + Send;
    /// The error type returned by trait operations.
    type Err: Display + Sync + Send;

    /// Retrieves all predicates associated with a given subject.
    ///
    /// # Arguments
    ///
    /// * `subject` - A reference to the subject whose predicates should be retrieved
    async fn get_predicates_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err>;

    /// Retrieves all objects for a given subject-predicate pair.
    ///
    /// # Arguments
    ///
    /// * `subject` - A reference to the subject of the triples
    /// * `pred` - A reference to the predicate (property) of the triples
    async fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err>;

    /// Retrieves all subjects that have a specific object-predicate pair.
    ///
    /// # Arguments
    ///
    /// * `object` - A reference to the object (term) to search for
    /// * `pred` - A reference to the predicate (property) to match
    async fn get_subjects_for_object_predicate(
        &self,
        object: &Self::Term,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Subject>, Self::Err>;
}
