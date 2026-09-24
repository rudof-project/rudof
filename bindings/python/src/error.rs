use pyo3::{exceptions::PyException, PyErr};
use rudof_lib::errors::RudofError as CoreError;

/// Declares one exception class: the pyo3 type, its `PyStubType` impl, and its entry in
/// the stub generator's registry.
macro_rules! exception {
    ($name:ident, $base:ty, $doc:literal) => {
        ::pyo3::create_exception!(pyrudof, $name, $base, $doc);

        #[cfg(feature = "stub-gen")]
        impl ::pyo3_stub_gen::PyStubType for $name {
            fn type_output() -> ::pyo3_stub_gen::TypeInfo {
                ::pyo3_stub_gen::TypeInfo::with_module(
                    stringify!($name),
                    ::pyo3_stub_gen::ModuleRef::Default,
                )
            }
        }

        #[cfg(feature = "stub-gen")]
        ::pyo3_stub_gen::inventory::submit! {
            ::pyo3_stub_gen::type_info::PyClassInfo {
                pyclass_name: stringify!($name),
                struct_id: std::any::TypeId::of::<$name>,
                getters: &[],
                setters: &[],
                module: Some("pyrudof._pyrudof"),
                doc: $doc,
                bases: &[|| <$base as ::pyo3_stub_gen::PyStubType>::type_output()],
                has_eq: false,
                has_ord: false,
                has_hash: false,
                has_str: false,
                subclass: true,
            }
        }
    };
}

exception!(RudofError, PyException, "Base class for all errors raised by pyrudof.");

/// Declares an exception deriving from [`RudofError`].
///
/// A local abbreviation over [`exception!`], nothing more.
macro_rules! sub {
    ($name:ident, $doc:literal) => {
        exception!($name, RudofError, $doc);
    };
}

sub!(ConfigError, "Invalid or unreadable rudof configuration.");
sub!(InputError, "An input string, path or URL could not be resolved.");
sub!(DataError, "RDF data could not be parsed, loaded or serialized.");
sub!(ShapeMapError, "A ShapeMap could not be parsed or serialized.");
sub!(ShExError, "A ShEx schema could not be parsed, compiled or serialized.");
sub!(MapStateError, "A ShEx map state could not be read.");
sub!(MaterializeError, "Materialization failed.");
sub!(ShaclError, "A SHACL shapes graph could not be parsed or serialized.");
sub!(ValidationError, "Validation itself failed (not: validation reported violations).");
sub!(PgSchemaError, "A property-graph schema could not be parsed or serialized.");
sub!(PgDbError, "A property-graph database operation failed.");
sub!(NodeInspectionError, "A node could not be inspected.");
sub!(DCTapError, "A DCTAP profile could not be read or converted.");
sub!(ConversionError, "A schema conversion failed.");
sub!(ComparisonError, "A schema comparison failed.");
sub!(RdfConfigError, "An rdf-config document could not be read or written.");
sub!(ServiceError, "A service description could not be read or written.");
sub!(QueryError, "A SPARQL query failed to parse or execute.");
sub!(GenerateError, "Synthetic data generation failed.");
sub!(IriError, "An IRI was malformed or could not be resolved.");
sub!(PrefixesError, "A prefix declaration operation failed.");
sub!(UnsupportedOperationError,"The requested operation is not implemented.");

pub(crate) struct Error(pub(crate) CoreError);
pub(crate) type Result<T> = std::result::Result<T, Error>;

impl<E: Into<CoreError>> From<E> for Error {
    fn from(value: E) -> Self {
        Error(value.into())
    }
}

impl From<Error> for PyErr {
    fn from(Error(e): Error) -> Self {
       let msg = e.to_string();
       let err = match &e {
            CoreError::Config(_)         => ConfigError::new_err(msg),
            CoreError::InputSpec(_)      => InputError::new_err(msg),
            CoreError::Data(_)           => DataError::new_err(msg),
            CoreError::ShapeMap(_)       => ShapeMapError::new_err(msg),
            CoreError::ShEx(_)           => ShExError::new_err(msg),
            CoreError::Shacl(_)          => ShaclError::new_err(msg),
            CoreError::PgSchema(_)       => PgSchemaError::new_err(msg),
            CoreError::PgDb(_)           => PgDbError::new_err(msg),
            CoreError::Validation(_)     => ValidationError::new_err(msg),
            CoreError::NodeInspection(_) => NodeInspectionError::new_err(msg),
            CoreError::DCTap(_)          => DCTapError::new_err(msg),
            CoreError::Conversion(_)     => ConversionError::new_err(msg),
            CoreError::Comparison(_)     => ComparisonError::new_err(msg),
            CoreError::RdfConfig(_)      => RdfConfigError::new_err(msg),
            CoreError::Service(_)        => ServiceError::new_err(msg),
            CoreError::Query(_)
            | CoreError::UnsupportedResultQueryFormatSelect { .. } => QueryError::new_err(msg),
            CoreError::Generate(_)       => GenerateError::new_err(msg),
            CoreError::Iri(_)            => IriError::new_err(msg),
            CoreError::MapState(_)       => MapStateError::new_err(msg),
            CoreError::Materialize(_)    => MaterializeError::new_err(msg),
            CoreError::Prefixes(_)       => PrefixesError::new_err(msg),
            CoreError::NotImplemented { .. } => UnsupportedOperationError::new_err(msg),
            CoreError::Generic { .. }    => RudofError::new_err(msg),
        };
        // TODO: Python only gets the flattened `msg`. `rudof_lib` errors store their cause as a `String`, so there is 
        // no deeper chain to keep.
        err
    }
}