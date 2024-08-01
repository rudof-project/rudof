//! This crate contains a _Simple_ RDF wrapper which can be useful to work with RDF data
//!
//! It contains several traits that handle RDF data:
//! - [`SRDFBasic`]: Basic comparisons on RDF nodes
//! - [`SRDF`]: Definitions on RDF graphs
//! - [`FocusRDF`]: RDF graphs with a focus node
//! - [`RDFNodeParse`]: RDF graphs that can be parsed
pub mod async_srdf;
pub mod bnode;
pub mod lang;
pub mod literal;
pub mod neighs;
pub mod numeric_literal;
pub mod object;
pub mod query_srdf;
pub mod rdf;
pub mod rdf_format;
pub mod shacl_path;
pub mod srdf;
pub mod srdf_basic;
pub mod srdf_builder;
pub mod srdf_graph;
pub mod srdf_parser;
pub mod srdf_sparql;
pub mod subject;
pub mod triple;
pub mod vocab;

pub use crate::async_srdf::*;
pub use crate::neighs::*;
pub use crate::query_srdf::*;
pub use crate::srdf::*;
pub use crate::srdf_basic::*;
pub use bnode::*;
pub use object::*;
pub use rdf::*;
pub use rdf_format::*;
pub use shacl_path::*;
pub use srdf_builder::*;
pub use srdf_graph::*;
pub use srdf_parser::*;
pub use srdf_sparql::*;
pub use subject::*;
pub use triple::*;
pub use vocab::*;

/// Creates an integer literal
///
#[macro_export]
macro_rules! int {
    (
        $n: tt
      ) => {
        $crate::literal::Literal::integer($n)
    };
}

/// Declares a named RDF parser which can be reused.
///
/// The expression which creates the parser should have no side effects as it may be called
/// multiple times even during a single parse attempt.
///
/// This macro is useful when declaring mutually recursive parsers
///
/// ```
///
/// #[macro_use]
/// use iri_s::IriS;
/// use srdf::{rdf_parser, RDFParser, RDF, RDFFormat, FocusRDF, satisfy, RDFNodeParse, SRDF, SRDFBasic, property_value, rdf_list, set_focus, parse_property_value_as_list};
/// use srdf::srdf_graph::SRDFGraph;
///
/// rdf_parser!{
///       fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
///       where [
///       ] {
///        let name = format!("is_{term}");
///        satisfy(|t| { t == *term }, name.as_str())
///       }
/// }
///
/// let s = r#"prefix : <http://example.org/>
///            :x :p 1.
/// "#;
/// let mut graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
/// let x = IriS::new_unchecked("http://example.org/x");
/// let term = <SRDFGraph as SRDFBasic>::iri_s2term(&x);
/// assert_eq!(is_term(&term).parse(&x, graph).unwrap(), ())
/// ````
#[macro_export]
macro_rules! rdf_parser {
 (
   $(#[$attr:meta])*
   $fn_vis: vis fn $name: ident [$($type_params: tt)*]( $($arg: ident :  $arg_type: ty),*)
     ($input_type: ty) -> $output_type: ty
   where [$($where_clause: tt)*]
     $parser: block
 ) => {
    $crate::combine_rdf_parser_impl!{
      #[allow(non_camel_case_types)]
      #[doc(hidden)]
      $fn_vis struct $name;
      $(#[$attr])*
      $fn_vis fn $name [$($type_params)*]($($arg : $arg_type),*)($input_type) -> $output_type
         where [$($where_clause)*]
      $parser
 }
 };
}

/// Auxiliary macro that is invoked from `rdf_parser` which supports different templates
#[macro_export]
macro_rules! combine_rdf_parser_impl {
    (
        $(#[$derive:meta])*
        $struct_vis: vis struct $type_name: ident;
        $(#[$attr:meta])*
        $fn_vis: vis fn $name: ident [$($type_params: tt)*]( $($arg: ident :  $arg_type: ty),*)
            ($input_type: ty) -> $output_type: ty
            where [$($where_clause: tt)*]
        $parser: block
    ) => {

        $(#[$derive])*
        $struct_vis struct $type_name<$($type_params)*>
            where
             $input_type : $crate::FocusRDF,
             $($where_clause)*
        {
            $(pub $arg : $arg_type,)*
            __marker: ::std::marker::PhantomData<$input_type>,
        }

        impl <$($type_params)*> $crate::RDFNodeParse<$input_type> for $type_name<$($type_params)*>
            where
                $input_type : $crate::FocusRDF,
                $($where_clause)*
        {

            type Output = $output_type;

            #[inline]
            fn parse_impl(
                &mut self,
                input: &mut $input_type,
                ) -> $crate::srdf_parser::PResult<$output_type>
            {
                let $type_name { $( $arg: ref mut $arg,)* .. } = *self;
                let r = $parser.parse_impl(input)?;
                Ok(r)
            }
        }

        $(#[$attr])*
        #[inline]
        $fn_vis fn $name< $($type_params)* >(
                $($arg : $arg_type),*
            ) -> $type_name<$($type_params)*>
            where
                $input_type: $crate::FocusRDF,
                $($where_clause)*
        {
            $type_name {
                $($arg,)*
                __marker: ::std::marker::PhantomData,
            }
        }
    }
}

#[macro_export]
macro_rules! combine_parsers {
    ($first : expr) => {
        $first
    };
    ($first : expr, $($rest : expr),+) => {
        combine_vec($first, combine_parsers!($($rest),+))
    }
}
