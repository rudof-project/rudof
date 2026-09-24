/// Declares a `#[pyclass]` unit enum together with both conversions, the dunder
/// protocol, and the stub-gen annotation.
///
/// # Syntax
///
/// ```ignore
/// pyenum! {
///     /// Doc comment for the Python class.
///     "RDFFormat": PyRDFFormat => DataFormat, default = Turtle, from_str {
///         Turtle   => Turtle,
///         NTriples => NTriples,
///         /// Doc comment for a single variant.
///         Pg       => Pg,
///     }
/// }
/// ```
///
/// * `"RDFFormat"` — the Python-visible name. Also used by `__repr__`.
/// * `PyRDFFormat` — the Rust type generated here.
/// * `DataFormat` — the `rudof_lib` (or `rudof_generate`) type it converts to and from.
/// * `default = Turtle` — what `RDFFormat()` returns from Python.
/// * `from_str` — optional literal token. Emits a `from_str` classmethod; only write it 
/// when the target type implements [`core::str::FromStr`].
/// * Variant pairs are written `Py => Rust` explicitly, because the names do not always
///   match.
///
/// # What it generates
///
/// * `From<&$Py> for $Rust` and `From<$Py> for $Rust`
/// * `From<$Rust> for $Py`
/// * `__init__`, `__str__`, `__repr__`, `all()`
/// * `from_str` classmethod
///
/// # Notes
///
/// * The `#[pyclass]` attribute is emitted by the macro rather than written at the call site
/// * `hash, frozen` make the enums usable as `dict` keys
/// * `from_py_object` is an explicit opt-in
/// * Enums whose target is not a `Copy` unit enum cannot use this macro and are written by hand
macro_rules! pyenum {
    (
        $(#[$meta:meta])*
        $name:literal : $Py:ident => $Rust:path, default = $Default:ident
        $(, $from_str:ident)?
        { $( $(#[$vmeta:meta])* $PyV:ident => $RustV:ident ),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[cfg_attr(feature = "stub-gen", ::pyo3_stub_gen_derive::gen_stub_pyclass_enum)]
        #[::pyo3::pyclass(eq, eq_int, hash, frozen, from_py_object, name = $name, module = "pyrudof._pyrudof")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $Py {
            $( $(#[$vmeta])* $PyV, )+
        }

        // Python -> Rust
        impl ::core::convert::From<&$Py> for $Rust {
            fn from(v: &$Py) -> Self {
                match v { $( $Py::$PyV => <$Rust>::$RustV, )+ }
            }
        }
        impl ::core::convert::From<$Py> for $Rust {
            fn from(v: $Py) -> Self { (&v).into() }
        }

        // Rust -> Python
        impl ::core::convert::From<$Rust> for $Py {
            fn from(v: $Rust) -> Self {
                match v { $( <$Rust>::$RustV => $Py::$PyV, )+ }
            }
        }

        #[cfg_attr(feature = "stub-gen", ::pyo3_stub_gen_derive::gen_stub_pymethods)]
        #[::pyo3::pymethods]
        impl $Py {
            #[new]
            fn __init__() -> Self { $Py::$Default }

            fn __str__(&self) -> String {
                match self { $( $Py::$PyV => stringify!($PyV).to_string(), )+ }
            }

            fn __repr__(&self) -> String {
                format!("{}.{}", $name, self.__str__())
            }
            
            #[classmethod]
            fn all(_cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>) -> Vec<$Py> {
                vec![ $( $Py::$PyV, )+ ]
            }

            $(
                #[doc = "Parses a variant from its name, case-insensitively."]
                #[classmethod]
                fn $from_str(
                    _cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>,
                    s: &str,
                ) -> crate::error::Result<$Py> {
                    let rust = <$Rust as ::core::str::FromStr>::from_str(s)
                        .map_err(|e| ::rudof_lib::errors::RudofError::Generic {
                            error: format!("{}: unknown {} '{}'", e, $name, s),
                        })?;
                    Ok(rust.into())
                }
            )?
        }
    };
}