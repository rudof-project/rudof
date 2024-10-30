pub mod implementations;

pub trait RandomLiteral {
    fn generate_random(&self) -> Self;
}
