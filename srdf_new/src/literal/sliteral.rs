use std::{hash::Hash, result, fmt::{self, Debug, Display}};

use crate::{error::RDFError, literal::{Lang, NumericLiteral, XsdDateTime}};
use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use tracing::trace;

/// Concrete representation of RDF literals with type-safe internal representations.
///
/// This enum provides a strongly-typed representation of RDF literals, using native
/// Rust types (integers, floats, booleans, etc.) internally for efficiency. It also
/// supports literals with incorrect datatypes to enable parsing and validation of
/// potentially malformed RDF data.
///
/// # Type Safety
///
/// The enum uses native Rust types internally, providing type safety and efficient
/// operations on numeric values. For example, `NumericLiteral` stores actual numeric
/// values rather than strings, enabling direct mathematical operations.
///
/// # Error Handling
///
/// The [`WrongDatatypeLiteral`](Self::WrongDatatypeLiteral) variant allows parsing
/// and representing malformed RDF data (e.g., `"hello"^^xsd:integer`) without losing
/// information. This enables validation workflows that need to report specific errors
/// while continuing to process other data.
///
/// # Comparison and Ordering
///
/// Literals implement [`PartialOrd`] and [`Ord`] following SPARQL ordering rules:
/// - String literals are compared lexicographically
/// - Numeric literals are compared by numeric value
/// - Boolean literals follow `false < true`
/// - Datetime literals are compared chronologically
///
/// # Panics
///
/// The [`Ord`] implementation panics when comparing incomparable literals (e.g., NaN
/// floating-point values or literals with different datatypes). Use [`PartialOrd`]
/// when comparing arbitrary literals to avoid panics.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum SLiteral {
    /// A plain string literal, optionally with a language tag.
    StringLiteral {
        lexical_form: String,
        lang: Option<Lang>,
    },
    
    /// A literal with an explicit datatype IRI.
    DatatypeLiteral {
        lexical_form: String,
        datatype: IriRef,
    },
    
    /// A numeric literal (integer, float, decimal, etc.).
    NumericLiteral(NumericLiteral),
    
    /// An XSD datetime literal.
    DatetimeLiteral(XsdDateTime),
    
    /// A boolean literal (true or false).
    #[serde(serialize_with = "serialize_boolean_literal")]
    BooleanLiteral(bool),
    
    /// Represents a literal with an invalid datatype.
    ///
    /// For example, a value like `"hello"^^xsd:integer` would be represented as a
    /// `WrongDatatypeLiteral`. This is useful for parsing RDF data that may contain
    /// malformed literals while still enabling validation.
    WrongDatatypeLiteral {
        lexical_form: String,
        datatype: IriRef,
        error: String,
    },
}

/// ## Display and formatting
impl SLiteral {
    /// Returns a string representation using the given prefix map to qualify IRIs.
    ///
    /// This method formats the literal using shortened IRI prefixes from the provided
    /// prefix map, making the output more readable.
    ///
    /// # Arguments
    ///
    /// * `prefixmap` - The prefix map used to shorten IRIs
    ///
    /// # Examples
    ///
    /// ```
    /// use srdf_new::SLiteral;
    /// use prefixmap::PrefixMap;
    ///
    /// let lit = SLiteral::integer(42);
    /// let prefixmap = PrefixMap::basic();
    /// println!("{}", lit.show_qualified(&prefixmap));
    /// ```
    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        trace!("Showing qualified literal: {self:?} with prefixmap: {prefixmap:?}");
        
