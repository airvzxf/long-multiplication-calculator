/// Get the length (digits) of a number.
///
/// Given a number, this function returns the length in digits
/// of that number.
/// - If the number is a unit, it will return the value of one.
/// - If the number is a dozen, it will return the value of two.
/// - If the number is a hundred, it will return the value of three.
/// - So, successively, for the other numbers.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let number: usize = 3;
/// let expected: usize = 1;
///
/// use long_multiplication_command_line::breakdown::get_number_length;
/// let length: usize = get_number_length(number);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```rust
/// let number: usize = 1234567890;
/// let expected: usize = 10;
///
/// use long_multiplication_command_line::breakdown::get_number_length;
/// let length: usize = get_number_length(number);
///
/// assert_eq!(expected, length);
/// ```
pub fn get_number_length(number: usize) -> usize {
    return (number.checked_ilog10().unwrap_or(0) + 1) as usize;
}

/// Get the length (digits) of two joined numbers.
///
/// Given two numbers, this function returns the length in digits
/// of these numbers.
/// - If the join of the numbers is a dozen, it will return the value of two.
/// - If the join of the numbers is a hundred, it will return the value of three.
/// - If the join of the numbers is a thousand, it will return the value of four.
/// - So, successively, for the other numbers.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let number_a: usize = 6;
/// let number_b: usize = 8;
/// let expected: usize = 2;
///
/// use long_multiplication_command_line::breakdown::get_numbers_length;
/// let length: usize = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```rust
/// let number_a: usize = 1234567890;
/// let number_b: usize = 12345;
/// let expected: usize = 15;
///
/// use long_multiplication_command_line::breakdown::get_numbers_length;
/// let length: usize = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
pub fn get_numbers_length(number_a: usize, number_b: usize) -> usize {
    let number_a_len: usize = get_number_length(number_a);
    let number_b_len: usize = get_number_length(number_b);

    return number_a_len + number_b_len;
}

/// Get a list of the sum of the rows for each column.
///
/// Given two numbers that are multiplied, it gets the
/// multiplication result (units and carriers) for each
/// multiplicand by each multiplier.
/// This method sums each row for each column and returns
/// a list with these sums split by columns.
///
/// The size of the list of the sums is the maximum possible
/// number of columns of the product for the number of digits
/// for multiplicand plus multiplier.
///
/// This starts from left to right; on the right, we have
/// the units, or the first column, then the second column,
/// which is the dozens. So on until you reach the last column.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 2;
/// let multiplier: usize = 3;
/// let addition: Vec<usize>;
/// let expected_addition: Vec<usize> = vec![6, 0];
///
/// use long_multiplication_command_line::breakdown::break_down_addition;
/// addition = break_down_addition(multiplicand, multiplier);
///
/// assert_eq!(expected_addition, addition);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let addition: Vec<usize>;
/// let expected_addition: Vec<usize> = vec![8, 13, 2, 0];
///
/// use long_multiplication_command_line::breakdown::break_down_addition;
/// addition = break_down_addition(multiplicand, multiplier);
///
/// assert_eq!(expected_addition, addition);
/// ```
pub fn break_down_addition(multiplicand: usize, multiplier: usize) -> Vec<usize> {
    let multiplicand_len: usize = get_number_length(multiplicand);
    let length: usize = get_numbers_length(multiplicand, multiplier);
    let step: usize = multiplicand_len;

    let units: Vec<usize>;
    let carriers: Vec<usize>;
    (units, carriers) = break_down_multiplication(multiplicand, multiplier);

    let mut addition: Vec<usize> = Vec::new();
    for _ in 0..length {
        addition.push(0);
    }

    let mut iteration: usize = 0;
    let total_units = units.len();
    for start in (0..total_units).step_by(step) {
        for sub_index in start..start + step {
            let carry_index = start + step + iteration - sub_index;
            let carry = carriers[sub_index];
            addition[carry_index] += carry;
            let unit_index = carry_index - 1;
            let unit = units[sub_index];
            addition[unit_index] += unit;
        }
        iteration += 1;
    }

    let addition: Vec<usize> = addition;
    return addition;
}

