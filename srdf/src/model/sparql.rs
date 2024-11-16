use std::fmt::Display;

pub trait Sparql {
    type QuerySolution;
    type Error: Display;

    fn select(&self, query: &str) -> Result<Vec<Self::QuerySolution>, Self::Error>;
    fn ask(&self, query: &str) -> Result<bool, Self::Error>;
}