        struct QualifiedDisplay<'a> {
            literal: &'a SLiteral,
            prefixmap: &'a PrefixMap,
        }

        impl<'a> Display for QualifiedDisplay<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.literal.display_qualified(f, self.prefixmap)
            }
        }
        
        format!("{}", QualifiedDisplay { literal: self, prefixmap })
    }

    /// Formats this literal using the given prefix map.
    ///
    /// This method is used internally by `show_qualified` and `Display`.
    pub fn display_qualified(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefixmap: &PrefixMap,
    ) -> fmt::Result {
        match self {
            Self::StringLiteral { lexical_form, lang: None } => {
                write!(f, "\"{lexical_form}\"")
            }
            Self::StringLiteral { lexical_form, lang: Some(lang) } => {
                write!(f, "\"{lexical_form}\"{lang}")
            }
            Self::DatatypeLiteral { lexical_form, datatype } => {
                self.format_datatype_literal(f, lexical_form, datatype, prefixmap)
            }
            Self::NumericLiteral(n) => write!(f, "{n}"),
            Self::BooleanLiteral(b) => write!(f, "{b}"),
            Self::DatetimeLiteral(dt) => write!(f, "{}", dt.value()),
            Self::WrongDatatypeLiteral { lexical_form, datatype, .. } => {
                self.format_datatype_literal(f, lexical_form, datatype, prefixmap)
            }
        }
    }

    /// Helper method to format datatype literals consistently.
    fn format_datatype_literal(
        &self,
        f: &mut fmt::Formatter<'_>,
        lexical_form: &str,
        datatype: &IriRef,
        prefixmap: &PrefixMap,
    ) -> fmt::Result {
        match datatype {
            IriRef::Iri(iri) => {
                write!(f, "\"{lexical_form}\"^^{}", prefixmap.qualify(iri))
            }
            IriRef::Prefixed { prefix, local } => {
                write!(f, "\"{lexical_form}\"^^{prefix}:{local}")
            }
        }
    }
}

/// ## Validation and Conversion
impl SLiteral {
    /// Validates that the lexical form matches the declared datatype.
    ///
    /// Returns a new literal with proper typing if valid, or a `WrongDatatypeLiteral`
    /// if the lexical form doesn't match the datatype.
    ///
    /// # Errors
    ///
    /// Returns an `RDFError` if validation fails.
    /// ```
    pub fn as_checked_literal(&self) -> Result<Self, RDFError> {
        match self {
            Self::DatatypeLiteral { lexical_form, datatype } => {
                check_literal_datatype(lexical_form, datatype)
            }
            _ => Ok(self.clone()),
        }
    }

    /// Compares this literal with another for equality.
    ///
    /// This method performs type-aware comparison, ensuring that literals of
    /// different types are not considered equal even if their lexical forms match.
    ///
    /// # Arguments
    ///
    /// * `literal_expected` - The literal to compare against
    ///
    /// # Returns
    ///
    /// `true` if the literals are equal, `false` otherwise.
    pub fn match_literal(&self, literal_expected: &Self) -> bool {
        let result = match (self, literal_expected) {
            (
                Self::StringLiteral { lexical_form, lang },
                Self::StringLiteral {
                    lexical_form: expected_lexical_form,
                    lang: expected_lang,
                },
            ) => {
                trace!(
                    "Comparing string literals: {lexical_form} ({lang:?}) with \
                     expected {expected_lexical_form} ({expected_lang:?})"
                );
                lexical_form == expected_lexical_form && lang == expected_lang
            }
            (
                Self::DatatypeLiteral { lexical_form, datatype },
                Self::DatatypeLiteral {
                    lexical_form: expected_lexical_form,
                    datatype: expected_datatype,
                },
            ) => lexical_form == expected_lexical_form && datatype == expected_datatype,
            (Self::NumericLiteral(n1), Self::NumericLiteral(n2)) => n1 == n2,
            (Self::DatetimeLiteral(dt1), Self::DatetimeLiteral(dt2)) => dt1 == dt2,
            (Self::BooleanLiteral(b1), Self::BooleanLiteral(b2)) => b1 == b2,
            (
                Self::WrongDatatypeLiteral { lexical_form, datatype, .. },
                Self::WrongDatatypeLiteral {
                    lexical_form: expected_lexical_form,
                    datatype: expected_datatype,
                    ..
                },
            ) => lexical_form == expected_lexical_form && datatype == expected_datatype,
            _ => false,
        };
        
        trace!("match_literal: self: {self}, expected: {literal_expected}: {result}");
        result
    }
}

