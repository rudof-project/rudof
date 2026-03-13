use pgschema::pg::PropertyGraph;
use sparql_service::RdfData;

#[derive(Debug, Clone)]
pub enum Data {
    RDFData(RdfData),
    PGData(PropertyGraph),
}