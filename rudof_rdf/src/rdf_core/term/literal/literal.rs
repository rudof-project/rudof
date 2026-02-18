use crate::rdf_core::{
    RDFError,
    term::literal::{Lang, NumericLiteral, XsdDateTime},
    vocab::{
        rdf_lang, xsd_boolean, xsd_byte, xsd_date_time, xsd_decimal, xsd_double, xsd_float, xsd_integer, xsd_long,
        xsd_negative_integer, xsd_non_negative_integer, xsd_non_positive_integer, xsd_positive_integer, xsd_short,
        xsd_string, xsd_unsigned_byte, xsd_unsigned_int, xsd_unsigned_long, xsd_unsigned_short,
    },
};
use rust_decimal::Decimal;
use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    result,
};

use iri_s::IriS;
use prefixmap::{Deref, DerefError, IriRef, PrefixMap};
use serde::{Deserialize, Serialize, Serializer};

/// Types that implement this trait can be used as RDF Literals.
///
/// This trait provides methods for accessing literal properties and converting
/// literals to specific Rust types based on their XSD datatype.
pub trait Literal: Debug + Clone + Display + PartialEq + Eq + Hash {
    /// Returns the lexical form of the literal as a string slice.
    fn lexical_form(&self) -> &str;

    /// Returns the language tag if this is a language-tagged literal.
    fn lang(&self) -> Option<Lang>;

    /// Returns the datatype IRI of this literal.
    fn datatype(&self) -> IriRef;

    /// Converts this literal to an `ConcreteLiteral` if possible.
    fn to_concrete_literal(&self) -> Option<ConcreteLiteral>;

    /// Attempts to convert this literal to a boolean value.
    ///
    /// Returns `Some(bool)` if the literal has datatype `xsd:boolean` and
    /// a valid lexical form ("true" or "false").
    fn to_bool(&self) -> Option<bool> {
        let datatype = self.datatype();
        let iri = datatype.get_iri().ok()?;

        if iri.as_str() != "http://www.w3.org/2001/XMLSchema#boolean" {
            return None;
        }

        match self.lexical_form() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }

    /// Attempts to convert this literal to an integer value.
    ///
    /// Returns `Some(isize)` if the literal has datatype `xsd:integer` and
    /// a valid parseable lexical form.
    fn to_integer(&self) -> Option<isize> {
        let datatype = self.datatype();
        let iri = datatype.get_iri().ok()?;

        if iri.as_str() != "http://www.w3.org/2001/XMLSchema#integer" {
            return None;
        }

        self.lexical_form().parse().ok()
    }

    /// Attempts to convert this literal to a datetime value.
    ///
    /// Returns `Some(XsdDateTime)` if the literal has datatype `xsd:dateTime` and
    /// a valid parseable lexical form.
    fn to_date_time(&self) -> Option<XsdDateTime> {
        let datatype = self.datatype();
        let iri = datatype.get_iri().ok()?;

        if iri.as_str() != "http://www.w3.org/2001/XMLSchema#dateTime" {
            return None;
        }

        XsdDateTime::new(self.lexical_form()).ok()
    }

    /// Attempts to convert this literal to a double-precision float value.
    ///
    /// Returns `Some(f64)` if the literal has datatype `xsd:double` and
    /// a valid parseable lexical form.
    fn to_double(&self) -> Option<f64> {
        let datatype = self.datatype();
        let iri = datatype.get_iri().ok()?;

        if iri.as_str() != "http://www.w3.org/2001/XMLSchema#double" {
            return None;
        }

        self.lexical_form().parse().ok()
    }

    /// Attempts to convert this literal to a decimal value.
    ///
    /// Returns `Some(Decimal)` if the literal has datatype `xsd:decimal` and
    /// a valid parseable lexical form.
    fn to_decimal(&self) -> Option<Decimal> {
        let datatype = self.datatype();
        let iri = datatype.get_iri().ok()?;

        if iri.as_str() != "http://www.w3.org/2001/XMLSchema#decimal" {
            return None;
        }

        self.lexical_form().parse().ok()
    }
}

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
#[allow(clippy::enum_variant_names)]
pub enum ConcreteLiteral {
    /// A plain string literal, optionally with a language tag.
    StringLiteral { lexical_form: String, lang: Option<Lang> },

    /// A literal with an explicit datatype IRI.
    DatatypeLiteral { lexical_form: String, datatype: IriRef },

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
impl ConcreteLiteral {
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
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// use prefixmap::PrefixMap;
    ///
    /// let lit = ConcreteLiteral::integer(42);
    /// let prefixmap = PrefixMap::basic();
    /// println!("{}", lit.show_qualified(&prefixmap));
    /// ```
    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        let mut s = String::new();
        let _ = self.display_qualified(&mut s, prefixmap);
        s
    }

