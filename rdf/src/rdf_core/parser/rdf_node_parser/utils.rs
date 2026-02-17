use crate::rdf_core::{
    FocusRDF, RDFError,
    parser::rdf_node_parser::{RDFNodeParse, constructors::SingleValuePropertyParser},
    term::{
        Iri, IriOrBlankNode,
        literal::{ConcreteLiteral, Literal, NumericLiteral},
    },
    vocab::{rdf_first, rdf_nil, rdf_rest},
};
use iri_s::{IriS, iri};

/// Converts an RDF term to an IRI string.
///
/// This function validates that the term is an IRI and converts it to the
/// [`IriS`] type used throughout the library for IRI representation.
///
/// # Type Parameters
///
/// * `RDF` - The RDF graph type implementing `FocusRDF`
///
/// # Arguments
///
/// * `value` - The RDF term to convert
///
/// # Returns
///
/// The IRI as an `IriS` on success.
///
/// # Errors
///
/// * `RDFError::ExpectedIRIError` - If the term is not an IRI (e.g., it's a blank node or literal)
pub fn value_to_iri<RDF>(value: RDF::Term) -> Result<IriS, RDFError>
where
    RDF: FocusRDF,
{
    let iri: IriS = RDF::term_as_iri(&value)
        .map_err(|_| RDFError::ExpectedIRIError {
            term: format!("{value}"),
        })?
        .into();
    Ok(iri)
}

/// Converts an RDF term to a literal.
///
/// This function validates that the term is a literal (not an IRI or blank node)
/// and returns it in the RDF implementation's native literal type.
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The literal on success.
///
/// # Errors
///
/// * `RDFError::ExpectedLiteralError` - If the term is not a literal
pub fn term_to_literal<R>(term: &R::Term) -> Result<R::Literal, RDFError>
where
    R: FocusRDF,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedLiteralError {
            term: format!("{term}"),
        })?;
    Ok(literal)
}

/// Converts an RDF term to an integer.
///
/// This function validates that the term is an integer literal and extracts
/// its numeric value as an `isize`.
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The integer value on success.
///
/// # Errors
///
/// * `RDFError::ExpectedLiteralError` - If the term is not a literal
/// * `RDFError::ExpectedIntegerError` - If the literal is not an integer type
pub fn term_to_int<R>(term: &R::Term) -> Result<isize, RDFError>
where
    R: FocusRDF,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedLiteralError {
            term: format!("{term}"),
        })?;
    let n = literal.to_integer().ok_or_else(|| RDFError::ExpectedIntegerError {
        term: format!("{term}"),
    })?;
    Ok(n)
}

/// Converts an RDF term to a numeric literal.
///
/// This function validates that the term is a numeric literal (integer, decimal,
/// float, or double) and returns it as a [`NumericLiteral`] enum.
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The numeric literal on success.
///
/// # Errors
///
/// * `RDFError::ExpectedLiteralError` - If the term is not a literal
/// * `RDFError::ExpectedConcreteLiteralError` - If the literal cannot be converted to a concrete type
/// * `RDFError::ExpectedNumberError` - If the concrete literal is not numeric
pub fn term_to_number<R>(term: &R::Term) -> Result<NumericLiteral, RDFError>
where
    R: FocusRDF,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedLiteralError {
            term: format!("{term}"),
        })?;
    let slit: ConcreteLiteral = literal
        .try_into()
        .map_err(|_e| RDFError::ExpectedConcreteLiteralError {
            term: format!("{term}"),
        })?;
    match slit {
        ConcreteLiteral::NumericLiteral(n) => Ok(n),
        _ => Err(RDFError::ExpectedNumberError {
            term: format!("{term}"),
        }),
    }
}

/// Converts an RDF term to a boolean value.
///
/// This function validates that the term is a boolean literal (typically `xsd:boolean`)
/// and extracts its boolean value.
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The boolean value on success.
///
/// # Errors
///
/// * `RDFError::ExpectedLiteralError` - If the term is not a literal
/// * `RDFError::ExpectedBooleanError` - If the literal is not a boolean type
pub fn term_to_bool<R>(term: &R::Term) -> Result<bool, RDFError>
where
    R: FocusRDF,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedLiteralError {
            term: format!("{term}"),
        })?;
    let n = literal.to_bool().ok_or_else(|| RDFError::ExpectedBooleanError {
        term: format!("{term}"),
    })?;
    Ok(n)
}

