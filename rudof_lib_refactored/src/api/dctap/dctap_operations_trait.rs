use crate::{
    Result,
    formats::{InputSpec, DCTapFormat, ResultDCTapFormat},
    api::dctap::implementations::{load_dctap, serialize_dctap, reset_dctap}
};
use std::io;

/// Operations for DC-TAP (Dublin Core Tabular Application Profiles).
pub trait DctapOperations {
    /// Loads a DC-TAP profile from an input specification.
    ///
    /// # Arguments
    ///
    /// * `dctap` - Input specification defining the DC-TAP source
    /// * `format` - Optional DC-TAP format (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the DC-TAP profile cannot be parsed or loaded.
    fn load_dctap(
        &mut self,
        dctap: &InputSpec,
        format: Option<&DCTapFormat>,
    ) -> Result<()>;

    /// Serializes the current DC-TAP profile to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format for the DC-TAP profile (uses default if None)
    /// * `writer` - The destination to write the serialized DC-TAP profile to
    ///
    /// # Errors
    ///
    /// Returns an error if no DC-TAP profile is loaded or serialization fails.
    fn serialize_dctap<W: io::Write>(
        &self,
        format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current dctap.
    fn reset_dctap(&mut self);
}

impl DctapOperations for crate::Rudof {
    fn load_dctap(
        &mut self,
        dctap: &InputSpec,
        format: Option<&DCTapFormat>,
    ) -> Result<()> {
        load_dctap(self,dctap, format)
    }

    fn serialize_dctap<W: io::Write>(
        &self,
        format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_dctap(self, format, writer)
    }

    fn reset_dctap(&mut self) {
        reset_dctap(self)
    }
}