    /// Formats this literal using the given prefix map and writes the result
    /// into the provided formatter.
    ///
    /// The output follows RDF/Turtle-style literal syntax:
    /// - String literals are quoted, with an optional language tag
    /// - Numeric and boolean literals are written as-is
    /// - Datatype literals are written using `^^` and qualified IRIs
    /// - Datatypes are shortened using the provided prefix map when possible
    ///
    /// This method is intended for internal use by [`show_qualified`] and
    /// `Display` implementations rather than being called directly.
    ///
    /// # Arguments
    ///
    /// * `f` - The output writer to which the literal representation is written
    /// * `prefixmap` - The prefix map used to qualify datatype IRIs
    ///
    /// # Errors
    ///
    /// Returns any formatting error encountered while writing to the formatter.
    pub fn display_qualified<W: fmt::Write>(&self, f: &mut W, prefixmap: &PrefixMap) -> fmt::Result {
        match self {
            Self::StringLiteral { lexical_form, lang } => {
                write!(f, "\"{lexical_form}\"")?;
                if let Some(lang) = lang {
                    write!(f, "{lang}")?;
                }
                Ok(())
            },
            Self::DatatypeLiteral { lexical_form, datatype } => {
                self.format_datatype_literal(f, lexical_form, datatype, prefixmap)
            },
            Self::NumericLiteral(n) => write!(f, "{n}"),
            Self::BooleanLiteral(b) => write!(f, "{b}"),
            Self::DatetimeLiteral(dt) => write!(f, "{}", dt.value()),
            Self::WrongDatatypeLiteral {
                lexical_form, datatype, ..
            } => self.format_datatype_literal(f, lexical_form, datatype, prefixmap),
        }
    }

    /// Helper method to format datatype literals in a consistent way.
    ///
    /// This method writes a typed literal using RDF/Turtle syntax:
    /// "lexical_form"^^datatype
    ///
    /// If the datatype IRI can be qualified using the provided prefix map,
    /// the shortened prefixed form is used (e.g. `xsd:string`). Otherwise,
    /// the full IRI is written.
    ///
    /// # Arguments
    ///
    /// * `f` - The output writer to which the literal representation is written
    /// * `lexical_form` - The lexical form of the literal
    /// * `datatype` - The datatype IRI or prefixed name
    /// * `prefixmap` - The prefix map used to qualify IRIs
    ///
    /// # Errors
    ///
    /// Returns any formatting error encountered while writing to the formatter.
    fn format_datatype_literal<W: fmt::Write>(
        &self,
        f: &mut W,
        lexical_form: &str,
        datatype: &IriRef,
        prefixmap: &PrefixMap,
    ) -> fmt::Result {
        match datatype {
            IriRef::Iri(iri) => {
                write!(f, "\"{lexical_form}\"^^{}", prefixmap.qualify(iri))
            },
            IriRef::Prefixed { prefix, local } => {
                write!(f, "\"{lexical_form}\"^^{prefix}:{local}")
            },
        }
    }
}

/// ## Validation and Conversion
impl ConcreteLiteral {
    /// Validates that the lexical form matches the declared datatype,
    /// consuming the literal and returning a validated version.
    ///
    /// This method checks whether the lexical form of a datatype literal
    /// is compatible with its declared datatype. If the validation succeeds,
    /// a properly typed literal is returned. If the validation fails,
    /// a `WrongDatatypeLiteral` is returned instead.
    ///
    /// For non-datatype literals, the value is returned unchanged.
    ///
    /// # Errors
    ///
    /// Returns an `LiteralError` if datatype validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// use iri_s::IriS;
    /// use prefixmap::IriRef;
    ///
    /// // Create a datatype literal with an integer value
    /// let dt_iri = IriRef::iri(IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
    /// let lit = ConcreteLiteral::lit_datatype("42", &dt_iri);
    ///
    /// // Validate the literal
    /// let checked = lit.into_checked_literal().unwrap();
    /// ```
    pub fn into_checked_literal(self) -> Result<Self, RDFError> {
        if let Self::DatatypeLiteral { lexical_form, datatype } = self {
            check_literal_datatype(lexical_form.as_ref(), &datatype)
        } else {
            Ok(self)
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// use rudof_rdf::rdf_core::term::literal::Lang;
    ///
    /// let lit1 = ConcreteLiteral::str("hello");
    /// let lit2 = ConcreteLiteral::str("hello");
    /// let lit3 = ConcreteLiteral::lang_str("hello", Lang::new("en").unwrap());
    /// let lit4 = ConcreteLiteral::integer(42);
    ///
    /// // Plain string literals with the same content are equal
    /// assert!(lit1.match_literal(&lit2));
    ///
    /// // Language-tagged string literals must match both lexical form and lang
    /// assert!(!lit1.match_literal(&lit3));
    ///
    /// // Numeric and string literals are not equal even if lexical forms match
    /// let lit5 = ConcreteLiteral::lit_datatype("42", &lit4.datatype());
    /// assert!(!lit5.match_literal(&lit4));
    ///
    /// // Comparing numeric literals of the same value returns true
    /// let lit6 = ConcreteLiteral::integer(42);
    /// assert!(lit4.match_literal(&lit6));
    /// ```
    pub fn match_literal(&self, literal_expected: &Self) -> bool {
        match (self, literal_expected) {
            (
                Self::StringLiteral { lexical_form, lang },
                Self::StringLiteral {
                    lexical_form: expected_lexical_form,
                    lang: expected_lang,
                },
            ) => lexical_form == expected_lexical_form && lang == expected_lang,
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
                Self::WrongDatatypeLiteral {
                    lexical_form, datatype, ..
                },
                Self::WrongDatatypeLiteral {
                    lexical_form: expected_lexical_form,
                    datatype: expected_datatype,
                    ..
                },
            ) => lexical_form == expected_lexical_form && datatype == expected_datatype,
            _ => false,
        }
    }
}

/// ## Constructor Methods - Numeric Types
impl ConcreteLiteral {
    /// Creates a literal representing an unbounded integer (`i128`).
    ///
    /// This corresponds to XSD's `xsd:integer` type, which is unbounded.
    #[inline]
    pub fn integer(n: i128) -> Self {
        Self::NumericLiteral(NumericLiteral::integer(n))
    }

