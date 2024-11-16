//! This crate contains a _Simple_ RDF wrapper which can be useful to work with
//! RDF data.
//!
//! It contains several traits that handle RDF data:
//! - [`SRDFBasic`]: Basic comparisons on RDF nodes
//! - [`Rdf`]: Definitions on RDF graphs
//! - [`FocusRDF`]: RDF graphs with a focus node
//! - [`RDFNodeParse`]: RDF graphs that can be parsed

// pub use crate::neighs::*;
pub use crate::rdf_data_config::*;
pub use graph::*;
pub use sparql::*;
pub use srdf_parser::*;
pub use vocab::*;

// TODO: move to ShEx
// pub mod neighs;
pub mod graph;
pub mod model;
pub mod rdf_data_config;
pub mod sparql;
pub mod srdf_parser;
pub mod vocab;

/// Declares a named RDF parser which can be reused.
///
/// The expression which creates the parser should have no side effects as it
/// may be called multiple times even during a single parse attempt.
///
/// This macro is useful when declaring mutually recursive parsers
/// ```
/// #[macro_use]
/// use iri_s::IriS;
/// use srdf::rdf_parser;
/// use srdf::RDFParser;
/// use srdf::RDF;
/// use srdf::satisfy;
/// use srdf::model::parse::ReaderMode;
/// use srdf::RDFNodeParse;
/// use srdf::property_value;
/// use srdf::rdf_list;
/// use srdf::parse_property_value_as_list;
/// use srdf::set_focus;
/// use srdf::oxgraph::OxGraph;
/// use srdf::model::rdf::Rdf;
/// use srdf::model::rdf::Object;
/// use srdf::model::rdf_format::RdfFormat;
/// use srdf::model::parse::RdfParse;
/// use oxrdf::NamedNode as OxNamedNode;
/// use srdf::model::Iri;
///
/// rdf_parser!{
///     fn is_term['a, RDF](term: &'a Object<RDF>)(RDF) -> ()
///     where [ ]
///     {
///         let name = format!("is_{term}");
///         satisfy(|t| { t == *term }, name.as_str())
///     }
/// }
///
/// let s = r#"
///     prefix : <http://example.org/>
///     :x :p 1.
/// "#;
///
/// let mut graph = OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
/// let x = OxNamedNode::new_unchecked("http://example.org/x".to_string());
/// let term = x.clone().into();
/// assert_eq!(is_term(&term).parse(&x.as_iri_s(), graph).unwrap(), ())
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
             $input_type : $crate::model::focus_rdf::FocusRdf,
             $($where_clause)*
        {
            $(pub $arg : $arg_type,)*
            __marker: ::std::marker::PhantomData<$input_type>,
        }

        impl <$($type_params)*> $crate::RDFNodeParse<$input_type> for $type_name<$($type_params)*>
            where
                $input_type : $crate::model::focus_rdf::FocusRdf,
                $($where_clause)*
        {

            type Output = $output_type;

            #[inline]
            fn parse_impl(
                &mut self,
                input: &mut $input_type,
                ) -> $crate::srdf_parser::ParserResult<$output_type>
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
                $input_type: $crate::model::focus_rdf::FocusRdf,
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
