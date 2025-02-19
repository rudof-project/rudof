use oxigraph::store::Store;
use srdf::model::matcher::Matcher;
use srdf::model::rdf::Rdf;
use srdf::oxgraph::OxGraph;

pub struct RdfDataStore {
    graph: OxGraph,
    store: Option<Store>,
}

impl Rdf for RdfDataStore {
    type Triple = oxrdf::Triple;
    type Error = Infallible;

    fn triples_matching(
        &self,
        subject: impl Into<Matcher<Self>>,
        predicate: impl Into<Matcher<Self>>,
        object: impl Into<Matcher<Self>>,
    ) -> Result<impl Iterator<Item = Self::Triple>, Self::Error> {
        let triples = self
            .graph
            .triples_matching(subject, predicate, object)
            .map_err(|_| Infallible)?;

        Ok(triples)
    }
}
