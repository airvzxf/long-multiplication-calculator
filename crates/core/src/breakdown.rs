//! Breakdown algorithm for the long multiplication table.

use crate::length::{get_string_length, get_strings_length};

/// Get a list of the sum for the rows in each column.
///
/// Given two numbers that are multiplied, it gets the
/// multiplication result (units and carriers) for each
/// multiplicand by each multiplier.
/// This method sums each row for each column and returns
/// a list with these sums split by columns.
///
/// The size of the list for the sums is the maximum possible
/// number of columns to the product for the number of digits
/// for multiplicand plus multiplier.
///
/// This starts from left to right; on the right, we have
/// the units, or the first column, then the second column,
/// which is the dozens. So on until you reach the last column.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::breakdown::break_down_addition;
///
/// assert_eq!(break_down_addition("2", "3"), vec![6, 0]);
/// assert_eq!(break_down_addition("13", "26"), vec![8, 13, 2, 0]);
/// ```
pub fn break_down_addition(multiplicand: &str, multiplier: &str) -> Vec<usize> {
    let multiplicand_len: usize = get_string_length(multiplicand);
    let length: usize = get_strings_length(multiplicand, multiplier);
    let step: usize = multiplicand_len;

    let units: Vec<usize>;
    let carriers: Vec<usize>;
    (units, carriers) = break_down_multiplication(multiplicand, multiplier);

    let mut addition: Vec<usize> = Vec::new();
    for _ in 0..length {
        addition.push(0);
    }

    let mut iteration: usize = 0;
    let total_units: usize = units.len();
    for start in (0..total_units).step_by(step) {
        for sub_index in start..start + step {
            let carry_index: usize = start + step + iteration - sub_index;
            let carry: usize = carriers[sub_index];
            addition[carry_index] += carry;
            let unit_index: usize = carry_index - 1;
            let unit: usize = units[sub_index];
            addition[unit_index] += unit;
        }
        iteration += 1;
    }

    addition
}

/// Break down the multiplication to get information of the
/// long multiplication.
///
/// Using the long multiplication method, we get the information
/// for each digit of the multiplicand by each digit of the
/// multiplier.
/// The information is the result of the products and the carriers
/// for each multiplicand by multiplier.
///
/// This information (result of the products and the carriers) is
/// returned as a collection of vectors.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::breakdown::break_down_multiplication;
///
/// let (operation_unit, operation_carry) = break_down_multiplication("25", "3");
/// assert_eq!(operation_unit, vec![6, 5]);
/// assert_eq!(operation_carry, vec![0, 1]);
///
/// let (operation_unit, operation_carry) = break_down_multiplication("13", "26");
/// assert_eq!(operation_unit, vec![6, 8, 2, 6]);
/// assert_eq!(operation_carry, vec![0, 1, 0, 0]);
/// ```
pub fn break_down_multiplication(multiplicand: &str, multiplier: &str) -> (Vec<usize>, Vec<usize>) {
    let mut operation_unit: Vec<usize> = Vec::new();
    let mut operation_carry: Vec<usize> = Vec::new();

    for a in multiplier.chars().rev() {
        let mut units: Vec<usize> = Vec::new();
        let mut carriers: Vec<usize> = Vec::new();
        for b in multiplicand.chars().rev() {
            let multiplicand_digit: usize = a as usize - 0x30;
            let multiplier_digit: usize = b as usize - 0x30;
            let product: usize = multiplicand_digit * multiplier_digit;
            let unit: usize = product % 10;
            let carry: usize = product / 10;
            units.push(unit);
            carriers.push(carry);
        }

        units.reverse();
        for unit in units {
            operation_unit.push(unit);
        }

        carriers.reverse();
        for carry in carriers {
            operation_carry.push(carry);
        }
    }

    (operation_unit, operation_carry)
}

/// Get a list of the last sum and sum again removing
/// the decimals.
///
/// Given a list of the numbers which are the result
/// for the last sum.
/// They are re-sum, but this time
/// it correctly joins the decimals and units for
/// different columns.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::breakdown::break_down_subtotal;
///
/// assert_eq!(break_down_subtotal(&vec![6, 0]), vec![6, 0]);
/// assert_eq!(
///     break_down_subtotal(&vec![1, 10, 19, 27, 27, 27, 26, 17, 8]),
///     vec![1, 0, 10, 8, 9, 9, 8, 9, 9]
/// );
/// ```
pub fn break_down_subtotal(addition: &Vec<usize>) -> Vec<usize> {
    let mut new_addition: Vec<usize> = Vec::new();
    for _ in 0..addition.len() {
        new_addition.push(0);
    }

    for index in 0..addition.len() {
        let number: usize = addition[index];
        if number < 10 {
            new_addition[index] += number;
        } else {
            let decimal: usize = number / 10;
            let unit: usize = number % 10;
            new_addition[index + 1] += decimal;
            new_addition[index] += unit;
        }
    }

    new_addition
}