/// Break down the multiplication to get information of the long multiplication.
///
/// Using the long multiplication method we get the information for each digit
/// of the multiplicand by each digit of the multiplier. The information is
/// the sub-product and the carriers for each multiplicand by multiplier.
///
/// This information (sub-product and the carriers) is returned as a collection
/// of vectors.
///
/// Examples
/// --------
///
/// Example #1
///
/// Algorithm:
/// ```text
///    2 5
///  x   3
/// ━━━━━━━
///  0 1    Carriers: 6 x 3 and 6 x 1
/// ┈┈┈┈┈┈┈
///    6 5  Sub-products: 6 x 3 and 6 x 1
/// ━━━━━━━
///  0 0    Carriers: sum of column 1, 2, 3 and 4
/// ┈┈┈┈┈┈┈
///  0 7 5  Product
/// ```
///
/// Code:
/// ```rust
/// let multiplicand: usize = 25;
/// let multiplier: usize = 3;
/// let operation_unit: Vec<usize>;
/// let operation_carry: Vec<usize>;
/// let expected_unit: Vec<usize> = vec![6, 5];
/// let expected_carry: Vec<usize> = vec![0, 1];
///
/// use long_multiplication_command_line::breakdown::break_down_multiplication;
/// (
///     operation_unit,
///     operation_carry
/// ) = break_down_multiplication(multiplicand, multiplier);
///
/// assert_eq!(expected_unit, operation_unit);
/// assert_eq!(expected_carry, operation_carry);
/// ```
///
/// Example #2
///
/// Algorithm:
/// ```text
///      1 3
///  x   2 6
/// ━━━━━━━━━
///    0 1    Carriers: 6 x 3 and 6 x 1
/// ┈┈┈┈┈┈┈┈┈
///      6 8  Sub-products: 6 x 3 and 6 x 1
/// ─────────
///  0 0      Carriers: 2 x 3 and 2 x 1
/// ┈┈┈┈┈┈┈┈┈
///    2 6    Sub-products: 2 x 3 and 2 x 1
/// ━━━━━━━━━
///  0 1 0    Carriers: sum of column 1, 2, 3 and 4
/// ┈┈┈┈┈┈┈┈┈
///  0 3 3 8  Product
/// ```
///
/// Code:
/// ```rust
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let operation_unit: Vec<usize>;
/// let operation_carry: Vec<usize>;
/// let expected_unit: Vec<usize> = vec![6, 8, 2, 6];
/// let expected_carry: Vec<usize> = vec![0, 1, 0, 0];
///
/// use long_multiplication_command_line::breakdown::break_down_multiplication;
/// (
///     operation_unit,
///     operation_carry
/// ) = break_down_multiplication(multiplicand, multiplier);
///
/// assert_eq!(expected_unit, operation_unit);
/// assert_eq!(expected_carry, operation_carry);
/// ```
pub fn break_down_multiplication(multiplicand: usize, multiplier: usize) -> (Vec<usize>, Vec<usize>) {
    let mut operation_unit: Vec<usize> = Vec::new();
    let mut operation_carry: Vec<usize> = Vec::new();

    for a in multiplier.to_string().chars().rev() {
        let mut units = Vec::new();
        let mut carriers = Vec::new();
        for b in multiplicand.to_string().chars().rev() {
            let multiplicand_digit = a as usize - 0x30;
            let multiplier_digit = b as usize - 0x30;
            let product = multiplicand_digit * multiplier_digit;
            let unit = product % 10;
            let carry = product / 10;
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

    return (operation_unit, operation_carry);
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
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let value: Vec<usize> = vec![6, 0];
/// let expected: Vec<usize> = vec![6, 0];
///
/// use long_multiplication_command_line::breakdown::break_down_subtotal;
/// let result: Vec<usize> = break_down_subtotal(&value);
///
/// assert_eq!(expected, result);
/// ```
///
/// Example #2
/// ```rust
/// let value: Vec<usize> = vec![1, 10, 19, 27, 27, 27, 26, 17, 8];
/// let expected: Vec<usize> = vec![1, 0, 10, 8, 9, 9, 8, 9, 9];
///
/// use long_multiplication_command_line::breakdown::break_down_subtotal;
/// let result: Vec<usize> = break_down_subtotal(&value);
///
/// assert_eq!(expected, result);
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

    let new_addition: Vec<usize> = new_addition;
    return new_addition;
}

#[cfg(test)]
mod tests {
    use super::*;

    // # -----------------------------------------------------------------------
    // # Function: get_number_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_number_length_for_one_digit() {
        // Arrange
        let number: usize = 5;
        let expected: usize = 1;

        // Action
        let length: usize = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_two_digit() {
        // Arrange
        let number: usize = 38;
        let expected: usize = 2;

        // Action
        let length: usize = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_three_digit() {
        // Arrange
        let number: usize = 376;
        let expected: usize = 3;

        // Action
        let length: usize = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_five_digit() {
        // Arrange
        let number: usize = 95173;
        let expected: usize = 5;

        // Action
        let length: usize = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_eleven_digit() {
        // Arrange
        let number: usize = 12345678901;
        let expected: usize = 11;

        // Action
        let length: usize = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    // # -----------------------------------------------------------------------
    // # Function: get_numbers_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_numbers_length_for_two_digit() {
        // Arrange
        let number_a: usize = 7;
        let number_b: usize = 9;
        let expected: usize = 2;

        // Action
        let length: usize = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_three_digit() {
        // Arrange
        let number_a: usize = 59;
        let number_b: usize = 7;
        let expected: usize = 3;

        // Action
        let length: usize = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_five_digit() {
        // Arrange
        let number_a: usize = 53;
        let number_b: usize = 824;
        let expected: usize = 5;

        // Action
        let length: usize = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_eleven_digit() {
        // Arrange
        let number_a: usize = 123456;
        let number_b: usize = 54321;
        let expected: usize = 11;

        // Action
        let length: usize = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_multiplication_with_three_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 25;
        let multiplier: usize = 3;
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 5];
        let expected_carry: Vec<usize> = vec![0, 1];

        // Action
        (
            operation_unit,
            operation_carry
        ) = break_down_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_three_digits_multiplier_is_greater() {
        // Arrange
        let multiplicand: usize = 3;
        let multiplier: usize = 25;
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![5, 6];
        let expected_carry: Vec<usize> = vec![1, 0];

        // Action
        (
            operation_unit,
            operation_carry
        ) = break_down_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 8, 2, 6];
        let expected_carry: Vec<usize> = vec![0, 1, 0, 0];

        // Action
        (
            operation_unit,
            operation_carry
        ) = break_down_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn test_break_down_multiplication_with_six_digit() {
        // Arrange
        let multiplicand: usize = 123;
        let multiplier: usize = 456;
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 2, 8, 5, 0, 5, 4, 8, 2];
        let expected_carry: Vec<usize> = vec![0, 1, 1, 0, 1, 1, 0, 0, 1];

        // Action
        (
            operation_unit,
            operation_carry
        ) = break_down_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_addition
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_addition_product_one_digit() {
        // Arrange
        let multiplicand: usize = 2;
        let multiplier: usize = 3;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 0];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_product_two_digits() {
        // Arrange
        let multiplicand: usize = 9;
        let multiplier: usize = 8;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![2, 7];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_three_digits() {
        // Arrange
        let multiplicand: usize = 37;
        let multiplier: usize = 8;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_three_digits_switch() {
        // Arrange
        let multiplicand: usize = 8;
        let multiplier: usize = 37;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 13, 2, 0];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_six_digit() {
        // Arrange
        let multiplicand: usize = 123;
        let multiplier: usize = 456;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 8, 10, 15, 4, 0];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_eleven_digits_multiplier_is_greater() {
        // Arrange
        let multiplicand: usize = 78924358;
        let multiplier: usize = 357;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_break_down_addition_with_eleven_digits_multiplier_is_less() {
        // Arrange
        let multiplicand: usize = 357;
        let multiplier: usize = 78924358;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        // Action
        addition = break_down_addition(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_subtotal
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_subtotal_result_two_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![6, 0];
        let expected: Vec<usize> = vec![6, 0];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_two_digits_without_zero() {
        // Arrange
        let value: Vec<usize> = vec![2, 4];
        let expected: Vec<usize> = vec![2, 4];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_three_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![2, 9, 0];
        let expected: Vec<usize> = vec![2, 9, 0];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_three_digits_without_zero() {
        // Arrange
        let value: Vec<usize> = vec![5, 8, 2];
        let expected: Vec<usize> = vec![5, 8, 2];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![4, 8, 4, 0];
        let expected: Vec<usize> = vec![4, 8, 4, 0];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_with_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![4, 11, 6, 0];
        let expected: Vec<usize> = vec![4, 1, 7, 0];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_four_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![6, 12, 6, 2];
        let expected: Vec<usize> = vec![6, 2, 7, 2];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_nine_digits_with_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![1, 10, 19, 27, 27, 27, 26, 17, 8];
        let expected: Vec<usize> = vec![1, 0, 10, 8, 9, 9, 8, 9, 9];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_nine_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![5, 10, 10, 10, 5, 16, 4, 0];
        let expected: Vec<usize> = vec![5, 0, 1, 1, 6, 6, 5, 0];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_subtotal_result_eleven_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![5, 12, 17, 14, 13, 8, 11, 26, 12, 10, 1];
        let expected: Vec<usize> = vec![5, 2, 8, 5, 4, 9, 1, 7, 4, 1, 2];

        // Action
        let result: Vec<usize> = break_down_subtotal(&value);

        // Assert
        assert_eq!(expected, result);
    }
}
