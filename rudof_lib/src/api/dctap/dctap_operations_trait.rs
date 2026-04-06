use crate::{
    Result,
    api::dctap::implementations::{load_dctap, reset_dctap, serialize_dctap},
    formats::{DCTapFormat, InputSpec, ResultDCTapFormat},
};
use std::io;

/// Operations for DC-TAP (Dublin Core Tabular Application Profiles).
pub trait DctapOperations {
    /// Loads a DC-TAP profile from an input specification.
    ///
    /// # Arguments
    ///
    /// * `dctap` - Input specification defining the DC-TAP source
    /// * `dctap_format` - Optional DC-TAP format (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the DC-TAP profile cannot be parsed or loaded.
    fn load_dctap(&mut self, dctap: &InputSpec, dctap_format: Option<&DCTapFormat>) -> Result<()>;

    /// Serializes the current DC-TAP profile to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_dctap_format` - Optional output format for the DC-TAP profile (uses default if None)
    /// * `writer` - The destination to write the serialized DC-TAP profile to
    ///
    /// # Errors
    ///
    /// Returns an error if no DC-TAP profile is loaded or serialization fails.
    fn serialize_dctap<W: io::Write>(
        &self,
        result_dctap_format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current dctap.
    fn reset_dctap(&mut self);
}

impl DctapOperations for crate::Rudof {
    fn load_dctap(&mut self, dctap: &InputSpec, dctap_format: Option<&DCTapFormat>) -> Result<()> {
        load_dctap(self, dctap, dctap_format)
    }

    fn serialize_dctap<W: io::Write>(
        &self,
        result_dctap_format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_dctap(self, result_dctap_format, writer)
    }

    fn reset_dctap(&mut self) {
        reset_dctap(self)
    }
}
