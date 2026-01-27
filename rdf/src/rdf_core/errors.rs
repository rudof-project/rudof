use thiserror::Error;

/// Represents all possible errors that can occur during RDF operations.
#[derive(Error, Debug, PartialEq)]
pub enum RDFError {
    // ========================================================================
    // Language and Literal Errors
    // ========================================================================
    /// Error parsing or validating a language tag in an RDF literal.
    ///
    /// # Fields
    /// - `literal`: The complete literal value that contains the invalid language tag
    /// - `language`: The invalid language tag that was encountered
    /// - `error`: A detailed description of why the language tag is invalid
    #[error("Error with language tag '{language}' in literal '{literal}': {error}")]
    LanguageTagError {
        literal: String,
        language: String,
        error: String,
    },

    // ========================================================================
    // IRI Errors
    // ========================================================================
    /// Error obtaining a valid IRI from an IRI reference.
    ///
    /// This occurs when attempting to resolve an IRI reference (which may be
    /// relative) into an absolute IRI, but the reference is invalid or cannot
    /// be resolved.
    ///
    /// # Fields
    /// - `iri_ref`: The IRI reference string that could not be resolved
    #[error("Error obtaining IRI from IriRef: {iri_ref}")]
    IriRefError { iri_ref: String },

    /// Error parsing a string into a valid IRI.
    ///
    /// # Fields
    /// - `iri`: The string that failed to parse as an IRI
    /// - `error`: Details about the parsing failure
    #[error("RDF error parsing iri {iri}: {error}")]
    ParsingIri { iri: String, error: String },

    // ========================================================================
    // Conversion Errors
    // ========================================================================
    /// Generic conversion error for RDF data transformations.
    ///
    /// # Fields
    /// - `msg`: A human-readable message describing the conversion failure
    #[error("Conversion error {msg}")]
    ConversionError { msg: String },

    /// Error converting an RDF object node to a generic RDF term.
    ///
    /// # Fields
    /// - `object`: String representation of the object that failed conversion
    #[error("Converting Object {object} to RDF term")]
    ObjectAsTerm { object: String },

    /// Error converting a generic RDF term to a specific IRI type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed IRI conversion
    #[error("Converting term {term} to IRI")]
    TermAsIri { term: String },

    /// Error converting a generic RDF term to a blank node.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed blank node conversion
    #[error("Converting term {term} to BNode")]
    TermAsBNode { term: String },

    /// Error converting a generic RDF term to a concrete IRI type (`IriS`).
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed IriS conversion
    #[error("Converting term {term} to concrete IRI")]
    TermAsIriS { term: String },

    /// Error converting a generic RDF term to a literal.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed literal conversion
    #[error("Converting term {term} to Literal")]
    TermAsLiteral { term: String },

    /// Error converting a generic literal to a specific string literal type (`ConcreteLiteral`).
    ///
    /// # Fields
    /// - `literal`: String representation of the literal that failed conversion
    #[error("Converting literal {literal} to ConcreteLiteral")]
    LiteralAsSLiteral { literal: String },

    /// Error converting a generic RDF term to an object node.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed object conversion
    /// - `error`: Detailed description of the conversion failure
    #[error("Converting Term {term} to Object: {error}")]
    TermAsObject { term: String, error: String },

    /// Error converting a generic RDF term to a subject node.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed subject conversion
    #[error("Converting term {term} to subject")]
    TermAsSubject { term: String },

    /// Error converting a generic RDF term to a language tag.
    ///
    /// This typically occurs when attempting to extract language information
    /// from a term that is not a language-tagged string literal.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed language tag conversion
    #[error("Converting Term {term} to Lang")]
    TermAsLang { term: String },

    // ========================================================================
    // Type Expectation Errors
    // ========================================================================
    /// Expected an IRI or blank node in subject/object position, but found a literal.
    ///
    /// # Fields
    /// - `literal`: The literal value that appeared in an invalid position
    #[error("Expected IRI or BlankNode, found literal: {literal}")]
    ExpectedIriOrBlankNodeFoundLiteral { literal: String },

    /// Expected an IRI or blank node, but found an RDF-star quoted triple.
    ///
    /// # Fields
    /// - `subject`: Subject of the unexpected triple term
    /// - `predicate`: Predicate of the unexpected triple term
    /// - `object`: Object of the unexpected triple term
    #[error("Expected IRI or BlankNode, found triple term ({subject},{predicate},{object})")]
    ExpectedIriOrBlankNodeFoundTriple {
        subject: String,
        predicate: String,
        object: String,
    },

    // ========================================================================
    // Comparison and Query Errors
    // ========================================================================
    /// Error comparing two RDF terms.
    ///
    /// # Fields
    /// - `term1`: String representation of the first term
    /// - `term2`: String representation of the second term
    #[error("Comparison error: {term1} with {term2}")]
    ComparisonError { term1: String, term2: String },

    /// Error retrieving triples from an RDF graph or dataset.
    ///
    /// # Fields
    /// - `error`: Detailed description of the triple retrieval failure
    #[error("Obtaining triples from RDF: {error}")]
    ObtainingTriples { error: String },

    /// Error checking whether a specific triple exists in an RDF graph.
    ///
    /// # Fields
    /// - `subject`: Subject of the triple being checked
    /// - `predicate`: Predicate of the triple being checked
    /// - `object`: Object of the triple being checked
    /// - `error`: Detailed description of why the check failed
    #[error(
        "Error checking if RDF contains the triple <{subject}, {predicate}, {object}>: {error}"
    )]
    FailedCheckingAssertion {
        subject: String,
        predicate: String,
        object: String,
        error: String,
    },

    /// Error finding subjects that match a given predicate-object pattern.
    ///
    /// # Fields
    /// - `predicate`: The predicate being queried
    /// - `object`: The object being queried
    /// - `error`: Detailed description of the query failure
    #[error("Error obtaining subjects for predicate {predicate} and object {object}: {error}")]
    ErrorSubjectsFor {
        predicate: String,
        object: String,
        error: String,
    },

    /// Error finding objects that match a given subject-predicate pattern.
    ///
    /// # Fields
    /// - `subject`: The subject being queried
    /// - `predicate`: The predicate being queried
    /// - `error`: Detailed description of the query failure
    #[error("Error obtaining objects for subject {subject} and predicate {predicate}: {error}")]
    ErrorObjectsFor {
        subject: String,
        predicate: String,
        error: String,
    },

    // ========================================================================
    // Output/Serialization Errors
    // ========================================================================
    /// Error formatting SPARQL query results as a table.
    ///
    /// # Fields
    /// - `error`: Detailed description of the table writing failure
    #[error("Writing query results in table: {error}")]
    WritingTableError { error: String },
}
