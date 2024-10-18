use rand::Rng;
use super::traits::RandomField;  // Import the RandomField trait

// Implement RandomField for i32
impl RandomField for i32 {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100)
    }
}

// Implement RandomField for f64
impl RandomField for f64 {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..100.0)
    }
}

// Implement RandomField for bool
impl RandomField for bool {
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen_bool(0.5)
    }
}