/// ## Constructor Methods - Numeric Types
impl SLiteral {
    /// Creates an integer literal.
    #[inline]
    pub fn integer(n: isize) -> Self {
        Self::NumericLiteral(NumericLiteral::integer(n))
    }

    /// Creates a non-negative integer literal.
    #[inline]
    pub fn non_negative_integer(n: usize) -> Self {
        Self::NumericLiteral(NumericLiteral::non_negative_integer(n))
    }

    /// Creates a non-positive integer literal.
    #[inline]
    pub fn non_positive_integer(n: isize) -> Self {
        Self::NumericLiteral(NumericLiteral::non_positive_integer(n))
    }

    /// Creates a positive integer literal.
    #[inline]
    pub fn positive_integer(n: usize) -> Self {
        Self::NumericLiteral(NumericLiteral::positive_integer(n))
    }

    /// Creates a negative integer literal.
    #[inline]
    pub fn negative_integer(n: isize) -> Self {
        Self::NumericLiteral(NumericLiteral::negative_integer(n))
    }

    /// Creates a double literal.
    #[inline]
    pub fn double(d: f64) -> Self {
        Self::NumericLiteral(NumericLiteral::double(d))
    }

    /// Creates a decimal literal.
    #[inline]
    pub fn decimal(d: Decimal) -> Self {
        Self::NumericLiteral(NumericLiteral::decimal(d))
    }

    /// Creates a long literal.
    #[inline]
    pub fn long(n: isize) -> Self {
        Self::NumericLiteral(NumericLiteral::long(n))
    }

    /// Creates an unsigned byte literal.
    #[inline]
    pub fn unsigned_byte(n: u8) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_byte(n))
    }

    /// Creates an unsigned short literal.
    #[inline]
    pub fn unsigned_short(n: u16) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_short(n))
    }

    /// Creates an unsigned int literal.
    #[inline]
    pub fn unsigned_int(n: u32) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_int(n))
    }

    /// Creates an unsigned long literal.
    #[inline]
    pub fn unsigned_long(n: u64) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_long(n))
    }

    /// Creates a byte literal.
    #[inline]
    pub fn byte(n: i8) -> Self {
        Self::NumericLiteral(NumericLiteral::byte(n))
    }

    /// Creates a float literal.
    #[inline]
    pub fn float(n: f64) -> Self {
        Self::NumericLiteral(NumericLiteral::float(n))
    }
}

/// ## Constructor Methods - Other Types
impl SLiteral {
    /// Creates a datatype literal with the given lexical form and datatype IRI.
    pub fn lit_datatype(lexical_form: &str, datatype: &IriRef) -> Self {
        Self::DatatypeLiteral {
            lexical_form: lexical_form.to_owned(),
            datatype: datatype.clone(),
        }
    }

    /// Creates a boolean literal.
    #[inline]
    pub fn boolean(b: bool) -> Self {
        Self::BooleanLiteral(b)
    }

    /// Creates a plain string literal without a language tag.
    pub fn str(lexical_form: &str) -> Self {
        Self::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: None,
        }
    }

    /// Creates a string literal with a language tag.
    pub fn lang_str(lexical_form: &str, lang: Lang) -> Self {
        Self::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: Some(lang),
        }
    }
}

/// ## Accessor Methods
impl SLiteral {
    /// Returns the language tag if this is a language-tagged string literal.
    pub fn lang(&self) -> Option<Lang> {
        match self {
            Self::StringLiteral { lang, .. } => lang.clone(),
            _ => None,
        }
    }

