use super::rdf::Rdf;

pub trait Sparql: Rdf {
    type QuerySolution;

    fn select(
        &self,
        prefixmap: Vec<Self::IRI>,
        query: &str,
    ) -> Result<Vec<Self::QuerySolution>, Self::Error>;

    fn ask(&self, prefixmap: Vec<Self::IRI>, query: &str) -> Result<bool, Self::Error>;

    fn construct(&self, prefixmap: Vec<Self::IRI>, query: &str) -> Result<(), Self::Error>;

    fn update(&self, prefixmap: Vec<Self::IRI>, query: &str) -> Result<(), Self::Error>;
}