    /// Creates a literal representing a non-negative integer (`u128` ≥ 0).
    ///
    /// This corresponds to XSD's `xsd:nonNegativeInteger` type.
    #[inline]
    pub fn non_negative_integer(n: u128) -> Self {
        Self::NumericLiteral(NumericLiteral::non_negative_integer(n))
    }

    /// Creates a literal representing a non-positive integer (`i128` ≤ 0).
    ///
    /// This corresponds to XSD's `xsd:nonPositiveInteger` type.
    ///
    /// # Errors
    /// Returns `RDFError::NumericOutOfRange` if `n` is greater than 0.
    #[inline]
    pub fn non_positive_integer(n: i128) -> Result<Self, RDFError> {
        Ok(Self::NumericLiteral(NumericLiteral::non_positive_integer(n)?))
    }

    /// Creates a literal representing a strictly positive integer (`u128` > 0).
    ///
    /// This corresponds to XSD's `xsd:positiveInteger` type.
    ///
    /// # Errors
    /// Returns `RDFError::NumericOutOfRange` if `n` is 0.
    #[inline]
    pub fn positive_integer(n: u128) -> Result<Self, RDFError> {
        Ok(Self::NumericLiteral(NumericLiteral::positive_integer(n)?))
    }

    /// Creates a literal representing a strictly negative integer (`i128` < 0).
    ///
    /// This corresponds to XSD's `xsd:negativeInteger` type.
    ///
    /// # Errors
    /// Returns `RDFError::NumericOutOfRange` if `n` is greater than or equal to 0.
    #[inline]
    pub fn negative_integer(n: i128) -> Result<Self, RDFError> {
        Ok(Self::NumericLiteral(NumericLiteral::negative_integer(n)?))
    }

    /// Creates a literal representing a double-precision floating-point number (`f64`).
    ///
    /// This corresponds to XSD's `xsd:double` type (64-bit IEEE 754).
    #[inline]
    pub fn double(d: f64) -> Self {
        Self::NumericLiteral(NumericLiteral::double(d))
    }

    /// Creates a literal representing a decimal number (`Decimal` type for precise arithmetic).
    ///
    /// This corresponds to XSD's `xsd:decimal` type for exact decimal arithmetic.
    #[inline]
    pub fn decimal(d: Decimal) -> Self {
        Self::NumericLiteral(NumericLiteral::decimal(d))
    }

    /// Creates a literal representing a 64-bit signed long integer (`i64`).
    ///
    /// This corresponds to XSD's `xsd:long` type (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807).
    #[inline]
    pub fn long(n: i64) -> Self {
        Self::NumericLiteral(NumericLiteral::long(n))
    }

    /// Creates a literal representing a signed byte (`i8`), values -128 to 127.
    ///
    /// This corresponds to XSD's `xsd:byte` type.
    #[inline]
    pub fn byte(n: i8) -> Self {
        Self::NumericLiteral(NumericLiteral::byte(n))
    }

    /// Creates a literal representing a signed short (`i16`), values -32,768 to 32,767.
    ///
    /// This corresponds to XSD's `xsd:short` type.
    #[inline]
    pub fn short(n: i16) -> Self {
        Self::NumericLiteral(NumericLiteral::short(n))
    }