    /// Returns the lexical form (string representation) of this literal.
    pub fn lexical_form(&self) -> String {
        match self {
            Self::StringLiteral { lexical_form, .. }
            | Self::DatatypeLiteral { lexical_form, .. }
            | Self::WrongDatatypeLiteral { lexical_form, .. } => lexical_form.clone(),
            Self::NumericLiteral(nl) => nl.lexical_form(),
            Self::BooleanLiteral(b) => b.to_string(),
            Self::DatetimeLiteral(dt) => dt.to_string(),
        }
    }

    /// Returns the datatype IRI of this literal.
    ///
    /// For string literals without language tags, returns `xsd:string`.
    /// For language-tagged strings, returns `rdf:langString`.
    pub fn datatype(&self) -> IriRef {
        match self {
            Self::DatatypeLiteral { datatype, .. }
            | Self::WrongDatatypeLiteral { datatype, .. } => datatype.clone(),
            Self::StringLiteral { lang: None, .. } => {
                IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#string"))
            }
            Self::StringLiteral { lang: Some(_), .. } => {
                IriRef::iri(IriS::new_unchecked(
                    "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"
                ))
            }
            Self::NumericLiteral(nl) => IriRef::iri(IriS::new_unchecked(nl.datatype())),
            Self::BooleanLiteral(_) => {
                IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#boolean"))
            }
            Self::DatetimeLiteral(_) => {
                IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#dateTime"))
            }
        }
    }

    /// Returns the numeric value if this is a numeric literal.
    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Self::NumericLiteral(nl) => Some(nl.clone()),
            _ => None,
        }
    }
}

/// ## Parsing Methods
impl SLiteral {
    /// Parses a boolean from its XSD lexical representation.
    ///
    /// Valid values are: "true", "false", "1" (true), "0" (false).
    /// The parsing is case-sensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid boolean representation.
    /// 
    /// # Examples
    ///
    /// ```
    /// use srdf_new::SLiteral;
    /// assert_eq!(SLiteral::parse_bool("true").unwrap(), true);
    /// assert_eq!(SLiteral::parse_bool("0").unwrap(), false);
    /// assert!(SLiteral::parse_bool("yes").is_err());
    /// ```
    pub fn parse_bool(s: &str) -> Result<bool, String> {
        match s {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(format!("Cannot convert '{s}' to boolean. Expected 'true', 'false', '1', or '0'")),
        }
    }

    /// Parses an integer from its string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use srdf_new::SLiteral;
    /// assert_eq!(SLiteral::parse_integer("-7").unwrap(), -7);
    /// assert_eq!(SLiteral::parse_integer("2").unwrap(), 2);
    /// assert!(SLiteral::parse_integer("x").is_err());
    /// ```
    pub fn parse_integer(s: &str) -> Result<isize, String> {
        s.parse::<isize>()
            .map_err(|_| format!("Cannot convert '{s}' to integer"))
    }

    /// Parses a negative integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not negative or cannot be parsed.
    pub fn parse_negative_integer(s: &str) -> Result<isize, String> {
        let value = s.parse::<isize>()
            .map_err(|_| format!("Cannot convert '{s}' to negative integer"))?;
        
        if value < 0 {
            Ok(value)
        } else {
            Err(format!("Value '{s}' is not a negative integer"))
        }
    }

    /// Parses a non-positive integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is positive or cannot be parsed.
    pub fn parse_non_positive_integer(s: &str) -> Result<isize, String> {
        let value = s.parse::<isize>()
            .map_err(|_| format!("Cannot convert '{s}' to non-positive integer"))?;
        
        if value <= 0 {
            Ok(value)
        } else {
            Err(format!("Value '{s}' is not a non-positive integer"))
        }
    }

    /// Parses a positive integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not positive or cannot be parsed.
    pub fn parse_positive_integer(s: &str) -> Result<usize, String> {
        let value = s.parse::<usize>()
            .map_err(|_| format!("Cannot convert '{s}' to positive integer"))?;
        
        if value > 0 {
            Ok(value)
        } else {
            Err(format!("Value '{s}' is not a positive integer (must be > 0)"))
        }
    }

