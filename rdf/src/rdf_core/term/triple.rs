use crate::rdf_core::{
    RDFError, Rdf,
    term::{
        Iri, IriOrBlankNode, Term, TermKind,
        literal::{ConcreteLiteral, Lang, NumericLiteral},
    },
};
use iri_s::IriS;
use prefixmap::IriRef;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// Represents an RDF triple.
///
/// An RDF triple consists of three components: a subject, a predicate, and an object.
///
/// # Type Parameters
///
/// * `S` - The subject type, which must implement `Subject`
/// * `P` - The predicate type, which must implement `Iri`
/// * `O` - The object type, which must implement `Term`
pub trait Triple<S, P, O>: Debug + Clone + Display
where
    S: Subject,
    P: Iri,
    O: Term,
{
    /// Constructs a new RDF triple from the given components.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject of the triple, convertible to type `S`
    /// * `pred` - The predicate of the triple, convertible to type `P`
    /// * `obj` - The object of the triple, convertible to type `O`
    fn new(subj: impl Into<S>, pred: impl Into<P>, obj: impl Into<O>) -> Self;

    /// Returns a reference to the subject of this triple.
    fn subj(&self) -> &S;

    /// Returns a reference to the predicate of this triple.
    fn pred(&self) -> &P;

    /// Returns a reference to the object of this triple.
    fn obj(&self) -> &O;

    /// Consumes the triple and returns its components as a tuple.
    ///
    /// This method takes ownership of the triple and returns `(subject, predicate, object)`,
    /// allowing you to extract the individual components without cloning.
    fn into_components(self) -> (S, P, O);

    /// Consumes the triple and returns only the subject.
    fn into_subject(self) -> S {
        self.into_components().0
    }

    /// Consumes the triple and returns only the predicate.
    fn into_predicate(self) -> P {
        self.into_components().1
    }

    /// Consumes the triple and returns only the object.
    fn into_object(self) -> O {
        self.into_components().2
    }
}

/// A concrete implementation of an RDF triple for a specific RDF model.
///
/// # Type Parameters
///
/// * `R` - The RDF implementation type that defines the specific types for
///         subjects, predicates, and objects through its associated types
pub struct ConcreteTriple<R>
where
    R: Rdf,
{
    subj: R::Subject,
    pred: R::IRI,
    obj: R::Term,
}

impl<R> ConcreteTriple<R>
where
    R: Rdf,
{
    /// Creates a new concrete triple from owned components.
    ///
    /// # Parameters
    ///
    /// * `subj` - The subject component from the RDF model `R`
    /// * `pred` - The predicate component from the RDF model `R`
    /// * `obj` - The object component from the RDF model `R`
    pub fn new(subj: R::Subject, pred: R::IRI, obj: R::Term) -> Self {
        ConcreteTriple { subj, pred, obj }
    }

    /// Returns a reference to the subject component.
    pub fn subj(&self) -> &R::Subject {
        &self.subj
    }

    /// Returns a reference to the predicate component.
    pub fn pred(&self) -> &R::IRI {
        &self.pred
    }

    /// Returns a reference to the object component.
    pub fn obj(&self) -> &R::Term {
        &self.obj
    }

    /// Converts this triple from one RDF implementation to another.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target RDF implementation type
    ///
    /// # Trait Bounds
    ///
    /// Requires that the target RDF model's types can be converted from the
    /// source RDF model's types:
    /// - `T::Subject: From<R::Subject>` - Subject conversion
    /// - `T::Term: From<R::Term>` - Term conversion
    /// - `T::IRI: From<R::IRI>` - IRI conversion
    pub fn cnv<T: Rdf>(self) -> ConcreteTriple<T>
    where
        T::Subject: From<R::Subject>,
        T::Term: From<R::Term>,
        T::IRI: From<R::IRI>,
    {
        ConcreteTriple {
            subj: T::Subject::from(self.subj),
            pred: T::IRI::from(self.pred),
            obj: T::Term::from(self.obj),
        }
    }
}

/// Represents the subject position of an RDF triple.
///
/// In RDF, a subject can be an IRI, a blank node, or (in RDF-star) a triple.
/// This trait defines the common behavior for all types that can appear as
/// subjects in RDF statements.
pub trait Subject: Debug + Display + PartialEq + Clone + Eq + Hash {
    /// Returns the kind of RDF term this subject represents.
    ///
    /// This method allows distinguishing between IRIs, blank nodes, and quoted triples at runtime.
    fn kind(&self) -> TermKind;

