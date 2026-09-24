use pyo3::prelude::*;
use rudof_generate::SchemaFormat;
use rudof_generate::config::{CardinalityStrategy, DataQuality, EntityDistribution, OutputFormat};

pyenum! {
    /// Schema language a generator input is written in.
    "SchemaFormat": PySchemaFormat => SchemaFormat, default = ShEx {
        /// ShEx (Shape Expressions).
        ShEx  => ShEx,
        /// SHACL (Shapes Constraint Language).
        Shacl => Shacl,
    }
}

pyenum! {
    /// RDF serialization used for generated data.
    "OutputFormat": PyOutputFormat => OutputFormat, default = Turtle {
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
    }
}

pyenum! {
    /// How many relationships to generate when a constraint allows a range.
    "CardinalityStrategy": PyCardinalityStrategy => CardinalityStrategy, default = Balanced {
        /// Always use the minimum cardinality.
        Minimum  => Minimum,
        /// Always use the maximum cardinality, within reasonable bounds.
        Maximum  => Maximum,
        /// Choose a random value within the allowed range.
        Random   => Random,
        /// Favour realistic distributions across the allowed range.
        Balanced => Balanced,
    }
}

pyenum! {
    /// How realistic and complex the generated data should be.
    "DataQuality": PyDataQuality => DataQuality, default = Medium {
        /// Simple random data.
        Low    => Low,
        /// Realistic patterns.
        Medium => Medium,
        /// Complex realistic data with correlations.
        High   => High,
    }
}

/// How entities are distributed across the shapes of a schema.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass_enum)]
#[pyclass(
    eq,
    eq_int,
    hash,
    frozen,
    from_py_object,
    name = "EntityDistribution",
    module = "pyrudof._pyrudof"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyEntityDistribution {
    /// Equal distribution across all shapes.
    Equal,
}

impl From<&PyEntityDistribution> for EntityDistribution {
    fn from(distribution: &PyEntityDistribution) -> Self {
        match distribution {
            PyEntityDistribution::Equal => EntityDistribution::Equal,
        }
    }
}

impl From<PyEntityDistribution> for EntityDistribution {
    fn from(distribution: PyEntityDistribution) -> Self {
        (&distribution).into()
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyEntityDistribution {
    #[new]
    fn __init__() -> Self {
        PyEntityDistribution::Equal
    }

    fn __str__(&self) -> String {
        match self {
            PyEntityDistribution::Equal => "Equal".to_string(),
        }
    }

    fn __repr__(&self) -> String {
        format!("EntityDistribution.{}", self.__str__())
    }

    #[classmethod]
    fn all(_cls: &Bound<'_, pyo3::types::PyType>) -> Vec<PyEntityDistribution> {
        vec![PyEntityDistribution::Equal]
    }
}