    /// Parses a non-negative integer from its string representation.
    pub fn parse_non_negative_integer(s: &str) -> Result<usize, String> {
        s.parse::<usize>()
            .map_err(|e| format!("Cannot convert '{s}' to non-negative integer: {e}"))
    }

    /// Parses an unsigned byte from its string representation.
    pub fn parse_unsigned_byte(s: &str) -> Result<u8, String> {
        s.parse::<u8>()
            .map_err(|_| format!("Cannot convert '{s}' to unsigned byte (0-255)"))
    }

    /// Parses an unsigned short from its string representation.
    pub fn parse_unsigned_short(s: &str) -> Result<u16, String> {
        s.parse::<u16>()
            .map_err(|_| format!("Cannot convert '{s}' to unsigned short (0-65535)"))
    }

    /// Parses an unsigned int from its string representation.
    pub fn parse_unsigned_int(s: &str) -> Result<u32, String> {
        s.parse::<u32>()
            .map_err(|_| format!("Cannot convert '{s}' to unsigned int"))
    }

    /// Parses an unsigned long from its string representation.
    pub fn parse_unsigned_long(s: &str) -> Result<u64, String> {
        s.parse::<u64>()
            .map_err(|_| format!("Cannot convert '{s}' to unsigned long"))
    }

    /// Parses a double from its string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use srdf_new::SLiteral;
    /// let d = SLiteral::parse_double("3.14").unwrap();
    /// assert!((d - 3.14).abs() < 1e-12);
    /// ```
    pub fn parse_double(s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|_| format!("Cannot convert '{s}' to double"))
    }

    /// Parses a long from its string representation.
    pub fn parse_long(s: &str) -> Result<isize, String> {
        s.parse::<isize>()
            .map_err(|_| format!("Cannot convert '{s}' to long"))
    }

    /// Parses a decimal from its string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use srdf_new::SLiteral;
    /// use rust_decimal::Decimal;
    /// let dec = SLiteral::parse_decimal("10.5").unwrap();
    /// assert_eq!(dec, Decimal::new(105, 1));
    /// ```
    pub fn parse_decimal(s: &str) -> Result<Decimal, String> {
        s.parse::<Decimal>()
            .map_err(|_| format!("Cannot convert '{s}' to decimal"))
    }

    /// Parses a float from its string representation.
    pub fn parse_float(s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|_| format!("Cannot convert '{s}' to float"))
    }

    /// Parses a byte from its string representation.
    pub fn parse_byte(s: &str) -> Result<i8, String> {
        s.parse::<i8>()
            .map_err(|_| format!("Cannot convert '{s}' to byte (-128 to 127)"))
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl Default for SLiteral {
    /// Returns an empty string literal without a language tag.
    ///
    /// This is used as a neutral default value when a literal is required
    /// but no concrete value is available.
    fn default() -> Self {
        Self::StringLiteral {
            lexical_form: String::default(),
            lang: None,
        }
    }
}

/// Partial ordering for literals following SPARQL comparison semantics.
///
/// Comparison rules:
/// - String literals are compared lexicographically by their lexical form.
/// - Datatype literals are comparable **only if** they share the same datatype,
///   and are then compared by lexical form.
/// - Numeric literals are compared by numeric value.
/// - Boolean literals follow `true > false`.
/// - Datetime literals are compared chronologically.
///
/// If two literals are not comparable under these rules, `None` is returned.
///
/// See: <https://www.w3.org/TR/sparql11-query/#OperatorMapping>
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for SLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {    
        match (self, other) {
            // Chronological comparison for datetime literals
            (Self::DatetimeLiteral(dt1), Self::DatetimeLiteral(dt2)) => {
                dt1.partial_cmp(dt2)
            }
            // Lexicographic comparison for plain string literals
            (
                Self::StringLiteral { lexical_form: lf1, .. },
                Self::StringLiteral { lexical_form: lf2, .. },
            ) => Some(lf1.cmp(lf2)),
            // Datatype literals are only comparable if their datatypes match
            (
                Self::DatatypeLiteral { lexical_form: lf1, datatype: dt1 },
                Self::DatatypeLiteral { lexical_form: lf2, datatype: dt2 },
            ) if dt1 == dt2 => Some(lf1.cmp(lf2)),
            // Numeric comparison (may return None for NaN)
            (Self::NumericLiteral(n1), Self::NumericLiteral(n2)) => n1.partial_cmp(n2),
            // Boolean ordering: false < true
            (Self::BooleanLiteral(b1), Self::BooleanLiteral(b2)) => Some(b1.cmp(b2)),
            // Wrong-datatype literals can still be compared lexically if the expected datatype matches
            (
                Self::WrongDatatypeLiteral { lexical_form: lf1, datatype: dt1, .. },
                Self::DatatypeLiteral { lexical_form: lf2, datatype: dt2 },
            ) if dt1 == dt2 => Some(lf1.cmp(lf2)),
            // All other combinations are considered incomparable
            _ => None,
        }
    }
}

