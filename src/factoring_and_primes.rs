use reikna::factor::quick_factorize;

pub fn our_factor(num_to_fac: &i32) -> Vec<i32> {

    // Need to be OK with negative numbers being input!!!
    let mut result_wrong_form = quick_factorize(*num_to_fac as u64);

    let mut result_correct_form: Vec<i32> = vec![];
    for factor in result_wrong_form {
        result_correct_form.push(factor as i32);
    } 

    return result_correct_form;
}



#[cfg(test)]
mod tests {
    use crate::factoring_and_primes::our_factor;

    /*
        Test that the factoring works correctly
    */
    #[test]
    fn our_factor_test_one() {
        let test_int: i32 = 5;
        assert_eq!(our_factor(&test_int), vec![5]);
    }

    #[test]
    fn our_factor_test_two() {
        let test_int: i32 = 144;
        assert_eq!(our_factor(&test_int), vec![2, 2, 2, 2, 3, 3]);
    }

    #[test]
    fn our_factor_test_three() {
        let test_int: i32 = 24;
        assert_eq!(our_factor(&test_int), vec![2, 2, 2, 3]);
    }

    #[test]
    fn our_factor_test_four() {
        let test_int: i32 = 18122;
        assert_eq!(our_factor(&test_int), vec![2, 13, 17, 41]);
    }

    #[test]
    #[should_panic]
    fn our_factor_test_five() {
        let test_int: i32 = -12;
        assert_eq!(our_factor(&test_int), vec![2, 2, 3]);
    }

    #[test]
    fn our_factor_test_six() {
        let test_int: i32 = 1;
        assert_eq!(our_factor(&test_int), vec![1]);
    }

    #[test]
    fn our_factor_test_seven() {
        let test_int: i32 = 7;
        assert_eq!(our_factor(&test_int), vec![7]);
    }

    #[test]
    fn our_factor_test_eight() {
        let test_int: i32 = 17;
        assert_eq!(our_factor(&test_int), vec![17]);
    }
}