    /// Creates a literal representing an unsigned byte (`u8`), values 0–255.
    ///
    /// This corresponds to XSD's `xsd:unsignedByte` type.
    #[inline]
    pub fn unsigned_byte(n: u8) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_byte(n))
    }

    /// Creates a literal representing an unsigned short (`u16`), values 0–65,535.
    ///
    /// This corresponds to XSD's `xsd:unsignedShort` type.
    #[inline]
    pub fn unsigned_short(n: u16) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_short(n))
    }

    /// Creates a literal representing an unsigned integer (`u32`).
    ///
    /// This corresponds to XSD's `xsd:unsignedInt` type.
    #[inline]
    pub fn unsigned_int(n: u32) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_int(n))
    }

    /// Creates a literal representing an unsigned long integer (`u64`).
    ///
    /// This corresponds to XSD's `xsd:unsignedLong` type.
    #[inline]
    pub fn unsigned_long(n: u64) -> Self {
        Self::NumericLiteral(NumericLiteral::unsigned_long(n))
    }

    /// Creates a literal representing a single-precision floating-point number (`f32`).
    ///
    /// This corresponds to XSD's `xsd:float` type (32-bit IEEE 754).
    #[inline]
    pub fn float(n: f32) -> Self {
        Self::NumericLiteral(NumericLiteral::float(n))
    }

    /// Creates a DatetimeLiteral
    pub fn datetime(dt: XsdDateTime) -> Self {
        Self::DatetimeLiteral(dt)
    }
}

/// ## Constructor Methods - Other Types
impl ConcreteLiteral {
    /// Creates a literal with a custom datatype.
    ///
    /// # Parameters
    /// - `lexical_form`: The string representation of the literal's value.
    /// - `datatype`: The IRI that identifies the literal's datatype.
    pub fn lit_datatype(lexical_form: &str, datatype: &IriRef) -> Self {
        Self::DatatypeLiteral {
            lexical_form: lexical_form.to_owned(),
            datatype: datatype.clone(),
        }
    }

    /// Creates a boolean literal (`true` or `false`).
    #[inline]
    pub fn boolean(b: bool) -> Self {
        Self::BooleanLiteral(b)
    }

    /// Creates a plain string literal without a language tag.
    ///
    /// # Parameters
    /// - `lexical_form`: The text content of the literal.
    pub fn str(lexical_form: &str) -> Self {
        Self::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: None,
        }
    }

    /// Creates a string literal with a language tag.
    ///
    /// # Parameters
    /// - `lexical_form`: The text content of the literal.
    /// - `lang`: The language of the literal, e.g., `"en"` for English.
    pub fn lang_str(lexical_form: &str, lang: Lang) -> Self {
        Self::StringLiteral {
            lexical_form: lexical_form.to_owned(),
            lang: Some(lang),
        }
    }
}

/// ## Accessor Methods
impl ConcreteLiteral {
    /// Returns the language tag of the literal, if it is a language-tagged string.
    pub fn lang(&self) -> Option<Lang> {
        match self {
            Self::StringLiteral { lang, .. } => lang.clone(),
            _ => None,
        }
    }

    /// Returns the lexical form (string representation) of the literal.
    ///
    /// # Returns
    /// A `String` representing the literal's value:
    /// - For string or datatype literals, returns the literal text.
    /// - For numeric literals, returns the numeric value as a string.
    /// - For boolean literals, returns `"true"` or `"false"`.
    /// - For datetime literals, returns a standard string representation.
    ///
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

    /// Returns the datatype IRI of the literal.
    ///
    /// # Returns
    /// - For explicitly typed literals (`DatatypeLiteral` or `WrongDatatypeLiteral`), returns the stored datatype IRI.
    /// - For plain string literals without a language tag, returns `xsd:string`.
    /// - For language-tagged string literals, returns `rdf:langString`.
    /// - For numeric literals, returns the appropriate XML Schema datatype (e.g., `xsd:integer`, `xsd:double`).
    /// - For boolean literals, returns `xsd:boolean`.
    /// - For datetime literals, returns `xsd:dateTime`.
    pub fn datatype(&self) -> IriRef {
        match self {
            Self::DatatypeLiteral { datatype, .. } | Self::WrongDatatypeLiteral { datatype, .. } => datatype.clone(),

            Self::StringLiteral { lang: None, .. } => IriRef::iri(IriS::new_unchecked(xsd_string().as_str())),

            Self::StringLiteral { lang: Some(_), .. } => IriRef::iri(IriS::new_unchecked(rdf_lang().as_str())),

            Self::NumericLiteral(nl) => IriRef::iri(IriS::new_unchecked(nl.datatype())),

            Self::BooleanLiteral(_) => IriRef::iri(IriS::new_unchecked(xsd_boolean().as_str())),
            Self::DatetimeLiteral(_) => IriRef::iri(IriS::new_unchecked(xsd_date_time().as_str())),
        }
    }

