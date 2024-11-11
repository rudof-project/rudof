pub trait Sparql {
    type QuerySolution;
    type Error;

    fn select(&self, query: &str) -> Result<Vec<Self::QuerySolution>, Self::Error>;
}
