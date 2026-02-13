use crate::rdf_core::{
    Any, FocusRDF, RDFError,
    parser::rdf_node_parser::{
        ParserExt, RDFNodeParse, constructors::{ListParser, SetFocusParser}, utils::{term_to_bool, term_to_int, term_to_iri, term_to_iri_or_blanknode, term_to_literal, term_to_string}
    },
    term::{IriOrBlankNode, Object, Triple, literal::ConcreteLiteral},
};
use iri_s::IriS;
use std::collections::HashSet;
use std::marker::PhantomData;

// ============================================================================
// Multiple value parsers
// ============================================================================

/// Parser that extracts all values of a property from the focus node.
///
/// Returns a `HashSet` of terms. If the property has no values, returns an empty set.
#[derive(Debug, Clone)]
pub struct ValuesPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> ValuesPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        ValuesPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for ValuesPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        if let Ok(subject) = rdf.get_focus_as_subject() {
            let pred: RDF::IRI = self.property.clone().into();

             Ok(rdf
                .triples_matching(&subject, &pred, &Any)
                .map_err(|e| RDFError::OutgoingArcsError {
                    focus: format!("{}", self.property),
                    error: e.to_string(),
                })?
                .map(Triple::into_object)
                .collect()
            )
        } else {
            Ok(HashSet::new())
        }
    }
}

/// Parser that extracts property values, failing if none exist
#[derive(Debug, Clone)]
pub struct NonEmptyValuesPropertyParser<RDF> {
    inner: ValuesPropertyParser<RDF>,
}

impl<RDF> NonEmptyValuesPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        NonEmptyValuesPropertyParser {
            inner: ValuesPropertyParser::new(property),
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for NonEmptyValuesPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<RDF::Term>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let vals = self.inner.parse_focused(rdf)?;
        if vals.is_empty() {
            Err(RDFError::NoValuesPredicateError {
                node: "focus node".to_string(),
                pred: format!("{}", self.inner.property),
            })
        } else {
            Ok(vals)
        }
    }
}

// ============================================================================
// Single value parsers
// ============================================================================

/// Parser that extracts exactly one value from the property.
///
/// Fails if the property has zero or more than one value.
#[derive(Debug, Clone)]
pub struct SingleValuePropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleValuePropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleValuePropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleValuePropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = RDF::Term;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus_str = rdf
            .get_focus()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "No focus node".to_string());

        let vals: HashSet<_> = ValuesPropertyParser {
            property: self.property.clone(),
            _marker: PhantomData,
        }
        .parse_focused(rdf)?;

        let mut iter = vals.into_iter();
        let first = iter
            .next()
            .ok_or_else(|| RDFError::NoValuesPredicateError {
                node: focus_str,
                pred: format!("{}", self.property),
            })?;

        Ok(first)
    }
}

/// Parser that extracts a property value and parses it as an RDF List.
#[derive(Debug, Clone)]
pub struct SingleValuePropertyAsListParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleValuePropertyAsListParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleValuePropertyAsListParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleValuePropertyAsListParser<RDF>
where
    RDF: FocusRDF + 'static,
{
    type Output = Vec<RDF::Term>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .then(|node| SetFocusParser::new(node).then(|_| ListParser::new())).parse_focused(rdf)
    }
}

// ============================================================================
// Typed multiple-values parsers
// ============================================================================

#[derive(Debug, Clone)]
pub struct IntegersPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> IntegersPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        IntegersPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for IntegersPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<isize>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| term_to_int::<RDF>(&t))
            .collect::<Result<Vec<_>, _>>()
    }
}

/// Parser for all boolean values of a property.
#[derive(Debug, Clone)]
pub struct BoolsPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> BoolsPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        BoolsPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for BoolsPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<bool>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| term_to_bool::<RDF>(&t))
            .collect::<Result<Vec<_>, _>>()
    }
}

/// Parser for all string values of a property.
#[derive(Debug, Clone)]
pub struct StringsPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> StringsPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        StringsPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for StringsPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<String>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| term_to_string::<RDF>(&t))
            .collect::<Result<Vec<_>, _>>()
    }
}

