//! Breakdown algorithm for the long multiplication table.

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
pub fn break_down_subtotal(addition: &[usize]) -> Vec<usize> {
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
