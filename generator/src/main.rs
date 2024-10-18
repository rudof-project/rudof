mod field_generator;

use field_generator::specific_generators::traits::RandomField;
fn main() {
  // Generate random values for different types
  let random_i32: i32 = i32::generate_random();
  let random_f64: f64 = f64::generate_random();
  let random_bool: bool = bool::generate_random();

  // Print the random values
  println!("Random i32: {}", random_i32);
  println!("Random f64: {}", random_f64);
  println!("Random bool: {}", random_bool);

}