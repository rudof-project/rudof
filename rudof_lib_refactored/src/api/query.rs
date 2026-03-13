use crate::{
    QueryOperations, Result,
    formats::{InputSpec, QueryType, ResultQueryFormat}
};
use std::io;

impl QueryOperations for crate::Rudof {
    fn load_query(
        &mut self,
        query: &InputSpec,
        query_type: &QueryType,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_query<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        todo!()
    }

    fn reset_query(&mut self) {
        todo!()
    }

    fn run_query(&mut self, endpoint: Option<&str>) -> Result<()> {
        todo!()
    }

    fn serialize_query_results<W: io::Write>(
        &self, 
        result_format: Option<&ResultQueryFormat>, 
        writer: &mut W
    ) -> Result<()> {
        todo!()
    }

    fn reset_query_results(&mut self) {
        todo!()
    }
}