/// Converts an RDF term to an IRI string.
///
/// This function validates that the term is an IRI and converts it to the
/// [`IriS`] type for standard IRI representation.
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The IRI as an `IriS` on success.
///
/// # Errors
///
/// * `RDFError::ExpectedIRIError` - If the term is not an IRI
pub fn term_to_iri<R>(term: &R::Term) -> Result<IriS, RDFError>
where
    R: FocusRDF,
{
    let iri: R::IRI = <R::Term as TryInto<R::IRI>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedIRIError {
        term: format!("{term}"),
    })?;
    let iri_string = iri.as_str();
    Ok(iri!(iri_string))
}

/// Converts an RDF term to an IRI or blank node.
///
/// This function validates that the term is either an IRI or a blank node
/// (i.e., a valid RDF subject) and returns it as an [`IriOrBlankNode`].
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The IRI or blank node on success.
///
/// # Errors
///
/// * `RDFError::ExpectedIriOrBlankNodeError` - If the term is not an IRI or blank node
/// * `RDFError::SubjectToIriOrBlankNodeError` - If the subject cannot be converted to `IriOrBlankNode`
pub fn term_to_iri_or_blanknode<R>(term: &R::Term) -> Result<IriOrBlankNode, RDFError>
where
    R: FocusRDF,
{
    let subj: R::Subject = <R::Term as TryInto<R::Subject>>::try_into(term.clone()).map_err(|_| {
        RDFError::ExpectedIriOrBlankNodeError {
            term: format!("{term}"),
            error: "Expected IRI or BlankNode".to_string(),
        }
    })?;
    let iri_or_bnode: IriOrBlankNode = subj
        .clone()
        .try_into()
        .map_err(|_| RDFError::SubjectToIriOrBlankNodeError {
            subject: format!("{subj}"),
        })?;
    Ok(iri_or_bnode)
}

/// Converts an RDF term to its string representation.
///
/// This function validates that the term is a literal and returns its lexical
/// form (the string representation of the literal value).
///
/// # Type Parameters
///
/// * `R` - The RDF graph type implementing `Rdf`
///
/// # Arguments
///
/// * `term` - The RDF term to convert
///
/// # Returns
///
/// The lexical form of the literal as a `String`.
///
/// # Errors
///
/// * `RDFError::ExpectedLiteralError` - If the term is not a literal
pub fn term_to_string<R>(term: &R::Term) -> Result<String, RDFError>
where
    R: FocusRDF,
{
    let literal: R::Literal =
        <R::Term as TryInto<R::Literal>>::try_into(term.clone()).map_err(|_| RDFError::ExpectedLiteralError {
            term: format!("{term}"),
        })?;
    Ok(literal.lexical_form().to_string())
}

pub fn parse_list_recursive<RDF>(mut visited: Vec<RDF::Term>, rdf: &mut RDF) -> Result<Vec<RDF::Term>, RDFError>
where
    RDF: FocusRDF,
{
    let focus = rdf.get_focus().ok_or(RDFError::NoFocusNodeError)?.clone();

    // Base case: rdf:nil
    if is_nil_node::<RDF>(&focus) {
        return Ok(Vec::new());
    }

    // Recursive case: extract first and rest
    let first = SingleValuePropertyParser::new(rdf_first().clone())
        .parse_focused(rdf)
        .map_err(|e| RDFError::PropertyNotFoundError {
            property: rdf_first().to_string(),
            subject: focus.to_string(),
            err: Box::new(e),
        })?;

    let rest = SingleValuePropertyParser::new(rdf_rest().clone())
        .parse_focused(rdf)
        .map_err(|e| RDFError::PropertyNotFoundError {
            property: rdf_rest().to_string(),
            subject: focus.to_string(),
            err: Box::new(e),
        })?;

    // Cycle detection
    if visited.contains(&rest) {
        return Err(RDFError::RecursiveRDFListError { node: rest.to_string() });
    }

    // Continue recursion
    visited.push(rest.clone());
    rdf.set_focus(&rest);

    let mut result = vec![first];
    result.extend(parse_list_recursive::<RDF>(visited, rdf)?);
    Ok(result)
}

fn is_nil_node<RDF>(node: &RDF::Term) -> bool
where
    RDF: FocusRDF,
{
    let tmp: Result<RDF::IRI, _> = <RDF::Term as TryInto<RDF::IRI>>::try_into(node.clone());
    match tmp {
        Ok(iri) => iri.as_str() == rdf_nil().as_str(),
        Err(_) => false,
    }
}
