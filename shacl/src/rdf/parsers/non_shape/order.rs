use crate::ir::OrderValue;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::SingleLiteralPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral};
use rudof_rdf::rdf_core::vocabs::{ShaclVocab, XsdVocab};
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rust_decimal::Decimal;

pub(crate) fn order<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = OrderValue> {
    println!("Parsing sh:order");
    SingleLiteralPropertyParser::new(ShaclVocab::sh_order()).flat_map(|lit: RDF::Literal| match lit.try_into() {
        Ok(concrete_literal) => {
            println!("Parsed literal: {}", concrete_literal);
            parse_order_value(concrete_literal)
        },
        Err(_e) => Err(RDFError::ParseFailError {
            msg: format!("Failed to convert literal to concrete literal"),
        }),
    })
}

fn parse_order_value(concrete_literal: ConcreteLiteral) -> Result<OrderValue, RDFError> {
    println!("Parsing order value from concrete literal: {}", concrete_literal);
    match concrete_literal {
        ConcreteLiteral::NumericLiteral(NumericLiteral::Integer(i)) => {
            println!("Parsed order value as integer: {}", i);
            Ok(OrderValue::Integer(i))
        },
        ConcreteLiteral::NumericLiteral(NumericLiteral::Decimal(d)) => Ok(OrderValue::Decimal(d)),
        ConcreteLiteral::DatatypeLiteral { lexical_form, datatype } => {
            let datatype_iris = datatype.get_iri().map_err(|e| RDFError::ParseFailError {
                msg: format!("Failed to get datatype IRI: {}", e),
            })?;
            if datatype_iris == &XsdVocab::xsd_integer() {
                match lexical_form.parse::<i128>() {
                    Ok(i) => Ok(OrderValue::Integer(i)),
                    Err(e) => Err(RDFError::ParseFailError {
                        msg: format!("Failed to parse integer from lexical form '{}': {}", lexical_form, e),
                    }),
                }
            } else if datatype_iris == &XsdVocab::xsd_decimal() {
                match Decimal::from_str_exact(&lexical_form) {
                    Ok(d) => Ok(OrderValue::Decimal(d)),
                    Err(e) => Err(RDFError::ParseFailError {
                        msg: format!("Failed to parse decimal from lexical form '{}': {}", lexical_form, e),
                    }),
                }
            } else {
                Err(RDFError::ParseFailError {
                    msg: format!(
                        "Value of sh:order must be an integer or a decimal literal. Got datatype {}",
                        datatype
                    ),
                })
            }
        },
        _ => {
            println!(
                "Failed to parse order value from concrete literal: {:?}",
                concrete_literal
            );
            Err(RDFError::ParseFailError {
                msg: format!(
                    "Value of sh:order must be an integer or a decimal literal. Got {}",
                    concrete_literal
                ),
            })
        },
    }
}