    /// Returns `true` if this subject is an IRI.
    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    /// Returns `true` if this subject is a blank node.
    fn is_blank_node(&self) -> bool {
        self.kind() == TermKind::BlankNode
    }

    /// Returns `true` if this subject is a quoted triple (RDF-star).
    fn is_triple(&self) -> bool {
        self.kind() == TermKind::Triple
    }
}

/// Represents an RDF object value in the object position of a triple.
///
/// In RDF, the object is the third component of a triple (subject-predicate-object)
/// and can be one of four types:
/// - **IRI**: A resource identified by an Internationalized Resource Identifier
/// - **Blank Node**: An anonymous resource without a global identifier
/// - **Literal**: A concrete value (string, number, date, etc.) with optional datatype/language
/// - **Triple** (RDF-star): A quoted triple that can be nested as an object
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    /// An IRI (Internationalized Resource Identifier) representing a named resource.
    Iri(IriS),
    /// A blank node (anonymous resource) identified by a local label.
    BlankNode(String),
    /// A literal value with a datatype and optional language tag.
    Literal(ConcreteLiteral),
    /// An RDF-star quoted triple that can be used as an object.
    ///
    /// # Fields
    /// - `subject`: The subject of the nested triple (IRI or blank node)
    /// - `predicate`: The predicate of the nested triple (IRI)
    /// - `object`: The object of the nested triple (recursively an Object)
    Triple {
        subject: Box<IriOrBlankNode>,
        predicate: IriS,
        object: Box<Object>,
    },
}

/// ## Constructors methods
impl Object {
    /// Creates an IRI object from an `IriS` instance.
    ///
    /// # Parameters
    /// - `iri`: The IRI to wrap as an object
    pub fn iri(iri: IriS) -> Object {
        Object::Iri(iri)
    }

    /// Creates a blank node object from a string identifier.
    ///
    /// # Parameters
    /// - `str`: The blank node identifier
    pub fn bnode(str: String) -> Object {
        Object::BlankNode(str)
    }

    /// Creates a literal object from a concrete literal value.
    ///
    /// # Parameters
    /// - `lit`: The concrete literal to wrap as an object
    pub fn literal(lit: ConcreteLiteral) -> Object {
        Object::Literal(lit)
    }

    /// Creates a string literal object from a string slice.
    ///
    /// # Parameters
    /// - `str`: The string value for the literal
    pub fn str(str: &str) -> Object {
        Object::Literal(ConcreteLiteral::str(str))
    }

    /// Creates a boolean literal object.
    ///
    /// # Parameters
    /// - `b`: The boolean value
    pub fn boolean(b: bool) -> Object {
        Object::Literal(ConcreteLiteral::boolean(b))
    }
}

/// ## Accessors methods
impl Object {
    // Returns the length (in bytes) of this object's string representation.
    ///
    /// - For IRIs: the length of the IRI string
    /// - For blank nodes: the length of the identifier
    /// - For literals: the length of the lexical form
    /// - For triples: the sum of all component lengths
    pub fn length(&self) -> usize {
        match self {
            Object::Iri(iri) => iri.as_str().len(),
            Object::BlankNode(bn) => bn.len(),
            Object::Literal(lit) => lit.lexical_form().len(),
            Object::Triple {
                subject,
                predicate,
                object,
            } => subject.as_ref().length() + predicate.as_str().len() + object.as_ref().length(),
        }
    }

    /// Extracts the numeric value if this is a numeric literal.
    ///
    /// # Returns
    /// - `Some(NumericLiteral)` if this is a numeric literal (integer, decimal, float, double)
    /// - `None` if this is not a literal or not a numeric type
    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Object::Literal(lit) => lit.numeric_value(),
            _ => None,
        }
    }

    /// Returns the datatype IRI of this object if it's a literal.
    /// # Returns
    /// - `Some(IriRef)` if this is a literal
    /// - `None` if this is an IRI, blank node, or triple
    pub fn datatype(&self) -> Option<IriRef> {
        match self {
            Object::Literal(lit) => Some(lit.datatype()),
            _ => None,
        }
    }

    /// Returns the language tag if this is a language-tagged string literal.
    ///
    /// # Returns
    /// - `Some(&Lang)` if this is a string literal with a language tag (e.g., "en", "es-MX")
    /// - `None` if this is not a language-tagged literal
    pub fn lang(&self) -> Option<&Lang> {
        match self {
            Object::Literal(ConcreteLiteral::StringLiteral {
                lang: Some(lang), ..
            }) => Some(lang),
            _ => None,
        }
    }
}