    /// Returns the numeric literal value, if this literal is numeric.
    pub fn numeric_value(&self) -> Option<NumericLiteral> {
        match self {
            Self::NumericLiteral(nl) => Some(nl.clone()),
            _ => None,
        }
    }
}

/// ## Parsing Methods
impl ConcreteLiteral {
    /// Parses a boolean from its XSD lexical representation.
    ///
    /// Valid values are: "true", "false", "1" (true), "0" (false).
    /// Parsing is case-sensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the input string is not a valid boolean representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// assert_eq!(ConcreteLiteral::parse_bool("true").unwrap(), true);
    /// assert_eq!(ConcreteLiteral::parse_bool("0").unwrap(), false);
    /// assert!(ConcreteLiteral::parse_bool("yes").is_err());
    /// ```
    pub fn parse_bool(s: &str) -> Result<bool, String> {
        match s {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(format!("Cannot convert {s} to boolean")),
        }
    }

    /// Parses an integer from its string representation.
    ///
    /// XSD integer is unbounded, so this returns `i128`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// assert_eq!(ConcreteLiteral::parse_integer("-7").unwrap(), -7);
    /// assert_eq!(ConcreteLiteral::parse_integer("2").unwrap(), 2);
    /// assert!(ConcreteLiteral::parse_integer("x").is_err());
    /// ```
    pub fn parse_integer(s: &str) -> Result<i128, String> {
        s.parse::<i128>()
            .map_err(|e| format!("Cannot convert {s} to integer: {e}"))
    }

    /// Parses a negative integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not negative or cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    /// assert_eq!(ConcreteLiteral::parse_negative_integer("-3").unwrap(), -3);
    /// assert!(ConcreteLiteral::parse_negative_integer("0").is_err());
    /// ```
    pub fn parse_negative_integer(s: &str) -> Result<i128, String> {
        let value = s
            .parse::<i128>()
            .map_err(|e| format!("Cannot convert {s} to negative integer: {e}"))?;

        if value < 0 {
            Ok(value)
        } else {
            Err(format!("Cannot convert {s} to negative integer: value is not negative"))
        }
    }

    /// Parses a non-positive integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is positive or cannot be parsed.
    pub fn parse_non_positive_integer(s: &str) -> Result<i128, String> {
        let value = s
            .parse::<i128>()
            .map_err(|e| format!("Cannot convert {s} to non-positive integer: {e}"))?;

        if value <= 0 {
            Ok(value)
        } else {
            Err(format!("Cannot convert {s} to non-positive integer: value is positive"))
        }
    }

    /// Parses a positive integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not positive or cannot be parsed.
    pub fn parse_positive_integer(s: &str) -> Result<u128, String> {
        let value = s
            .parse::<u128>()
            .map_err(|e| format!("Cannot convert {s} to positive integer: {e}"))?;

        if value > 0 {
            Ok(value)
        } else {
            Err(format!("Cannot convert {s} to positive integer: value is not positive"))
        }
    }

    /// Parses a non-negative integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a non-negative integer.
    pub fn parse_non_negative_integer(s: &str) -> Result<u128, String> {
        s.parse::<u128>()
            .map_err(|e| format!("Cannot convert {s} to non-negative integer: {e}"))
    }

    /// Parses an unsigned byte (0–255) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `u8`.
    pub fn parse_unsigned_byte(s: &str) -> Result<u8, String> {
        s.parse::<u8>()
            .map_err(|e| format!("Cannot convert {s} to unsignedByte: {e}"))
    }

    /// Parses an unsigned short (0–65535) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `u16`.
    pub fn parse_unsigned_short(s: &str) -> Result<u16, String> {
        s.parse::<u16>()
            .map_err(|e| format!("Cannot convert {s} to unsignedShort: {e}"))
    }

    /// Parses an unsigned integer from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `u32`.
    pub fn parse_unsigned_int(s: &str) -> Result<u32, String> {
        s.parse::<u32>()
            .map_err(|e| format!("Cannot convert {s} to unsignedInt: {e}"))
    }

    /// Parses an unsigned long (0–u64::MAX) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `u64`.
    pub fn parse_unsigned_long(s: &str) -> Result<u64, String> {
        s.parse::<u64>()
            .map_err(|e| format!("Cannot convert {s} to unsignedLong: {e}"))
    }

    /// Parses a double (64-bit float) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `f64`.
    pub fn parse_double(s: &str) -> Result<f64, String> {
        s.parse::<f64>()
            .map_err(|e| format!("Cannot convert {s} to double: {e}"))
    }

    /// Parses a long integer (64-bit signed) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as an `i64`.
    pub fn parse_long(s: &str) -> Result<i64, String> {
        s.parse::<i64>().map_err(|e| format!("Cannot convert {s} to long: {e}"))
    }

