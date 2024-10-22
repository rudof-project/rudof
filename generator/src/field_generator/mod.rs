pub mod implementations;

pub trait RandomField {
    fn generate_random(&self) -> Self;
}