impl Object {
    /// ## Parsing methods
    /// Parses a string into an RDF object, with optional base IRI resolution.
    ///
    /// This method attempts to parse:
    /// - Blank nodes: strings starting with "_:"
    /// - Literals: strings starting with '"' (not yet implemented)
    /// - IRIs: all other strings, resolved against the base if provided
    ///
    /// # Parameters
    /// - `str`: The string to parse
    /// - `base`: Optional base IRI for resolving relative IRI references
    ///
    /// # Errors
    /// - `RDFError::ParsingIri` if IRI parsing fails
    pub fn parse(str: &str, base: Option<&str>) -> Result<Object, RDFError> {
        if let Some(bnode_id) = str.strip_prefix("_:") {
            Ok(Object::bnode(bnode_id.to_string()))
        } else if str.starts_with('"') {
            todo!()
        } else {
            let iri = IriS::from_str_base(str, base).map_err(|e| RDFError::ParsingIri {
                iri: str.to_string(),
                error: e.to_string(),
            })?;
            Ok(Object::iri(iri))
        }
    }
}

/// ## Formatting methods
impl Object {
    /// Formats this object using qualified names (prefixes) where possible.
    ///
    /// This method produces a compact representation by replacing full IRIs
    /// with prefixed names (e.g., "rdf:type" instead of "http://www.w3.org/1999/02/22-rdf-syntax-ns#type").
    ///
    /// # Parameters
    /// - `prefixmap`: A prefix map containing IRI-to-prefix mappings
    pub fn show_qualified(&self, prefixmap: &prefixmap::PrefixMap) -> String {
        match self {
            Object::Iri(iri) => prefixmap.qualify(iri),
            Object::BlankNode(bnode) => format!("_:{bnode}"),
            Object::Literal(lit) => lit.show_qualified(prefixmap),
            Object::Triple {
                subject,
                predicate,
                object,
            } => format!(
                "<< {} {} {} >>",
                subject.show_qualified(prefixmap),
                prefixmap.qualify(predicate),
                object.show_qualified(prefixmap)
            ),
        }
    }
}

// ============================================================================
// Trait Implementations - Conversions
// ============================================================================

/// Converts an `IriS` into an `Object::Iri`.
///
/// This allows IRIs to be seamlessly used where objects are expected.
impl From<IriS> for Object {
    fn from(iri: IriS) -> Self {
        Object::Iri(iri)
    }
}

/// Converts a `ConcreteLiteral` into an `Object::Literal`.
///
/// This allows literals to be seamlessly used where objects are expected.
impl From<ConcreteLiteral> for Object {
    fn from(lit: ConcreteLiteral) -> Self {
        Object::Literal(lit)
    }
}

/// Converts an `Object` into an `oxrdf::Term`.
///
/// This enables interoperability with the `oxrdf` library by converting
/// the custom `Object` representation into oxrdf's term type.
impl From<Object> for oxrdf::Term {
    fn from(value: Object) -> Self {
        match value {
            Object::Iri(iri_s) => oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into(),
            Object::BlankNode(bnode) => oxrdf::BlankNode::new_unchecked(bnode).into(),
            Object::Literal(literal) => oxrdf::Term::Literal(literal.into()),
            Object::Triple { .. } => todo!(),
        }
    }
}

/// Attempts to convert an `oxrdf::Term` into an `Object`.
///
/// This enables interoperability with the `oxrdf` library by converting
/// oxrdf's term type into the custom `Object` representation.
/// # Errors
/// Returns `RDFError` if the conversion fails (e.g., invalid literal format).
impl TryFrom<oxrdf::Term> for Object {
    type Error = RDFError;

    fn try_from(value: oxrdf::Term) -> Result<Self, Self::Error> {
        match value {
            oxrdf::Term::NamedNode(named_node) => {
                Ok(Object::iri(named_node.into()))
            }
            oxrdf::Term::BlankNode(blank_node) => Ok(Object::bnode(blank_node.into_string())),
            oxrdf::Term::Literal(literal) => {
                let lit: ConcreteLiteral = literal.try_into()?;
                Ok(Object::literal(lit))
            }
            oxrdf::Term::Triple(triple) => {
                let (s, p, o) = triple.into_components();
                let object = Object::try_from(o)?;
                let subject = IriOrBlankNode::from(s);
                let predicate = p.into();
                Ok(Object::Triple {
                    subject: Box::new(subject),
                    predicate,
                    object: Box::new(object),
                })
            }
        }
    }
}