    /// Parses a decimal from its string representation using `rust_decimal::Decimal`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a `Decimal`.
    pub fn parse_decimal(s: &str) -> Result<Decimal, String> {
        s.parse::<Decimal>()
            .map_err(|e| format!("Cannot convert {s} to decimal: {e}"))
    }

    /// Parses a float (32-bit) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as a float.
    pub fn parse_float(s: &str) -> Result<f32, String> {
        s.parse::<f32>()
            .map_err(|e| format!("Cannot convert {s} to float: {e}"))
    }

    /// Parses a signed byte (-128 to 127) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as an `i8`.
    pub fn parse_byte(s: &str) -> Result<i8, String> {
        s.parse::<i8>().map_err(|e| format!("Cannot convert {s} to byte: {e}"))
    }

    /// Parses a signed short (-32768 to 32767) from its string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the string cannot be parsed as an `i16`.
    pub fn parse_short(s: &str) -> Result<i16, String> {
        s.parse::<i16>()
            .map_err(|e| format!("Cannot convert {s} to short: {e}"))
    }

    /// Parses a datetime string using XsdDateTime
    pub fn parse_datetime(s: &str) -> Result<XsdDateTime, String> {
        XsdDateTime::new(s).map_err(|e| e.to_string())
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl Default for ConcreteLiteral {
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
impl PartialOrd for ConcreteLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Chronological comparison for datetime literals
            (Self::DatetimeLiteral(dt1), Self::DatetimeLiteral(dt2)) => dt1.partial_cmp(dt2),
            // Lexicographic comparison for plain string literals
            (Self::StringLiteral { lexical_form: lf1, .. }, Self::StringLiteral { lexical_form: lf2, .. }) => {
                Some(lf1.cmp(lf2))
            },
            // Datatype literals are only comparable if their datatypes match
            (
                Self::DatatypeLiteral {
                    lexical_form: lf1,
                    datatype: dt1,
                },
                Self::DatatypeLiteral {
                    lexical_form: lf2,
                    datatype: dt2,
                },
            ) if dt1 == dt2 => Some(lf1.cmp(lf2)),
            // Numeric comparison (may return None for NaN)
            (Self::NumericLiteral(n1), Self::NumericLiteral(n2)) => n1.partial_cmp(n2),
            // Boolean ordering: false < true
            (Self::BooleanLiteral(b1), Self::BooleanLiteral(b2)) => Some(b1.cmp(b2)),
            // Wrong-datatype literals can still be compared lexically if the expected datatype matches
            (
                Self::WrongDatatypeLiteral {
                    lexical_form: lf1,
                    datatype: dt1,
                    ..
                },
                Self::DatatypeLiteral {
                    lexical_form: lf2,
                    datatype: dt2,
                },
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
impl Ord for ConcreteLiteral {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("Cannot compare literals {self} and {other}"))
    }
}

impl Display for ConcreteLiteral {
    /// Formats the literal using a basic prefix map for qualified display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_qualified(f, &PrefixMap::basic())
    }
}

impl Deref for ConcreteLiteral {
    /// Resolves IRIs and prefixes contained in the literal.
    ///
    /// - Value-based literals (`NumericLiteral`, `BooleanLiteral`, `DatetimeLiteral`, `StringLiteral`) are cloned directly.
    /// - Datatype literals have their datatype IRIs dereferenced using `base` and `prefixmap`.
    /// - Wrong datatype literals are converted into properly typed literals.
    ///
    /// # Errors
    ///
    /// Returns `DerefError` if datatype resolution fails.
    fn deref(self, base: Option<&IriS>, prefixmap: Option<&PrefixMap>) -> Result<Self, DerefError> {
        match self {
            Self::NumericLiteral(_) | Self::BooleanLiteral(_) | Self::DatetimeLiteral(_) => Ok(self.clone()),
            Self::StringLiteral { .. } => Ok(self.clone()),

            Self::DatatypeLiteral { lexical_form, datatype }
            | Self::WrongDatatypeLiteral {
                lexical_form, datatype, ..
            } => {
                let dt = datatype.deref(base, prefixmap)?;
                Ok(Self::DatatypeLiteral {
                    lexical_form: lexical_form.clone(),
                    datatype: dt,
                })
            },
        }
    }
}

// ============================================================================
// Conversions
// ============================================================================

impl TryFrom<oxrdf::Literal> for ConcreteLiteral {
    type Error = RDFError;

