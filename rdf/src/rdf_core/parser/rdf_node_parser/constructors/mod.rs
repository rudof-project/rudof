mod combinators;
mod focus;
mod list;
mod neighs;
mod primitive;
mod property;
mod shacl;
mod typing;
mod validation;

pub use combinators::*;
pub use focus::*;
pub use list::*;
pub use neighs::*;
pub use primitive::*;
pub use property::*;
pub use shacl::*;
pub use typing::*;
pub use validation::*;

/// Declares a named RDF parser which can be reused.
///
/// The expression which creates the parser should have no side effects as it may be called
/// multiple times even during a single parse attempt.
///
/// This macro is useful when declaring mutually recursive parsers
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

/// Auxiliary macro that is invoked from `rdf_parser`
/// Supports different templates
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
             $input_type : $crate::rdf_core::FocusRDF,
             $($where_clause)*
        {
            $(pub $arg : $arg_type,)*
            __marker: ::std::marker::PhantomData<$input_type>,
        }

        impl<$($type_params)*> $crate::rdf_core::parser::rdf_node_parser::RDFNodeParse<$input_type>
            for $type_name<$($type_params)*>
        where
            $input_type: $crate::rdf_core::FocusRDF,
            $($where_clause)*
        {
            type Output = $output_type;

            #[inline]
            fn parse_focused(
                &self,
                rdf: &mut $input_type,
            ) -> Result<Self::Output, $crate::rdf_core::RDFError> {
                let $type_name { $( $arg, )* .. } = self;
                $parser.parse_focused(rdf)
            }
        }

        $(#[$attr])*
        #[inline]
        $fn_vis fn $name< $($type_params)* >(
                $($arg : $arg_type),*
            ) -> $type_name<$($type_params)*>
            where
                $input_type: $crate::rdf_core::FocusRDF,
                $($where_clause)*
        {
            $type_name {
                $($arg,)*
                __marker: ::std::marker::PhantomData,
            }
        }
    }
}
