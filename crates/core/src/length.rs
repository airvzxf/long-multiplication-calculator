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

/// Get the length (digits) of a string.
///
/// Given a string, this function returns the length in digits
/// of that string.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::length::get_string_length;
///
/// assert_eq!(get_string_length("3"), 1);
/// assert_eq!(get_string_length("1234567890"), 10);
/// ```
pub fn get_string_length(number: &str) -> usize {
    number.len()
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
/// # Examples
///
/// ```
/// use long_multiplication_core::length::get_numbers_length;
///
/// assert_eq!(get_numbers_length(6, 8), 2);
/// assert_eq!(get_numbers_length(1234567890, 12345), 15);
/// ```
#[allow(dead_code)]
pub fn get_numbers_length(number_a: usize, number_b: usize) -> usize {
    let number_a_len: usize = get_number_length(number_a);
    let number_b_len: usize = get_number_length(number_b);

    number_a_len + number_b_len
}

/// Get the length (digits) of two joined strings.
///
/// Given two strings, this function returns the length in digits
/// of both strings.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::length::get_strings_length;
///
/// assert_eq!(get_strings_length("6", "8"), 2);
/// assert_eq!(get_strings_length("1234567890", "12345"), 15);
/// ```
pub fn get_strings_length(number_a: &str, number_b: &str) -> usize {
    let number_a_len: usize = get_string_length(number_a);
    let number_b_len: usize = get_string_length(number_b);

    number_a_len + number_b_len
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

    // # -----------------------------------------------------------------------
    // # Function: get_string_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_string_length_for_one_digit() {
        let number: &str = "5";
        let length: usize;
        let expected: usize = 1;

        length = get_string_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_string_length_for_two_digit() {
        let number: &str = "38";
        let length: usize;
        let expected: usize = 2;

        length = get_string_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_string_length_for_three_digit() {
        let number: &str = "376";
        let length: usize;
        let expected: usize = 3;

        length = get_string_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_string_length_for_five_digit() {
        let number: &str = "95173";
        let length: usize;
        let expected: usize = 5;

        length = get_string_length(number);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_string_length_for_eleven_digit() {
        let number: &str = "12345678901";
        let length: usize;
        let expected: usize = 11;

        length = get_string_length(number);

        assert_eq!(expected, length);
    }

    // # -----------------------------------------------------------------------
    // # Function: get_numbers_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_numbers_length_for_two_digit() {
        let number_a: usize = 7;
        let number_b: usize = 9;
        let length: usize;
        let expected: usize = 2;

        length = get_numbers_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_three_digit() {
        let number_a: usize = 59;
        let number_b: usize = 7;
        let length: usize;
        let expected: usize = 3;

        length = get_numbers_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_five_digit() {
        let number_a: usize = 53;
        let number_b: usize = 824;
        let length: usize;
        let expected: usize = 5;

        length = get_numbers_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_eleven_digit() {
        let number_a: usize = 123456;
        let number_b: usize = 54321;
        let length: usize;
        let expected: usize = 11;

        length = get_numbers_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    // # -----------------------------------------------------------------------
    // # Function: get_strings_length
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_strings_length_for_two_digit() {
        let number_a: &str = "7";
        let number_b: &str = "9";
        let length: usize;
        let expected: usize = 2;

        length = get_strings_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_strings_length_for_three_digit() {
        let number_a: &str = "59";
        let number_b: &str = "7";
        let length: usize;
        let expected: usize = 3;

        length = get_strings_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_strings_length_for_five_digit() {
        let number_a: &str = "53";
        let number_b: &str = "824";
        let length: usize;
        let expected: usize = 5;

        length = get_strings_length(number_a, number_b);

        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_strings_length_for_eleven_digit() {
        let number_a: &str = "123456";
        let number_b: &str = "54321";
        let length: usize;
        let expected: usize = 11;

        length = get_strings_length(number_a, number_b);

        assert_eq!(expected, length);
    }
}
