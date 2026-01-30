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

    /// Error when expecting the focus node to be an IRI or blank node.
    /// # Fields
    /// - `term`: String representation of the focus node that is not an IRI or blank node
    /// - `error`: Detailed description of why the conversion failed
    #[error("Expected focus node to be an IRI or blank node, but found: {term}. Error: {error}")]
    ExpectedIriOrBlankNodeError { term: String, error: String },

    /// Error when expecting a literal but found a different term type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not a literal
    #[error("Expected literal, but found: {term}")]
    ExpectedLiteralError { term: String },

    /// Error when expecting an integer literal but found a different literal type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not an integer/boolean literal
    #[error("Expected integer or boolean literal, but found: {term}")]
    ExpectedIntegerError { term: String },

    /// Error when expecting an IRI but found a different term type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not an IRI
    #[error("Expected IRI, but found: {term}")]
    ExpectedIRIError { term: String },

    /// Error when expecting a concrete literal but found a different literal type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that cannot be converted to concrete literal
    #[error("Expected concrete literal, but found: {term}")]
    ExpectedConcreteLiteralError { term: String },

    /// Error when expecting a numeric literal but found a different literal type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not a numeric literal
    #[error("Expected numeric literal, but found: {term}")]
    ExpectedNumberError { term: String },

    /// Error when expecting a boolean literal but found a different literal type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not a boolean literal
    #[error("Expected boolean literal, but found: {term}")]
    ExpectedBooleanError { term: String },

    /// Error when expecting an object but found a different term type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not an object
    #[error("Expected object, but found: {term}")]
    ExpectedObjectError { term: String },

    /// Error when expecting rdf:nil but found a different term type.
    ///
    /// # Fields
    /// - `term`: String representation of the term that is not an rdf:nil
    #[error("Expected rdf:nil, but found: {term}")]
    ExpectedNilError {term: String},

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

    // ========================================================================
    // Focus Node Errors
    // ========================================================================
    /// Error when a parsing operation requires a focus node but none is set.
    #[error("No focus node set. A focus node is required for this operation.")]
    NoFocusNodeError,

    /// Error when expecting the focus node to be a valid RDF subject.
    ///
    /// # Fields
    /// - `node`: String representation of the focus node that is not a valid subject
    /// - `context`: The operation or context where the subject was expected
    #[error(
        "Expected focus node to be a subject (IRI or blank node), but found: {node} in context: {context}"
    )]
    ExpectedSubjectError { node: String, context: String },

    // ========================================================================
    // Parse Errors
    // ========================================================================
    /// Error when attempting to parse or process an unsupported RDF format.
    ///
    /// # Fields
    /// - `format`: The unsupported format identifier that was requested
    #[error("Format {format} not supported")]
    NotSupportedRDFFormatError { format: String },

    /// Error when both branches of an `or` combinator fail.
    ///
    /// # Fields
    /// - `err1`: The error from the first parser that was tried
    /// - `err2`: The error from the second (fallback) parser
    #[error("Both branches of OR combinator failed. First: {err1}, Second: {err2}")]
    FailedOrError {
        err1: Box<RDFError>,
        err2: Box<RDFError>,
    },

    /// Error when a `not` combinator's inner parser unexpectedly succeeds.
    ///
    /// # Fields
    /// - `value`: String representation of the unexpected successful parse result
    #[error("NOT combinator failed: parser unexpectedly succeeded with value: {value}")]
    FailedNotError { value: String },

    /// Error when an RDF node cannot be parsed as a valid SHACL path.
    ///
    /// # Fields
    /// - `node`: String representation of the RDF node being parsed
    /// - `context`: Where the parsing was attempted (e.g. function name)
    /// - `error`: The underlying parsing error
    #[error("Invalid SHACL path at node {node}: {error}")]
    InvalidSHACLPathError { node: String, error: Box<RDFError> },

    /// Error when an opaque parser fails to execute or provide a valid parser.
    ///
    /// # Fields
    /// - `msg`: A detailed description of why the opaque parser failed
    /// - `context`: Optional context about where in the parsing process the failure occurred
    #[error("Opaque parser failed: {msg}{}", context.as_ref().map(|c| format!(" (context: {})", c)).unwrap_or_default())]
    FailedOpaqueError {
        msg: String,
        context: Option<String>,
    },

    /// Error when a parser explicitly fails with a custom message.
    ///
    /// # Fields
    /// - `msg`: The custom error message describing why the parser failed
    #[error("Parse failed: {msg}")]
    ParseFailError { msg: String },

    /// Error when a conditional validation fails.
    ///
    /// # Fields
    /// - `msg`: The error message describing which condition failed
    #[error("Condition validation failed: {msg}")]
    FailedCondError { msg: String },

    /// Error when parsing an RDF list that contains a cycle.
    ///
    /// # Fields
    /// - `node`: String representation of the node that creates the cycle
    #[error("Recursive RDF list detected: node {node} was already visited, creating a cycle")]
    RecursiveRDFListError { node: String },

    /// Error when attempting to convert a term to an RDF node (Object) fails.
    ///
    /// # Fields
    /// - `term`: String representation of the term that failed conversion
    #[error("Failed to convert term to RDF node: {term}")]
    FailedTermToRDFNodeError { term: String },

    /// Error when attempting to convert an RDF subject to an IRI or blank node.
    ///
    /// # Fields
    /// - `subject`: String representation of the subject that failed conversion
    #[error("Failed to convert subject to IRI or blank node: {subject}")]
    SubjectToIriOrBlankNodeError { subject: String },

    /// Error when expecting the focus node to be a valid RDF subject.
    ///
    /// # Fields
    /// - `focus`: String representation of the focus node that is not a valid subject
    #[error("Expected focus node to be a subject (IRI or blank node), but found: {focus}")]
    ExpectedFocusAsSubjectError { focus: String },

    /// Error when a property has more than one value but exactly one was expected.
    ///
    /// # Fields
    /// - `node`: String representation of the node being queried
    /// - `pred`: The property/predicate that has multiple values
    /// - `value1`: String representation of the first value found
    /// - `value2`: String representation of the second value found
    #[error(
        "Node {node} has more than one value for predicate {pred}: found at least {value1} and {value2}"
    )]
    MoreThanOneValuePredicateError {
        node: String,
        pred: String,
        value1: String,
        value2: String,
    },

    /// Error when a property has no values but at least one was expected.
    ///
    /// # Fields
    /// - `node`: String representation of the node being queried
    /// - `pred`: The property/predicate that has no values
    #[error("Node {node} has no values for predicate {pred}")]
    NoValuesPredicateError { node: String, pred: String },

    /// Error when a property has no values (debug version with neighborhood info).
    ///
    /// # Fields
    /// - `node`: String representation of the node being queried
    /// - `pred`: The property/predicate that has no values
    /// - `outgoing_arcs`: String representation of all outgoing arcs from the node
    #[error("Node {node} has no values for predicate {pred}. Outgoing arcs: {outgoing_arcs}")]
    NoValuesPredicateDebugError {
        node: String,
        pred: String,
        outgoing_arcs: String,
    },

    /// Error when failing to obtain outgoing arcs from a focus node.
    ///
    /// # Fields
    /// - `focus`: String representation of the focus node
    /// - `error`: Detailed description of why obtaining outgoing arcs failed
    #[error("Error obtaining outgoing arcs from node {focus}: {error}")]
    OutgoingArcsError { focus: String, error: String },

    /// Error when a node fails to satisfy a predicate condition.
    ///
    /// # Fields
    /// - `condition_name`: The name/description of the condition that was not satisfied
    /// - `node`: String representation of the node that failed the condition
    #[error("Node {node} does not satisfy condition: {condition_name}")]
    NodeDoesntSatisfyConditionError {
        condition_name: String,
        node: String,
    },

    /// Error when no instances of a specified type are found in the RDF data.
    ///
    /// # Fields
    /// - `object`: The IRI of the expected type/class that has no instances
    #[error("No instances found for type: {object}")]
    FailedInstancesOfError { object: String },
}