/// Total ordering for literals.
///
/// # Panics
///
/// This implementation **panics** if two literals are not comparable, such as:
/// - Literals with different datatypes
/// - Numeric literals involving `NaN`
///
/// This is intended as a temporary solution to support sorting in validation
/// workflows where such cases are not expected.
///
/// # TODO
///
/// Define a total ordering that is well-defined for *all* literals.
impl Ord for SLiteral {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("Cannot compare literals {self} and {other}"))
    }
}

impl Display for SLiteral {
    /// Formats the literal using a basic prefix map for qualified display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_qualified(f, &PrefixMap::basic())
    }
}

impl Deref for SLiteral {
    /// Resolves IRIs and prefixes contained in the literal.
    ///
    /// This operation:
    /// - Clones value-based literals directly
    /// - Dereferences datatype IRIs using the provided base and prefix map
    /// - Converts wrong-datatype literals into properly typed literals
    ///
    /// # Errors
    ///
    /// Returns a `DerefError` if datatype resolution fails.
    fn deref(
        &self,
        base: &Option<IriS>,
        prefixmap: &Option<PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            Self::NumericLiteral(n) => Ok(Self::NumericLiteral(n.clone())),
            Self::BooleanLiteral(b) => Ok(Self::BooleanLiteral(*b)),
            Self::StringLiteral { lexical_form, lang } => Ok(Self::StringLiteral {
                lexical_form: lexical_form.clone(),
                lang: lang.clone(),
            }),
            Self::DatatypeLiteral { lexical_form, datatype } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(Self::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            }
            Self::DatetimeLiteral(dt) => Ok(Self::DatetimeLiteral(dt.clone())),
            Self::WrongDatatypeLiteral { lexical_form, datatype, .. } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(Self::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            }
        }
    }
}

// ============================================================================
// Conversions
// ============================================================================

impl TryFrom<oxrdf::Literal> for SLiteral {
    type Error = RDFError;

    /// Attempts to convert an RDF literal into an `SLiteral`.
    ///
    /// Supported cases:
    /// - Plain string literals
    /// - Language-tagged string literals
    /// - Typed literals (with datatype parsing)
    ///
    /// # Errors
    ///
    /// Returns an `RDFError` if:
    /// - The language tag is invalid
    /// - The datatype is unsupported or malformed
    /// - The literal structure is unknown
    fn try_from(value: oxrdf::Literal) -> Result<Self, Self::Error> {
        let value_str = value.to_string();
        
        match value.destruct() {
            (s, None, None, None) => Ok(Self::str(&s)),
            
            (s, None, Some(language), None) => {
                let lang_str = language.to_string();
                Lang::new(language)
                    .map(|lang| Self::lang_str(&s, lang))
                    .map_err(|e| RDFError::LanguageTagError {
                        literal: value_str,
                        language: lang_str,
                        error: e.to_string(),
                    })
            }
            
            (value, Some(dtype), None, None) => {
                parse_typed_literal(&value, &dtype)
            }
            
            _ => Err(RDFError::ConversionError {
                msg: format!("Unknown literal value: {value_str}"),
            }),
        }
    }
}

