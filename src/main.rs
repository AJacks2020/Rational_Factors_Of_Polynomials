// External crates
#[macro_use] extern crate reikna;

// Declares the modules
pub mod polynomial_object;
pub mod factoring_and_primes;

// Declares the crates to use
use crate::polynomial_object::PolyEquat;
use crate::factoring_and_primes::our_factor;


// This file is set up ready for you to use as you wish. It can be ignored if you wish you just use the objects created in the other files.

fn main() {

    let test_value:Vec<i32> =  vec![8, 10];

    let test_poly: PolyEquat = PolyEquat::new(test_value);
    let test_value: f64 = 5.0;

}