/// Attempts to convert an `Object` into an `oxrdf::NamedOrBlankNode`.
///
/// This conversion is used when an object appears in subject position
/// (which can only be IRIs or blank nodes, not literals).
///
/// # Errors
/// Returns `RDFError` for objects that cannot be subjects (literals and triples).
impl TryFrom<Object> for oxrdf::NamedOrBlankNode {
    type Error = RDFError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        println!("Trying from Object: {value}");
        match value {
            Object::Iri(iri_s) => Ok(oxrdf::NamedNode::new_unchecked(iri_s.as_str()).into()),
            Object::BlankNode(bnode) => Ok(oxrdf::BlankNode::new_unchecked(bnode).into()),
            Object::Literal(_) => todo!(),
            Object::Triple { .. } => todo!(),
        }
    }
}

// ============================================================================
// Trait Implementations - Default, Display, Debug
// ============================================================================

impl<R> Display for ConcreteTriple<R>
where
    R: Rdf,
{
    /// Formats the triple as a string.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter to write to
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>", self.subj, self.pred, self.obj)
    }
}

impl Default for Object {
    /// Provides a default `Object` value (empty IRI).
    fn default() -> Self {
        Object::Iri(IriS::default())
    }
}

impl Display for Object {
    /// Formats the object for display (human-readable output).
    ///
    /// - IRIs: displayed as-is
    /// - Blank nodes: prefixed with "_:"
    /// - Literals: uses the literal's Display implementation
    /// - Triples: not yet implemented
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri(iri) => write!(f, "{iri}"),
            Object::BlankNode(bnode) => write!(f, "_:{bnode}"),
            Object::Literal(lit) => write!(f, "{lit}"),
            Object::Triple { .. } => todo!(),
        }
    }
}

impl Debug for Object {
    /// Formats the object for debugging (verbose output with type information).
    ///
    /// Includes type tags for each variant:
    /// - "Iri {<iri>}"
    /// - "Bnode{<id>}"
    /// - "Literal{<value>}"
    /// - "Triple {<s>, <p>, <o>}"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Iri(iri) => write!(f, "Iri {{{iri:?}}}"),
            Object::BlankNode(bnode) => write!(f, "Bnode{{{bnode:?}}}"),
            Object::Literal(lit) => write!(f, "Literal{{{lit:?}}}"),
            Object::Triple {
                subject,
                predicate,
                object,
            } => write!(f, "Triple {{{subject:?}, {predicate:?}, {object:?}}}"),
        }
    }
}

// ============================================================================
// Trait Implementations - Ordering
// ============================================================================

impl PartialOrd for Object {
    /// Implements partial ordering for objects.
    ///
    /// Since `Object` implements total ordering via [`Ord`], this always returns
    /// `Some(ordering)`.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    /// Implements total ordering for objects according to RDF semantics.
    ///
    /// The ordering priority is: IRIs < Blank Nodes < Literals.
    /// Within each category, standard comparison applies:
    /// - IRIs: lexicographic ordering of IRI strings
    /// - Blank nodes: lexicographic ordering of identifiers
    /// - Literals: ordering defined by `ConcreteLiteral::cmp`
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Object::Iri(a), Object::Iri(b)) => a.cmp(b),
            (Object::BlankNode(a), Object::BlankNode(b)) => a.cmp(b),
            (Object::Literal(a), Object::Literal(b)) => a.cmp(b),
            (Object::Iri(_), _) => std::cmp::Ordering::Less,
            (Object::BlankNode(_), Object::Iri(_)) => std::cmp::Ordering::Greater,
            (Object::BlankNode(_), Object::Literal(_)) => std::cmp::Ordering::Less,
            (Object::Literal(_), _) => std::cmp::Ordering::Greater,
            (
                Object::BlankNode(_),
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::Iri(_iri_s),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::BlankNode(_),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _,
                    predicate: _,
                    object: _,
                },
                Object::Literal(_sliteral),
            ) => todo!(),
            (
                Object::Triple {
                    subject: _subject1,
                    predicate: _predicate1,
                    object: _object1,
                },
                Object::Triple {
                    subject: _subject2,
                    predicate: _predicate2,
                    object: _object2,
                },
            ) => todo!(),
        }
    }
}