impl From<SLiteral> for oxrdf::Literal {
     /// Converts an `SLiteral` into an oxrdf literal.
    ///
    /// If datatype resolution fails, the literal gracefully degrades into
    /// a plain string literal.
    fn from(value: SLiteral) -> Self {
        match value {
            SLiteral::StringLiteral { lexical_form, lang } => match lang {
                Some(lang) => oxrdf::Literal::new_language_tagged_literal_unchecked(
                    lexical_form,
                    lang.to_string(),
                ),
                None => lexical_form.into(),
            },
            SLiteral::DatatypeLiteral { lexical_form, datatype } => {
                datatype.get_iri()
                    .map(|dt| oxrdf::Literal::new_typed_literal(
                        lexical_form.clone(),
                        dt.as_named_node().to_owned(),
                    ))
                    .unwrap_or_else(|_| lexical_form.into())
            }
            SLiteral::NumericLiteral(number) => number.into(),
            SLiteral::BooleanLiteral(b) => b.into(),
            SLiteral::DatetimeLiteral(dt) => (*dt.value()).into(),
            SLiteral::WrongDatatypeLiteral { lexical_form, datatype, .. } => {
                datatype.get_iri()
                    .map(|dt| oxrdf::Literal::new_typed_literal(
                        lexical_form.clone(),
                        dt.as_named_node().to_owned(),
                    ))
                    .unwrap_or_else(|_| lexical_form.into())
            }
        }
    }
}

impl From<&SLiteral> for oxrdf::Literal {
    /// Converts a borrowed literal by cloning it first.
    fn from(value: &SLiteral) -> Self {
        oxrdf::Literal::from(value.clone())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Custom serializer for boolean literals to ensure consistent string output.
fn serialize_boolean_literal<S>(value: &bool, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *value { "true" } else { "false" })
}

/// Parses a typed literal from oxrdf format.
fn parse_typed_literal(value: &str, dtype: &oxrdf::NamedNode) -> Result<SLiteral, RDFError> {
    use oxrdf::vocab::xsd;
    
    let datatype_iri = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
    
    // Helper macro to reduce repetition
    macro_rules! parse_or_wrong {
        ($parser:expr, $constructor:expr) => {
            match $parser(value) {
                Ok(val) => Ok($constructor(val)),
                Err(e) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: value.to_string(),
                    datatype: datatype_iri.clone(),
                    error: e,
                }),
            }
        };
    }
    
    match dtype {
        d if *d == xsd::BOOLEAN => {
            parse_or_wrong!(SLiteral::parse_bool, SLiteral::BooleanLiteral)
        }
        d if *d == xsd::DOUBLE => {
            parse_or_wrong!(SLiteral::parse_double, SLiteral::double)
        }
        d if *d == xsd::DECIMAL => {
            parse_or_wrong!(SLiteral::parse_decimal, SLiteral::decimal)
        }
        d if *d == xsd::FLOAT => {
            parse_or_wrong!(SLiteral::parse_float, SLiteral::float)
        }
        d if *d == xsd::LONG => {
            parse_or_wrong!(SLiteral::parse_long, SLiteral::long)
        }
        d if *d == xsd::INTEGER => {
            parse_or_wrong!(SLiteral::parse_integer, SLiteral::integer)
        }
        d if *d == xsd::BYTE => {
            parse_or_wrong!(SLiteral::parse_byte, SLiteral::byte)
        }
        d if *d == xsd::DATE_TIME => {
            match XsdDateTime::new(value) {
                Ok(dt) => Ok(SLiteral::DatetimeLiteral(dt)),
                Err(e) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: value.to_string(),
                    datatype: datatype_iri,
                    error: e.to_string(),
                }),
            }
        }
        _ => Ok(SLiteral::lit_datatype(value, &datatype_iri)),
    }
}

