
pub mod impls;


pub trait RandomField {
    fn generate_random(&self) -> Self;
}