/// Parser for all IRI values of a property (as HashSet for uniqueness).
#[derive(Debug, Clone)]
pub struct IrisPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> IrisPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        IrisPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for IrisPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<IriS>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| term_to_iri::<RDF>(&t))
            .collect::<Result<HashSet<_>, _>>()
    }
}

/// Parser for all IRI or BlankNode values of a property (as HashSet for uniqueness).
#[derive(Debug, Clone)]
pub struct IrisOrBnodesPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> IrisOrBnodesPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        IrisOrBnodesPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for IrisOrBnodesPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = HashSet<IriOrBlankNode>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| term_to_iri_or_blanknode::<RDF>(&t))
            .collect::<Result<HashSet<_>, _>>()
    }
}

/// Parser for all Object values of a property
#[derive(Debug, Clone)]
pub struct ObjectsPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> ObjectsPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        ObjectsPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for ObjectsPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<Object>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| {
                let term_str = t.to_string();
                t.try_into()
                    .map_err(|_| RDFError::FailedTermToObjectError { term: term_str })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

/// Parser for all literal values of a property.
#[derive(Debug, Clone)]
pub struct LiteralsPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> LiteralsPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        LiteralsPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for LiteralsPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<ConcreteLiteral>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        ValuesPropertyParser::new(self.property.clone())
            .parse_focused(rdf)?
            .into_iter()
            .map(|t| {
                let rdf_lit: RDF::Literal = term_to_literal::<RDF>(&t)?;
                
                let slit: ConcreteLiteral = rdf_lit.clone().try_into().map_err(|_| {
                    RDFError::DefaultError {
                        msg: format!("Error converting literal {} to SLiteral", rdf_lit),
                    }
                })?;
                
                Ok(slit)
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

// ============================================================================
// Typed single-value parsers
// ============================================================================

/// Parser for a single integer value.
#[derive(Debug, Clone)]
pub struct SingleIntegerPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleIntegerPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleIntegerPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleIntegerPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = isize;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .parse_focused(rdf)
            .and_then(|term| term_to_int::<RDF>(&term))
    }
}

/// Parser for a single boolean value.
#[derive(Debug, Clone)]
pub struct SingleBoolPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleBoolPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleBoolPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleBoolPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = bool;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .parse_focused(rdf)
            .and_then(|term| term_to_bool::<RDF>(&term))
    }
}

/// Parser for a single IRI value.
#[derive(Debug, Clone)]
pub struct SingleIriPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleIriPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleIriPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleIriPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = IriS;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .parse_focused(rdf)
            .and_then(|term| term_to_iri::<RDF>(&term))
    }
}

/// Parser for a single string value.
#[derive(Debug, Clone)]
pub struct SingleStringPropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleStringPropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        SingleStringPropertyParser {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleStringPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = String;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .parse_focused(rdf)
            .and_then(|term| term_to_string::<RDF>(&term))
    }
}

/// Parser for a single iri or blank node value.
#[derive(Debug, Clone)]
pub struct SingleIriOrBlankNodePropertyParser<RDF> {
    property: IriS,
    _marker: PhantomData<RDF>,
}

impl<RDF> SingleIriOrBlankNodePropertyParser<RDF> {
    pub fn new(property: IriS) -> Self {
        Self {
            property,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for SingleIriOrBlankNodePropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = IriOrBlankNode;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        SingleValuePropertyParser::new(self.property.clone())
            .parse_focused(rdf)
            .and_then(|term| term_to_iri_or_blanknode::<RDF>(&term))
    }
}

// ============================================================================
// Other parsers
// ============================================================================

/// Parser that finds subjects with a specific property-value pair (reverse lookup).
#[derive(Debug, Clone)]
pub struct SubjectsWithValuePropertyParser<RDF>
where
    RDF: FocusRDF,
{
    property: RDF::IRI,
    value: RDF::Term,
}

impl<RDF> SubjectsWithValuePropertyParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(property: RDF::IRI, value: RDF::Term) -> Self {
        SubjectsWithValuePropertyParser { property, value }
    }
}

impl<RDF> RDFNodeParse<RDF> for SubjectsWithValuePropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDF::Subject>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        Ok(rdf
            .triples_matching(&Any, &self.property, &self.value)
            .map_err(|e| RDFError::ObtainingTriples {
                error: e.to_string(),
            })?
            .map(Triple::into_subject)
            .collect())
    }
}
