//! Length utilities for the long multiplication algorithm.

/// Get the length (digits) of a number.
///
/// Given a number, this function returns the length in digits
/// of that number.
/// - If the number is a unit, it will return the value of one.
/// - If the number is a dozen, it will return the value of two.
/// - If the number is a hundred, it will return the value of three.
/// - So, successively, for the other numbers.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::length::get_number_length;
///
/// assert_eq!(get_number_length(3), 1);
/// assert_eq!(get_number_length(1234567890), 10);
/// ```
pub fn get_number_length(number: usize) -> usize {
    (number.checked_ilog10().unwrap_or(0) + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    // # -----------------------------------------------------------------------
    // # Function: get_number_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_number_length_for_one_digit() {
        let number: usize = 5;
        let length: usize;
        let expected: usize = 1;

        length = get_number_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_two_digit() {
        let number: usize = 38;
        let length: usize;
        let expected: usize = 2;

        length = get_number_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_three_digit() {
        let number: usize = 376;
        let length: usize;
        let expected: usize = 3;

        length = get_number_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_five_digit() {
        let number: usize = 95173;
        let length: usize;
        let expected: usize = 5;

        length = get_number_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_eleven_digit() {
        let number: usize = 12345678901;
        let length: usize;
        let expected: usize = 11;

        length = get_number_length(number);

        assert_eq!(expected, length);
    }
}
