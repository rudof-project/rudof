use crate::{
    DctapOperations, Result,
    formats::{InputSpec, DCTapFormat, ResultDCTapFormat}
};
use std::io;

impl DctapOperations for crate::Rudof {
    fn load_dctap(
        &mut self,
        dctap: &InputSpec,
        format: Option<&DCTapFormat>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_dctap<W: io::Write>(
        &self,
        format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_dctap(&mut self) {
        todo!()
    }
}
