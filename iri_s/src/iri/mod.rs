mod iris;
mod iri;
mod test;
mod visitor;

pub use iri::Iri;
pub use iris::IriS;

/// Generates an [`IriS`] from a string literal.
/// ```
///
/// #[macro_use]
/// # use iri_s::{IriS, iri};
///
/// let iri = iri!("https://example.org/");
///
/// assert_eq!(iri.as_str(), "https://example.org/");
/// ```
///
/// At this moment the implementation leverages on [`oxrdf::NamedNode`](https://docs.rs/oxrdf/latest/oxrdf/struct.NamedNode.html)
///
/// Example
///
/// ```
/// # use iri_s::IriS;
/// # use std::str::FromStr;
///
/// let iri = IriS::from_str("https://example.org/").unwrap();
///
/// assert_eq!(iri.as_str(), "https://example.org/")
/// ```
///
#[macro_export]
macro_rules! iri {
    ($lit: tt) => {
        $crate::IriS::new_unchecked($lit)
    };
}

/// This macro creates a static variable that is initialized once andm can be accessed globally.
// TODO - This should be in a general utilities crate or even removed since is not used currently
#[macro_export]
macro_rules! static_once {
    ($name:ident, $type:ty, $init:expr) => {
        pub fn $name() -> &'static $type {
            static ONCE: std::sync::OnceLock<$type> = std::sync::OnceLock::new();
            ONCE.get_or_init(|| $init)
        }
    };
}

/// This macro creates a static [`IriS`] variable that is initialized once and can be accessed globally.
#[macro_export]
macro_rules! iri_once {
    ($name:ident, $str:expr) => {
        pub fn $name() -> &'static IriS {
            static ONCE: std::sync::OnceLock<IriS> = std::sync::OnceLock::new();
            ONCE.get_or_init(|| IriS::new_unchecked($str))
        }
    };
}
