use crate::factoring_and_primes::our_factor;


pub struct PolyEquat{
    pub polynomial_coefficients: Vec<i32>
}

impl PolyEquat{

    pub fn new(input_coeffs: Vec<i32>) -> Self {
        Self {
            polynomial_coefficients: input_coeffs
        }
    }

    pub fn horner_eval(&self, value: f64) -> f64 {

        let mut current_polynomial_value: f64 = 0.0;

        // Iteratively updates the above defined current_polynomial_value to implement Horner's method of evaluating polynomials
        for coeff in &self.polynomial_coefficients {
            current_polynomial_value = *coeff as f64 + value * current_polynomial_value;
        }

        return current_polynomial_value;
    }


    pub fn rational_solve(&self) -> Vec<Vec<i32>> {

        let allowed_tolerance: f64 = 0.000001; 


        // Finds all the factors of the 
        let mut poss_ps: Vec<i32> = our_factor(&self.polynomial_coefficients[self.polynomial_coefficients.len() - 1].abs());
        let mut poss_qs: Vec<i32> = our_factor(&self.polynomial_coefficients[0].abs());

        // Removes the repeats from the vectors of factors
        poss_ps.sort_unstable();
        poss_ps.dedup();
        poss_qs.sort_unstable();
        poss_qs.dedup();

        // Add in a quick check that the factors make sense!!!
        // Panic if they don't

        let mut poss_rational_sols: Vec<Vec<i32>> = vec![];
        for numerator in &poss_ps {
            for denominator in &poss_qs {
                poss_rational_sols.push(vec![*numerator, *denominator]);
                poss_rational_sols.push(vec![-1 * *numerator, *denominator]);
            }
        }
        
        // seems to be empty at this point

        // Tries the correct solutions to see which gives the correct solution
        let mut known_rational_sols: Vec<Vec<i32>> = vec![];
        for poss_solution in poss_rational_sols {
            let mut curr_test_sol:f64 = poss_solution[0] as f64 / poss_solution[1] as f64;

            if PolyEquat::horner_eval(&self, curr_test_sol) < allowed_tolerance {
                known_rational_sols.push(poss_solution);
            }
        }

        return known_rational_sols;
    }


}



// JUST TESTS FROM HERE ON DOWN


#[cfg(test)]
mod tests {
    use crate::PolyEquat;

    /*
        Tests of the new associated function of PolyEquat
    */
    #[test]
    fn new_test_one() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![1, 2, 3, 4, 5];
        let test_poly: PolyEquat = PolyEquat::new(test_value.clone());

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(test_poly.polynomial_coefficients[0], test_value[0]);
    }

    #[test]
    fn new_test_two() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![3, 4, 5, 6, 7, 8];
        let test_poly: PolyEquat = PolyEquat::new(test_value);

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(test_poly.polynomial_coefficients.len(), 6);
    }

    /*
        Tests of the horner_eval function and new (tangentially)
    */
    #[test]
    fn horner_test_one() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![1, 2, 3, 4, 5];
        let test_poly: PolyEquat = PolyEquat::new(test_value);
        let test_value: f64 = 5.0;

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(PolyEquat::horner_eval(&test_poly, test_value), 975.0);
    }

    #[test]
    fn horner_test_two() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![8, 6, 7, 5, 2, 0, 9];
        let test_poly: PolyEquat = PolyEquat::new(test_value);
        let test_value: f64 = 20.0;

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(PolyEquat::horner_eval(&test_poly, test_value), 532360809.0);
    }

    #[test]
    fn horner_test_three() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![1, -3, 5];
        let test_poly: PolyEquat = PolyEquat::new(test_value);
        let test_value: f64 = 0.000;

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(PolyEquat::horner_eval(&test_poly, test_value), 5.0);
    }

    #[test]
    fn horner_test_four() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![0];
        let test_poly: PolyEquat = PolyEquat::new(test_value);
        let test_value: f64 = 100.0;

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(PolyEquat::horner_eval(&test_poly, test_value), 0.0);
    }

    #[test]
    fn horner_test_five() {

        // Sets up values to test the function
        let test_value:Vec<i32> =  vec![0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let test_poly: PolyEquat = PolyEquat::new(test_value);
        let test_value: f64 = 1.0;

        // Compares output of horner_eval (associated function of PolyEquat struct) to known correct answer
        assert_eq!(PolyEquat::horner_eval(&test_poly, test_value), 8.0);
    }
}