/// Validates a literal's lexical form against its declared datatype.
///
/// Returns a properly typed literal if validation succeeds, or a
/// `WrongDatatypeLiteral` if the lexical form doesn't match the datatype.
fn check_literal_datatype(lexical_form: &str, datatype: &IriRef) -> Result<SLiteral, RDFError> {
    trace!("check_literal_datatype: {lexical_form}^^{datatype}");
    
    let iri = datatype.get_iri().map_err(|_| RDFError::IriRefError {
        iri_ref: datatype.to_string(),
    })?;
    
    // Helper macro to reduce repetition
    macro_rules! validate_datatype {
        ($parser:expr, $constructor:expr) => {
            match $parser(lexical_form) {
                Ok(value) => Ok($constructor(value)),
                Err(err) => Ok(SLiteral::WrongDatatypeLiteral {
                    lexical_form: lexical_form.to_string(),
                    datatype: datatype.clone(),
                    error: err,
                }),
            }
        };
    }
    
    match iri.as_str() {
        "http://www.w3.org/2001/XMLSchema#integer" => {
            validate_datatype!(SLiteral::parse_integer, SLiteral::integer)
        }
        "http://www.w3.org/2001/XMLSchema#long" => {
            validate_datatype!(SLiteral::parse_long, SLiteral::long)
        }
        "http://www.w3.org/2001/XMLSchema#double" => {
            validate_datatype!(SLiteral::parse_double, SLiteral::double)
        }
        "http://www.w3.org/2001/XMLSchema#boolean" => {
            validate_datatype!(SLiteral::parse_bool, SLiteral::boolean)
        }
        "http://www.w3.org/2001/XMLSchema#float" => {
            validate_datatype!(SLiteral::parse_float, SLiteral::float)
        }
        "http://www.w3.org/2001/XMLSchema#decimal" => {
            validate_datatype!(SLiteral::parse_decimal, SLiteral::decimal)
        }
        "http://www.w3.org/2001/XMLSchema#negativeInteger" => {
            validate_datatype!(SLiteral::parse_negative_integer, SLiteral::negative_integer)
        }
        "http://www.w3.org/2001/XMLSchema#positiveInteger" => {
            validate_datatype!(SLiteral::parse_positive_integer, SLiteral::positive_integer)
        }
        "http://www.w3.org/2001/XMLSchema#nonNegativeInteger" => {
            validate_datatype!(SLiteral::parse_non_negative_integer, SLiteral::non_negative_integer)
        }
        "http://www.w3.org/2001/XMLSchema#nonPositiveInteger" => {
            validate_datatype!(SLiteral::parse_non_positive_integer, SLiteral::non_positive_integer)
        }
        "http://www.w3.org/2001/XMLSchema#unsignedInt" => {
            validate_datatype!(SLiteral::parse_unsigned_int, SLiteral::unsigned_int)
        }
        "http://www.w3.org/2001/XMLSchema#unsignedLong" => {
            validate_datatype!(SLiteral::parse_unsigned_long, SLiteral::unsigned_long)
        }
        "http://www.w3.org/2001/XMLSchema#unsignedByte" => {
            validate_datatype!(SLiteral::parse_unsigned_byte, SLiteral::unsigned_byte)
        }
        "http://www.w3.org/2001/XMLSchema#unsignedShort" => {
            validate_datatype!(SLiteral::parse_unsigned_short, SLiteral::unsigned_short)
        }
        _ => {
            // For other datatypes, don't validate the lexical form
            // This includes rdf:langString and custom datatypes
            trace!("Not checking datatype {iri}");
            Ok(SLiteral::DatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
            })
        }
    }
}