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
/// let length: usize;
/// let expected: usize = 1;
///
/// use long_multiplication_command_line::length::get_number_length;
/// length = get_number_length(number);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```rust
/// let number: usize = 1234567890;
/// let length: usize;
/// let expected: usize = 10;
///
/// use long_multiplication_command_line::length::get_number_length;
/// length = get_number_length(number);
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
/// let length: usize;
/// let expected: usize = 2;
///
/// use long_multiplication_command_line::length::get_numbers_length;
/// length = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```rust
/// let number_a: usize = 1234567890;
/// let number_b: usize = 12345;
/// let length: usize;
/// let expected: usize = 15;
///
/// use long_multiplication_command_line::length::get_numbers_length;
/// length = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
pub fn get_numbers_length(number_a: usize, number_b: usize) -> usize {
    let number_a_len: usize = get_number_length(number_a);
    let number_b_len: usize = get_number_length(number_b);

    return number_a_len + number_b_len;
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
        let length: usize;
        let expected: usize = 1;

        // Action
        length = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_two_digit() {
        // Arrange
        let number: usize = 38;
        let length: usize;
        let expected: usize = 2;

        // Action
        length = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_three_digit() {
        // Arrange
        let number: usize = 376;
        let length: usize;
        let expected: usize = 3;

        // Action
        length = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_five_digit() {
        // Arrange
        let number: usize = 95173;
        let length: usize;
        let expected: usize = 5;

        // Action
        length = get_number_length(number);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_number_length_for_eleven_digit() {
        // Arrange
        let number: usize = 12345678901;
        let length: usize;
        let expected: usize = 11;

        // Action
        length = get_number_length(number);

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
        let length: usize;
        let expected: usize = 2;

        // Action
        length = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_three_digit() {
        // Arrange
        let number_a: usize = 59;
        let number_b: usize = 7;
        let length: usize;
        let expected: usize = 3;

        // Action
        length = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_five_digit() {
        // Arrange
        let number_a: usize = 53;
        let number_b: usize = 824;
        let length: usize;
        let expected: usize = 5;

        // Action
        length = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }

    #[test]
    fn test_get_numbers_length_for_eleven_digit() {
        // Arrange
        let number_a: usize = 123456;
        let number_b: usize = 54321;
        let length: usize;
        let expected: usize = 11;

        // Action
        length = get_numbers_length(number_a, number_b);

        // Assert
        assert_eq!(expected, length);
    }
}