#[cfg(test)]
mod tests {
    use super::*;

    // # -----------------------------------------------------------------------
    // # Function: break_down_multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_multiplication_with_three_digits_multiplicand_is_greater() {
        let multiplicand: &str = "25";
        let multiplier: &str = "3";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 5];
        let expected_carry: Vec<usize> = vec![0, 1];

        (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_three_digits_multiplier_is_greater() {
        let multiplicand: &str = "3";
        let multiplier: &str = "25";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![5, 6];
        let expected_carry: Vec<usize> = vec![1, 0];

        (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_four_digit() {
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 8, 2, 6];
        let expected_carry: Vec<usize> = vec![0, 1, 0, 0];

        (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_six_digit() {
        let multiplicand: &str = "123";
        let multiplier: &str = "456";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 2, 8, 5, 0, 5, 4, 8, 2];
        let expected_carry: Vec<usize> = vec![0, 1, 1, 0, 1, 1, 0, 0, 1];

        (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_addition
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_addition_product_one_digit() {
        let multiplicand: &str = "2";
        let multiplier: &str = "3";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 0];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_product_two_digits() {
        let multiplicand: &str = "9";
        let multiplier: &str = "8";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![2, 7];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_three_digits() {
        let multiplicand: &str = "37";
        let multiplier: &str = "8";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_three_digits_switch() {
        let multiplicand: &str = "8";
        let multiplier: &str = "37";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_four_digit() {
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 13, 2, 0];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_six_digit() {
        let multiplicand: &str = "123";
        let multiplier: &str = "456";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 8, 10, 15, 4, 0];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_eleven_digits_multiplier_is_greater() {
        let multiplicand: &str = "78924358";
        let multiplier: &str = "357";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_eleven_digits_multiplier_is_less() {
        let multiplicand: &str = "357";
        let multiplier: &str = "78924358";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        addition = break_down_addition(multiplicand, multiplier);

        assert_eq!(expected_addition, addition);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_subtotal
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_subtotal_result_two_digits_with_zero() {
        let value: Vec<usize> = vec![6, 0];
        let expected: Vec<usize> = vec![6, 0];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_two_digits_without_zero() {
        let value: Vec<usize> = vec![2, 4];
        let expected: Vec<usize> = vec![2, 4];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_three_digits_with_zero() {
        let value: Vec<usize> = vec![2, 9, 0];
        let expected: Vec<usize> = vec![2, 9, 0];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_three_digits_without_zero() {
        let value: Vec<usize> = vec![5, 8, 2];
        let expected: Vec<usize> = vec![5, 8, 2];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_with_zero() {
        let value: Vec<usize> = vec![4, 8, 4, 0];
        let expected: Vec<usize> = vec![4, 8, 4, 0];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_with_zero_and_carry() {
        let value: Vec<usize> = vec![4, 11, 6, 0];
        let expected: Vec<usize> = vec![4, 1, 7, 0];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_without_zero_and_carry() {
        let value: Vec<usize> = vec![6, 12, 6, 2];
        let expected: Vec<usize> = vec![6, 2, 7, 2];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_nine_digits_with_zero_and_carry() {
        let value: Vec<usize> = vec![1, 10, 19, 27, 27, 27, 26, 17, 8];
        let expected: Vec<usize> = vec![1, 0, 10, 8, 9, 9, 8, 9, 9];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_nine_digits_without_zero_and_carry() {
        let value: Vec<usize> = vec![5, 10, 10, 10, 5, 16, 4, 0];
        let expected: Vec<usize> = vec![5, 0, 1, 1, 6, 6, 5, 0];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_eleven_digits_without_zero_and_carry() {
        let value: Vec<usize> = vec![5, 12, 17, 14, 13, 8, 11, 26, 12, 10, 1];
        let expected: Vec<usize> = vec![5, 2, 8, 5, 4, 9, 1, 7, 4, 1, 2];

        let result: Vec<usize> = break_down_subtotal(&value);

        assert_eq!(expected, result);
    }
}