    /// Attempts to convert an oxrdf literal into an `SLiteral`.
    ///
    /// Supported cases:
    /// - Plain string literals
    /// - Language-tagged string literals
    /// - Typed literals (with datatype parsing)
    ///
    /// # Errors
    ///
    /// Returns an `LiteralError` if:
    /// - The language tag is invalid
    /// - The datatype is unsupported or malformed
    /// - The literal structure is unknown
    fn try_from(value: oxrdf::Literal) -> Result<Self, Self::Error> {
        let literal_str = value.to_string();

        let (lexical, datatype_opt, lang_opt, _) = value.destruct();

        match (lexical, datatype_opt, lang_opt) {
            (s, None, None) => Ok(Self::str(&s)),

            (s, None, Some(lang_tag)) => Lang::new(lang_tag.clone())
                .map(|lang| Self::lang_str(&s, lang))
                .map_err(|e| RDFError::LanguageTagError {
                    literal: literal_str,
                    language: lang_tag,
                    error: e.to_string(),
                }),

            (s, Some(dtype), None) => {
                // Use safe IRI creation if possible
                let datatype_iri = IriRef::iri(IriS::new_unchecked(dtype.as_str()));
                check_literal_datatype(s.as_ref(), &datatype_iri)
            },

            _ => Err(RDFError::ConversionError {
                msg: format!("Unknown literal structure: {literal_str}"),
            }),
        }
    }
}

impl From<ConcreteLiteral> for oxrdf::Literal {
    /// Converts an `SLiteral` into an `oxrdf::Literal`
    fn from(value: ConcreteLiteral) -> Self {
        // Helper for datatype literals to reduce repetition
        fn typed_literal(lexical: String, datatype: &IriRef) -> oxrdf::Literal {
            datatype
                .get_iri()
                .map(|dt: &IriS| oxrdf::Literal::new_typed_literal(lexical.clone(), oxrdf::NamedNode::from(dt.clone())))
                .unwrap_or_else(|_| lexical.into())
        }

        match value {
            ConcreteLiteral::StringLiteral { lexical_form, lang } => match lang {
                Some(l) => oxrdf::Literal::new_language_tagged_literal_unchecked(lexical_form, l.to_string()),
                None => lexical_form.into(),
            },

            ConcreteLiteral::DatatypeLiteral { lexical_form, datatype }
            | ConcreteLiteral::WrongDatatypeLiteral {
                lexical_form, datatype, ..
            } => typed_literal(lexical_form, &datatype),

            ConcreteLiteral::NumericLiteral(number) => number.into(),
            ConcreteLiteral::BooleanLiteral(b) => b.into(),
            ConcreteLiteral::DatetimeLiteral(dt) => (*dt.value()).into(),
        }
    }
}

impl From<&ConcreteLiteral> for oxrdf::Literal {
    // Converts a reference to an `SLiteral` into an `oxrdf::Literal`
    fn from(value: &ConcreteLiteral) -> Self {
        oxrdf::Literal::from(value.clone())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Serializes a boolean literal as a string ("true" or "false").
///
/// # Parameters
/// - `value`: A reference to the boolean value to serialize.
/// - `serializer`: The serializer to use (implements the `Serializer` trait).
fn serialize_boolean_literal<S>(value: &bool, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *value { "true" } else { "false" })
}

/// Validates a literal's lexical form against its declared datatype.
///
/// Returns a properly typed literal if validation succeeds, or a
/// `WrongDatatypeLiteral` if the lexical form doesn't match the datatype.
///
/// For unknown or custom datatypes, returns a `DatatypeLiteral` without validating.
///
/// # Arguments
///
/// * `lexical_form` - The string value of the literal
/// * `datatype` - The declared datatype as an owned `IriRef`
///
/// # Errors
///
/// Returns `RDFError` if the datatype IRI itself is invalid.
fn check_literal_datatype(lexical_form: &str, datatype: &IriRef) -> Result<ConcreteLiteral, RDFError> {
    // Resolve the IRI
    let iri = datatype.get_iri().map_err(|_| RDFError::IriRefError {
        iri_ref: datatype.to_string(),
    })?;

    let iri_str = iri.as_str();

    // Macro for constructors that return ConcreteLiteral directly
    macro_rules! check_xsd_type {
        ($xsd_const:expr, $parse_fn:expr, $construct_fn:expr) => {
            if iri_str == $xsd_const {
                return Ok(validate(lexical_form, datatype, $parse_fn, $construct_fn));
            }
        };
    }

    // Macro for constructors that return Result<ConcreteLiteral, RDFError>
    macro_rules! check_xsd_type_result {
        ($xsd_const:expr, $parse_fn:expr, $construct_fn:expr) => {
            if iri_str == $xsd_const {
                return Ok(validate_with_result(
                    lexical_form,
                    datatype,
                    $parse_fn,
                    $construct_fn,
                ));
            }
        };
    }

    // Check all XSD types using appropriate macro
    check_xsd_type!(
        xsd_integer().as_str(),
        ConcreteLiteral::parse_integer,
        ConcreteLiteral::integer
    );
    check_xsd_type!(xsd_long().as_str(), ConcreteLiteral::parse_long, ConcreteLiteral::long);
    check_xsd_type!(
        xsd_double().as_str(),
        ConcreteLiteral::parse_double,
        ConcreteLiteral::double
    );
    check_xsd_type!(
        xsd_boolean().as_str(),
        ConcreteLiteral::parse_bool,
        ConcreteLiteral::boolean
    );
    check_xsd_type!(
        xsd_float().as_str(),
        ConcreteLiteral::parse_float,
        ConcreteLiteral::float
    );
    check_xsd_type!(
        xsd_decimal().as_str(),
        ConcreteLiteral::parse_decimal,
        ConcreteLiteral::decimal
    );
    check_xsd_type!(xsd_byte().as_str(), ConcreteLiteral::parse_byte, ConcreteLiteral::byte);
    check_xsd_type!(
        xsd_short().as_str(),
        ConcreteLiteral::parse_short,
        ConcreteLiteral::short
    );
    check_xsd_type!(
        xsd_unsigned_int().as_str(),
        ConcreteLiteral::parse_unsigned_int,
        ConcreteLiteral::unsigned_int
    );
    check_xsd_type!(
        xsd_unsigned_long().as_str(),
        ConcreteLiteral::parse_unsigned_long,
        ConcreteLiteral::unsigned_long
    );
    check_xsd_type!(
        xsd_unsigned_byte().as_str(),
        ConcreteLiteral::parse_unsigned_byte,
        ConcreteLiteral::unsigned_byte
    );
    check_xsd_type!(
        xsd_unsigned_short().as_str(),
        ConcreteLiteral::parse_unsigned_short,
        ConcreteLiteral::unsigned_short
    );
    check_xsd_type!(
        xsd_non_negative_integer().as_str(),
        ConcreteLiteral::parse_non_negative_integer,
        ConcreteLiteral::non_negative_integer
    );

    // These constructors return Result and need special handling
    check_xsd_type_result!(
        xsd_negative_integer().as_str(),
        ConcreteLiteral::parse_negative_integer,
        ConcreteLiteral::negative_integer
    );
    check_xsd_type_result!(
        xsd_positive_integer().as_str(),
        ConcreteLiteral::parse_positive_integer,
        ConcreteLiteral::positive_integer
    );
    check_xsd_type_result!(
        xsd_non_positive_integer().as_str(),
        ConcreteLiteral::parse_non_positive_integer,
        ConcreteLiteral::non_positive_integer
    );
    check_xsd_type!(
        xsd_date_time().as_str(),
        ConcreteLiteral::parse_datetime,
        ConcreteLiteral::datetime
    );

    // Unknown or custom datatype: do not validate lexical form
    Ok(ConcreteLiteral::DatatypeLiteral {
        lexical_form: lexical_form.to_string(),
        datatype: datatype.clone(),
    })
}

/// Validates a lexical form against a specified datatype using a parser and constructor.
///
/// # Parameters
/// - `lexical_form`: The literal value as a string that needs to be validated.
/// - `datatype`: The IRI of the expected datatype.
/// - `parser`: A function that attempts to parse the `lexical_form` into a value of type `T`.
/// - `constructor`: A function that constructs a `ConcreteLiteral` from a successfully parsed value.
fn validate<T, P, C>(lexical_form: &str, datatype: &IriRef, parser: P, constructor: C) -> ConcreteLiteral
where
    P: Fn(&str) -> Result<T, String>,
    C: Fn(T) -> ConcreteLiteral,
{
    match parser(lexical_form) {
        Ok(value) => constructor(value),
        Err(err) => ConcreteLiteral::WrongDatatypeLiteral {
            lexical_form: lexical_form.to_string(),
            datatype: datatype.clone(),
            error: err.to_string(),
        },
    }
}

/// Validates a lexical form with a constructor that returns `Result`.
///
/// This variant handles constructors that perform additional validation
/// (e.g., range checks for positiveInteger, negativeInteger).
///
/// # Parameters
/// - `lexical_form`: The literal value as a string that needs to be validated.
/// - `datatype`: The IRI of the expected datatype.
/// - `parser`: A function that attempts to parse the `lexical_form` into a value of type `T`.
/// - `constructor`: A function that constructs a `ConcreteLiteral` from a parsed value, returning `Result`.
fn validate_with_result<T, P, C>(lexical_form: &str, datatype: &IriRef, parser: P, constructor: C) -> ConcreteLiteral
where
    P: Fn(&str) -> Result<T, String>,
    C: Fn(T) -> Result<ConcreteLiteral, RDFError>,
{
    match parser(lexical_form) {
        Ok(value) => match constructor(value) {
            Ok(literal) => literal,
            Err(err) => ConcreteLiteral::WrongDatatypeLiteral {
                lexical_form: lexical_form.to_string(),
                datatype: datatype.clone(),
                error: err.to_string(),
            },
        },
        Err(err) => ConcreteLiteral::WrongDatatypeLiteral {
            lexical_form: lexical_form.to_string(),
            datatype: datatype.clone(),
            error: err.to_string(),
        },
    }
}
