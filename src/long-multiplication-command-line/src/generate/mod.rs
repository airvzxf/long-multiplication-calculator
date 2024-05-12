use std::ops::Index;

/// Store the symbol description of the long multiplication.
///
/// It generates the table symbols for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let expected: &str = "\n\
///                       Symbols\n\
///                       =======\n\
///                       Pos. = Position.\n\
///                       Ops. = Operations of the long multiplication.\n\
///                       Sum. = Sum of each column of the multiplication.\n\
///                       Sub n. = Subtotal of the last sum.\n\
///                       Pro. = Product of the multiplication.\n\
///                       n ^ = Carry-over.\n\
///                       n R = The row number.\n\
///                       n C = The column number of the sum of the rows.\n\
///                       * Replace 'n' for a number.\n\
///                       P = The product of multiplication.\n\
///                       \n";
/// let mut text: String = String::from("");
///
/// use long_multiplication_command_line::generate;
/// generate::symbols(&mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn symbols(text: &mut String) {
    text.push('\n');
    text.push_str("Symbols\n");
    text.push_str("=======\n");
    text.push_str("Pos. = Position.\n");
    text.push_str("Ops. = Operations of the long multiplication.\n");
    text.push_str("Sum. = Sum of each column of the multiplication.\n");
    text.push_str("Sub n. = Subtotal of the last sum.\n");
    text.push_str("Pro. = Product of the multiplication.\n");
    text.push_str("n ^ = Carry-over.\n");
    text.push_str("n R = The row number.\n");
    text.push_str("n C = The column number of the sum of the rows.\n");
    text.push_str("* Replace 'n' for a number.\n");
    text.push_str("P = The product of multiplication.\n");
    text.push('\n');
}

/// Store the top border of the long multiplication.
///
/// It generates the table top-border for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 2;
/// let multiplier: usize = 5;
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━┓\n";
///
/// use long_multiplication_command_line::generate;
/// generate::top_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 2;
/// let multiplier: usize = 75;
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━━━━━┓\n";
///
/// use long_multiplication_command_line::generate;
/// generate::top_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn top_border(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);

    // Create first row
    text.push('┏');
    for _ in 1..(length * 3) + length {
        text.push('━');
    }
    text.push('┓');
    text.push('\n');
}

/// Store the bottom border of the long multiplication.
///
/// It generates the table bottom-border for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 2;
/// let multiplier: usize = 5;
/// let mut text: String = String::from("");
/// let expected: &str = "┗━━━┷━━━┛\n";
///
/// use long_multiplication_command_line::generate;
/// generate::bottom_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 12;
/// let multiplier: usize = 57;
/// let mut text: String = String::from("");
/// let expected: &str = "┗━━━┷━━━┷━━━┷━━━┛\n";
///
/// use long_multiplication_command_line::generate;
/// generate::bottom_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn bottom_border(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);

    // Create first row
    text.push('┗');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┷');
    }
    text.push('┛');
    text.push('\n');
}

/// Store the position title of the long multiplication.
///
/// It generates the table position-title for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 7;
/// let multiplier: usize = 8;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.   ┃\n\
///                       ┠┄┄┄┬┄┄┄┨\n\
///                       ┃ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::position_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 123;
/// let multiplier: usize = 456;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.                   ┃\n\
///                       ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
///                       ┃ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::position_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn position_title(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);

    // Create first row
    text.push_str("┃Pos.");
    for _ in 1..(length * 3) + length - 4 {
        text.push(' ');
    }
    text.push('┃');
    text.push('\n');

    // Create second row
    text.push('┠');
    for n in 1..length + 1 {
        text.push_str("┄┄┄");
        if n == length {
            break;
        }
        text.push('┬');
    }
    text.push('┨');
    text.push('\n');

    // Create third row
    text.push('┃');
    for n in 1..length + 1 {
        let number = length + 1 - n;
        if number < 100 {
            text.push(' ');
        }
        text.push_str(&*number.to_string());
        if number < 10 {
            text.push(' ');
        }
        if n == length {
            break;
        }
        text.push('│');
    }
    text.push('┃');
    text.push('\n');

    // Create fourth row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┷');
    }
    text.push('┫');
    text.push('\n');
}

/// Store the operation title of the long multiplication.
///
/// It generates the table operation-title for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 73;
/// let multiplier: usize = 4;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.       ┃\n\
///                       ┣━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operation_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 123;
/// let multiplier: usize = 45;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operation_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn operation_title(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);

    // Create first row
    text.push_str("┃Ops.");
    for _ in 1..(length * 3) + length - 4 {
        text.push(' ');
    }
    text.push('┃');
    text.push('\n');

    // Create second row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┯');
    }
    text.push('┫');
    text.push('\n');
}

/// Store the multiplication section of the long multiplication.
///
/// It generates the table multiplication-section for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 3;
/// let multiplier: usize = 5;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 3 ┃\n\
///                       ┃ x │ 5 ┃\n\
///                       ┣━━━┿━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::multiplication(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 12;
/// let multiplier: usize = 345;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │   │   │ 1 │ 2 ┃\n\
///                       ┃ x │   │ 3 │ 4 │ 5 ┃\n\
///                       ┣━━━┿━━━┿━━━┿━━━┿━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::multiplication(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn multiplication(multiplicand: usize, multiplier: usize, text: &mut String) {
    let multiplicand_len: usize = get_number_length(multiplicand);
    let multiplier_len: usize = get_number_length(multiplier);
    let length: usize = multiplicand_len + multiplier_len;

    // Create first row
    text.push('┃');
    for n in 0..(length - multiplicand_len) {
        text.push_str("   ");
        if n == length {
            break;
        }
        text.push('│');
    }

    for i in multiplicand.to_string().chars() {
        text.push(' ');
        text.push(i);
        text.push_str(" │");
    }
    text.pop();
    text.push('┃');
    text.push('\n');

    // Create second row
    text.push('┃');
    text.push_str(" x │");
    for n in 0..(length - multiplier_len - 1) {
        text.push_str("   ");
        if n == length {
            break;
        }
        text.push('│');
    }

    for i in multiplier.to_string().chars() {
        text.push(' ');
        text.push(i);
        text.push_str(" │");
    }
    text.pop();
    text.push('┃');
    text.push('\n');

    // Create third row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┿');
    }
    text.push('┫');
    text.push('\n');
}

/// Store the operations section of the long multiplication.
///
/// It generates the table operations-section for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 9;
/// let multiplier: usize = 3;
/// let mut text: String = String::from("");
/// let expected: &str = "┃ 2 │   ┃ 1 ^\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 7 ┃ 1 R\n\
///                       ┣━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operations(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 579;
/// let multiplier: usize = 48;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 4 │ 5 │ 7 │   ┃ 1 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │ 0 │ 6 │ 2 ┃ 1 R\n\
///                       ┠───┼───┼───┼───┼───┨\n\
///                       ┃ 2 │ 2 │ 3 │   │   ┃ 2 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 0 │ 8 │ 6 │   ┃ 2 R\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operations(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn operations(multiplicand: usize, multiplier: usize, text: &mut String) {
    let multiplicand_len: usize = get_number_length(multiplicand);
    let length: usize = get_numbers_length(multiplicand, multiplier);

    let operation_unit: Vec<usize>;
    let operation_carry: Vec<usize>;
    (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

    let step: usize = multiplicand_len;

    let max_group_rows = operation_unit.len() / step;

    let mut iteration: usize = 1;
    for start in (0..operation_unit.len()).step_by(step) {
        let start: usize = start;
        let end: usize = start + step;

        let slice = &operation_carry[start..end];

        // Create first row
        text.push('┃');
        let start_spaces = length - step - iteration;
        for _ in 0..start_spaces {
            text.push_str("   │");
        }
        for n in slice {
            text.push(' ');
            text.push_str(&*n.to_string());
            text.push(' ');
            text.push('│');
        }
        let end_spaces = iteration;
        for n in 0..end_spaces {
            text.push_str("   ");
            if n < end_spaces - 1 {
                text.push('│');
            }
        }
        text.push_str("┃ ");
        let row: String = iteration.to_string();
        text.push_str(&*row);
        text.push_str(" ^\n");

        // Create second row
        text.push('┠');
        for n in 1..length + 1 {
            text.push_str("┈┈┈");
            if n == length {
                break;
            }
            text.push('┼');
        }
        text.push('┨');
        text.push('\n');

        let slice = &operation_unit[start..end];

        // Create third row
        text.push('┃');
        let start_spaces = length - step - iteration + 1;
        for _ in 0..start_spaces {
            text.push_str("   │");
        }
        for n in slice {
            text.push(' ');
            text.push_str(&*n.to_string());
            text.push(' ');
            text.push('│');
        }
        let end_spaces = iteration - 1;
        if end_spaces == 0 {
            text.pop();
        }
        for n in 0..end_spaces {
            text.push_str("   ");
            if n < end_spaces - 1 {
                text.push('│');
            }
        }
        text.push_str("┃ ");
        let row: String = iteration.to_string();
        text.push_str(&*row);
        text.push_str(" R\n");

        // Create fourth row
        if iteration == max_group_rows {
            break;
        }
        text.push('┠');
        for n in 1..length + 1 {
            text.push_str("───");
            if n == length {
                break;
            }
            text.push('┼');
        }
        text.push('┨');
        text.push('\n');

        iteration += 1;
    }

    // Create final row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┷');
    }
    text.push('┫');
    text.push('\n');
}

/// Store the sum title of the long multiplication.
///
/// It generates the table sum-title for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 13;
/// let multiplier: usize = 8;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Sum.       ┃\n\
///                       ┣━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::sum_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 951;
/// let multiplier: usize = 46;
/// let mut text: String = String::from("");
/// let expected: &str = "┃Sum.               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::sum_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn sum_title(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);

    // Create first row
    text.push_str("┃Sum.");
    for _ in 1..(length * 3) + length - 4 {
        text.push(' ');
    }
    text.push('┃');
    text.push('\n');

    // Create second row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┯');
    }
    text.push('┫');
    text.push('\n');
}

/// Store the long-sum section of the long multiplication.
///
/// It generates the table long-sum-section for the
/// long multiplication and stores it in a text variable.
///
/// It means that sums the rows for each column.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 3;
/// let multiplier: usize = 2;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 6 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃ 0 │   ┃ 2 C\n\
///                       ┣━━━┷━━━┫\n\
///                       ┃Pro.   ┃\n\
///                       ┣━━━┯━━━┫\n\
///                       ┃ 0 │ 6 ┃ P\n";
///
/// use long_multiplication_command_line::generate;
/// generate::long_sum(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │   │   │ 8 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 1 │ 3 │   ┃ 2 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 2 │   │   ┃ 3 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃ 0 │   │   │   ┃ 4 C\n\
///                       ┣━━━┷━━━┷━━━┷━━━┫\n\
///                       ┃Pro.           ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┫\n\
///                       ┃ 0 │ 3 │ 3 │ 8 ┃ P\n";
///
/// use long_multiplication_command_line::generate;
/// generate::long_sum(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn long_sum(multiplicand: usize, multiplier: usize, text: &mut String) {
    let additions: Vec<usize> = break_down_addition_of_multiplication(multiplicand, multiplier);

    let length: usize = get_numbers_length(multiplicand, multiplier);
    let mut iteration: usize = 0;

    for row in &additions {
        // Create first row
        let row_size: usize = get_number_length(*row);
        text.push('┃');
        for _ in 0..(length - iteration - row_size) {
            text.push_str("   ");
            text.push('│');
        }

        for i in row.to_string().chars() {
            text.push(' ');
            text.push(i);
            text.push_str(" │");
        }
        text.pop();

        if iteration > 0 {
            text.push('│');
        }
        for n in 0..iteration {
            text.push_str("   ");
            if n == iteration - 1 {
                break;
            }
            text.push('│');
        }
        iteration += 1;
        text.push_str("┃ ");
        let row: String = iteration.to_string();
        text.push_str(&*row);
        text.push_str(" C");
        text.push('\n');

        // Create second row
        if iteration == length {
            break;
        }
        text.push('┠');
        for n in 1..length + 1 {
            text.push_str("┈┈┈");
            if n == length {
                break;
            }
            text.push('┼');
        }
        text.push('┨');
        text.push('\n');
    }

    let mut sub_addition: Vec<usize> = break_down_addition(&additions);
    let mut sub_index: usize = 0;
    loop {
        let mut decimals: bool = false;
        for number in &sub_addition {
            if number > &9 {
                decimals = true;
                break;
            }
        }

        if !decimals {
            break;
        }

        // Create the first row of the sub-addition
        text.push('┣');
        for n in 1..length + 1 {
            text.push_str("━━━");
            if n == length {
                break;
            }
            text.push('┷');
        }
        text.push('┫');
        text.push('\n');

        // Create the second row of the sub-addition
        text.push_str("┃Sub ");
        sub_index += 1;
        text.push_str(&*sub_index.to_string());
        text.push('.');
        for _ in 1..(length * 3) + length - 6 {
            text.push(' ');
        }
        text.push('┃');
        text.push('\n');

        // Create the third row of the sub-addition
        text.push('┣');
        for n in 1..length + 1 {
            text.push_str("━━━");
            if n == length {
                break;
            }
            text.push('┯');
        }
        text.push('┫');
        text.push('\n');

        // Create the sum of columns
        let mut iteration: usize = 0;
        for row in &sub_addition {
            // Create first row
            let row_size: usize = get_number_length(*row);
            text.push('┃');
            for _ in 0..(length - iteration - row_size) {
                text.push_str("   ");
                text.push('│');
            }

            for i in row.to_string().chars() {
                text.push(' ');
                text.push(i);
                text.push_str(" │");
            }
            text.pop();

            if iteration > 0 {
                text.push('│');
            }
            for n in 0..iteration {
                text.push_str("   ");
                if n == iteration - 1 {
                    break;
                }
                text.push('│');
            }
            iteration += 1;
            text.push_str("┃ ");
            let row: String = iteration.to_string();
            text.push_str(&*row);
            text.push_str(" C");
            text.push('\n');

            // Create second row
            if iteration == length {
                break;
            }
            text.push('┠');
            for n in 1..length + 1 {
                text.push_str("┈┈┈");
                if n == length {
                    break;
                }
                text.push('┼');
            }
            text.push('┨');
            text.push('\n');
        }

        sub_addition = break_down_addition(&sub_addition);
    }

    // Create last row
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┷');
    }
    text.push('┫');
    text.push('\n');

    // Create first row product title
    text.push_str("┃Pro.");
    for _ in 1..(length * 3) + length - 4 {
        text.push(' ');
    }
    text.push('┃');
    text.push('\n');

    // Create second row product title
    text.push('┣');
    for n in 1..length + 1 {
        text.push_str("━━━");
        if n == length {
            break;
        }
        text.push('┯');
    }
    text.push('┫');
    text.push('\n');

    // Create first row for product
    sub_addition.reverse();
    text.push('┃');
    for i in sub_addition {
        text.push(' ');
        text.push_str(&*i.to_string());
        text.push_str(" │");
    }
    text.pop();

    text.push_str("┃ P");
    text.push('\n');
}

/// Store the author section of the long multiplication.
///
/// It generates the table author-section for the
/// long multiplication and stores it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let mut text: String = String::from("");
/// let expected: &str = "\n\
///                       ---\n\
///                       Author: Israel Roldan\n\
///                       E-mail: israel.alberto.rv@gmail.com\n\
///                       License: GPL-3.0\n\
///                       Project: https://github.com/airvzxf/long-multiplication-calculator\n";
///
/// use long_multiplication_command_line::generate;
/// generate::author(&mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn author(text: &mut String) {
    text.push_str("\n");
    text.push_str("---\n");
    text.push_str("Author: Israel Roldan\n");
    text.push_str("E-mail: israel.alberto.rv@gmail.com\n");
    text.push_str("License: GPL-3.0\n");
    text.push_str("Project: https://github.com/airvzxf/long-multiplication-calculator\n");
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
/// ```text
/// let multiplicand: usize = 2;
/// let multiplier: usize = 3;
/// let addition: Vec<usize>;
/// let expected_addition: Vec<usize> = vec![0, 6];
///
/// addition = break_down_addition_of_multiplication(multiplicand, multiplier);
///
/// assert_eq!(expected_addition, addition);
/// ```
///
/// Example #2
/// ```text
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let addition: Vec<usize>;
/// let expected_addition: Vec<usize> = vec![0, 2, 13, 8];
///
/// addition = break_down_addition_of_multiplication(multiplicand, multiplier);
///
/// assert_eq!(expected_addition, addition);
/// ```
// TODO: Extract this private functions in other modules. Then make them public and call here.
// TODO: Rename break_down_addition_of_multiplication to break_down_addition
fn break_down_addition_of_multiplication(multiplicand: usize, multiplier: usize) -> Vec<usize> {
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
            let carry = carriers.index(sub_index);
            addition[carry_index] += carry;
            let unit_index = carry_index - 1;
            let unit = units.index(sub_index);
            addition[unit_index] += unit;
        }
        iteration += 1;
    }

    let addition: Vec<usize> = addition;
    return addition;
}

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
/// ```text
/// let number: usize = 3;
/// let expected: usize = 1;
///
/// let length: usize = get_number_length(number);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```text
/// let number: usize = 1234567890;
/// let expected: usize = 10;
///
/// let length: usize = get_number_length(number);
///
/// assert_eq!(expected, length);
/// ```
// TODO: Extract this private functions in other modules. Then make them public and call here.
fn get_number_length(number: usize) -> usize {
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
/// ```text
/// let number_a: usize = 6;
/// let number_b: usize = 8;
/// let expected: usize = 2;
///
/// let length: usize = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
///
/// Example #2
/// ```text
/// let number_a: usize = 1234567890;
/// let number_b: usize = 12345;
/// let expected: usize = 15;
///
/// let length: usize = get_numbers_length(number_a, number_b);
///
/// assert_eq!(expected, length);
/// ```
// TODO: Extract this private functions in other modules. Then make them public and call here.
fn get_numbers_length(number_a: usize, number_b: usize) -> usize {
    let number_a_len: usize = get_number_length(number_a);
    let number_b_len: usize = get_number_length(number_b);

    return number_a_len + number_b_len;
}

/// Breakdown the multiplication to get information of the long multiplication.
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
/// ```text
/// let multiplicand: usize = 25;
/// let multiplier: usize = 3;
/// let operation_unit: Vec<usize>;
/// let operation_carry: Vec<usize>;
/// let expected_unit: Vec<usize> = vec![6, 5];
/// let expected_carry: Vec<usize> = vec![0, 1];
///
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
/// ```text
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let operation_unit: Vec<usize>;
/// let operation_carry: Vec<usize>;
/// let expected_unit: Vec<usize> = vec![6, 8, 2, 6];
/// let expected_carry: Vec<usize> = vec![0, 1, 0, 0];
///
/// (
///     operation_unit,
///     operation_carry
/// ) = break_down_multiplication(multiplicand, multiplier);
///
/// assert_eq!(expected_unit, operation_unit);
/// assert_eq!(expected_carry, operation_carry);
/// ```
// TODO: Extract this private functions in other modules. Then make them public and call here.
fn break_down_multiplication(multiplicand: usize, multiplier: usize) -> (Vec<usize>, Vec<usize>) {
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
/// ```text
/// let value: Vec<usize> = vec![6, 0];
/// let expected: Vec<usize> = vec![6, 0];
///
/// let result: Vec<usize> = break_down_addition(&value);
///
/// assert_eq!(expected, result);
/// ```
///
/// Example #2
/// ```text
/// let value: Vec<usize> = vec![1, 10, 19, 27, 27, 27, 26, 17, 8];
/// let expected: Vec<usize> = vec![1, 0, 10, 8, 9, 9, 8, 9, 9];
///
/// let result: Vec<usize> = break_down_addition(&value);
///
/// assert_eq!(expected, result);
/// ```
// TODO: Extract this private functions in other modules. Then make them public and call here.
// TODO: Rename break_down_addition to break_down_subtotal
fn break_down_addition(addition: &Vec<usize>) -> Vec<usize> {
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
    // # Function: symbols
    // # -----------------------------------------------------------------------
    #[test]
    fn test_symbols_description() {
        // Arrange
        let mut text: String = String::from("");
        let expected: &str = "\n\
                              Symbols\n\
                              =======\n\
                              Pos. = Position.\n\
                              Ops. = Operations of the long multiplication.\n\
                              Sum. = Sum of each column of the multiplication.\n\
                              Sub n. = Subtotal of the last sum.\n\
                              Pro. = Product of the multiplication.\n\
                              n ^ = Carry-over.\n\
                              n R = The row number.\n\
                              n C = The column number of the sum of the rows.\n\
                              * Replace 'n' for a number.\n\
                              P = The product of multiplication.\n\
                              \n";

        // Action
        symbols(&mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: top_border
    // # -----------------------------------------------------------------------
    #[test]
    fn test_top_border_size_two_digits() {
        // Arrange
        let multiplicand: usize = 2;
        let multiplier: usize = 4;
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━┓\n";

        // Action
        top_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_three_digits() {
        // Arrange
        let multiplicand: usize = 12;
        let multiplier: usize = 3;
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━┓\n";

        // Action
        top_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_five_digits() {
        // Arrange
        let multiplicand: usize = 345;
        let multiplier: usize = 12;
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━━━━━━━━━┓\n";

        // Action
        top_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_twelve_digits() {
        // Arrange
        let multiplicand: usize = 123456;
        let multiplier: usize = 123456;
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n";

        // Action
        top_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: bottom_border
    // # -----------------------------------------------------------------------
    #[test]
    fn test_bottom_border_size_two_digits() {
        // Arrange
        let multiplicand: usize = 7;
        let multiplier: usize = 3;
        let mut text: String = String::from("");
        let expected: &str = "┗━━━┷━━━┛\n";

        // Action
        bottom_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_bottom_border_size_three_digits() {
        // Arrange
        let multiplicand: usize = 8;
        let multiplier: usize = 43;
        let mut text: String = String::from("");
        let expected: &str = "┗━━━┷━━━┷━━━┛\n";

        // Action
        bottom_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_bottom_border_size_five_digits() {
        // Arrange
        let multiplicand: usize = 519;
        let multiplier: usize = 43;
        let mut text: String = String::from("");
        let expected: &str = "┗━━━┷━━━┷━━━┷━━━┷━━━┛\n";

        // Action
        bottom_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_bottom_border_size_twelve_digits() {
        // Arrange
        let multiplicand: usize = 12;
        let multiplier: usize = 1234567890;
        let mut text: String = String::from("");
        let expected: &str = "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n";

        // Action
        bottom_border(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: position_title
    // # -----------------------------------------------------------------------
    #[test]
    fn test_position_title_size_two_digits() {
        // Arrange
        let multiplicand: usize = 6;
        let multiplier: usize = 3;
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.   ┃\n\
                              ┠┄┄┄┬┄┄┄┨\n\
                              ┃ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┫\n";

        // Action
        position_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_three_digits() {
        // Arrange
        let multiplicand: usize = 18;
        let multiplier: usize = 6;
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.       ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┫\n";

        // Action
        position_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_five_digits() {
        // Arrange
        let multiplicand: usize = 78;
        let multiplier: usize = 327;
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.               ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        position_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_eleven_digits() {
        // Arrange
        let multiplicand: usize = 123456;
        let multiplier: usize = 54321;
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.                                       ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 11│ 10│ 9 │ 8 │ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        position_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: operation_title
    // # -----------------------------------------------------------------------
    #[test]
    fn test_operation_title_size_two_digits() {
        // Arrange
        let multiplicand: usize = 9;
        let multiplier: usize = 1;
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.   ┃\n\
                              ┣━━━┯━━━┫\n";

        // Action
        operation_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_three_digits() {
        // Arrange
        let multiplicand: usize = 53;
        let multiplier: usize = 4;
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.       ┃\n\
                              ┣━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_five_digits() {
        // Arrange
        let multiplicand: usize = 53;
        let multiplier: usize = 618;
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_eleven_digits() {
        // Arrange
        let multiplicand: usize = 654321;
        let multiplier: usize = 12345;
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.                                       ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_multiplication_size_two_digits() {
        // Arrange
        let multiplicand: usize = 8;
        let multiplier: usize = 4;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 8 ┃\n\
                              ┃ x │ 4 ┃\n\
                              ┣━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_multiplication_size_three_digits() {
        // Arrange
        let multiplicand: usize = 2;
        let multiplier: usize = 37;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │ 2 ┃\n\
                              ┃ x │ 3 │ 7 ┃\n\
                              ┣━━━┿━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_multiplication_size_five_digits() {
        // Arrange
        let multiplicand: usize = 81;
        let multiplier: usize = 925;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │ 8 │ 1 ┃\n\
                              ┃ x │   │ 9 │ 2 │ 5 ┃\n\
                              ┣━━━┿━━━┿━━━┿━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_multiplication_size_eleven_digits() {
        // Arrange
        let multiplicand: usize = 12345;
        let multiplier: usize = 654321;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │   │   │ 1 │ 2 │ 3 │ 4 │ 5 ┃\n\
                              ┃ x │   │   │   │   │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_multiplication_multiplicand_bigger_than_a_multiplier() {
        // Arrange
        let multiplicand: usize = 1234;
        let multiplier: usize = 5;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 1 │ 2 │ 3 │ 4 ┃\n\
                              ┃ x │   │   │   │ 5 ┃\n\
                              ┣━━━┿━━━┿━━━┿━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_multiplication_multiplier_bigger_than_a_multiplicand() {
        // Arrange
        let multiplicand: usize = 8765;
        let multiplier: usize = 1234;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │ 8 │ 7 │ 6 │ 5 ┃\n\
                              ┃ x │   │   │   │ 1 │ 2 │ 3 │ 4 ┃\n\
                              ┣━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┫\n";

        // Action
        multiplication(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: operations
    // # -----------------------------------------------------------------------
    #[test]
    fn test_operations_with_three_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 25;
        let multiplier: usize = 3;
        let mut text: String = String::from("");
        let expected: &str = "┃ 0 │ 1 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 6 │ 5 ┃ 1 R\n\
                              ┣━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operations_with_three_digits_multiplicand_is_less() {
        // Arrange
        let multiplicand: usize = 3;
        let multiplier: usize = 25;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 1 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 5 ┃ 1 R\n\
                              ┠───┼───┼───┨\n\
                              ┃ 0 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 6 │   ┃ 2 R\n\
                              ┣━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operations_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 0 │ 1 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │ 8 ┃ 1 R\n\
                              ┠───┼───┼───┼───┨\n\
                              ┃ 0 │ 0 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 2 │ 6 │   ┃ 2 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operations_with_eleven_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 246802468;
        let multiplier: usize = 357;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │ 1 │ 2 │ 4 │ 5 │ 0 │ 1 │ 2 │ 4 │ 5 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 4 │ 8 │ 2 │ 6 │ 0 │ 4 │ 8 │ 2 │ 6 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 1 │ 2 │ 3 │ 4 │ 0 │ 1 │ 2 │ 3 │ 4 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 0 │ 1 │ 1 │ 2 │ 0 │ 0 │ 1 │ 1 │ 2 │   │   │   ┃ 3 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 6 │ 2 │ 8 │ 4 │ 0 │ 6 │ 2 │ 8 │ 4 │   │   ┃ 3 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operations_with_eleven_digits_multiplicand_is_less() {
        // Arrange
        let multiplicand: usize = 357;
        let multiplier: usize = 246802468;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │   │   │   │   │ 2 │ 4 │ 5 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 4 │ 0 │ 6 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 3 │ 4 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 8 │ 0 │ 2 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │ 1 │ 2 │ 2 │   │   │   ┃ 3 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 2 │ 0 │ 8 │   │   ┃ 3 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │ 0 │ 1 │ 1 │   │   │   │   ┃ 4 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 6 │ 0 │ 4 │   │   │   ┃ 4 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │ 0 │ 0 │ 0 │   │   │   │   │   ┃ 5 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 0 │ 0 │ 0 │   │   │   │   ┃ 5 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │ 2 │ 4 │ 5 │   │   │   │   │   │   ┃ 6 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 4 │ 0 │ 6 │   │   │   │   │   ┃ 6 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │ 1 │ 3 │ 4 │   │   │   │   │   │   │   ┃ 7 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 8 │ 0 │ 2 │   │   │   │   │   │   ┃ 7 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 1 │ 2 │ 2 │   │   │   │   │   │   │   │   ┃ 8 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 2 │ 0 │ 8 │   │   │   │   │   │   │   ┃ 8 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 0 │ 1 │ 1 │   │   │   │   │   │   │   │   │   ┃ 9 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 6 │ 0 │ 4 │   │   │   │   │   │   │   │   ┃ 9 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operations_with_thirteen_rows() {
        // Arrange
        let multiplicand: usize = 7;
        let multiplier: usize = 9876543210123;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │   │   │   │   │   │   │   │   │ 2 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │   │   │ 1 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 1 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │   │ 4 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 0 │   │   │   ┃ 3 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 7 │   │   ┃ 3 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 0 │   │   │   │   ┃ 4 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 0 │   │   │   ┃ 4 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │ 0 │   │   │   │   │   ┃ 5 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 7 │   │   │   │   ┃ 5 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │   │   │   │   │   │   ┃ 6 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 4 │   │   │   │   │   ┃ 6 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │ 2 │   │   │   │   │   │   │   ┃ 7 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │   │   │   │   │   │   ┃ 7 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │ 2 │   │   │   │   │   │   │   │   ┃ 8 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 8 │   │   │   │   │   │   │   ┃ 8 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │ 3 │   │   │   │   │   │   │   │   │   ┃ 9 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 5 │   │   │   │   │   │   │   │   ┃ 9 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │ 4 │   │   │   │   │   │   │   │   │   │   ┃ 10 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 2 │   │   │   │   │   │   │   │   │   ┃ 10 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │ 4 │   │   │   │   │   │   │   │   │   │   │   ┃ 11 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 9 │   │   │   │   │   │   │   │   │   │   ┃ 11 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 5 │   │   │   │   │   │   │   │   │   │   │   │   ┃ 12 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 6 │   │   │   │   │   │   │   │   │   │   │   │   │   ┃ 13 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 3 │   │   │   │   │   │   │   │   │   │   │   │   ┃ 13 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_breakdown_multiplication_with_three_digits_multiplicand_is_greater() {
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
    fn test_breakdown_multiplication_with_three_digits_multiplier_is_greater() {
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
    fn test_breakdown_multiplication_with_four_digit() {
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
    fn test_breakdown_multiplication_with_six_digit() {
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
    // # Function: sum_title
    // # -----------------------------------------------------------------------
    #[test]
    fn test_sum_title_size_two_digits() {
        // Arrange
        let multiplicand: usize = 4;
        let multiplier: usize = 2;
        let mut text: String = String::from("");
        let expected: &str = "┃Sum.   ┃\n\
                              ┣━━━┯━━━┫\n";

        // Action
        sum_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_sum_title_size_three_digits() {
        // Arrange
        let multiplicand: usize = 19;
        let multiplier: usize = 5;
        let mut text: String = String::from("");
        let expected: &str = "┃Sum.       ┃\n\
                              ┣━━━┯━━━┯━━━┫\n";

        // Action
        sum_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_sum_title_size_five_digits() {
        // Arrange
        let multiplicand: usize = 73;
        let multiplier: usize = 438;
        let mut text: String = String::from("");
        let expected: &str = "┃Sum.               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        sum_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_sum_title_size_eleven_digits() {
        // Arrange
        let multiplicand: usize = 123456;
        let multiplier: usize = 54321;
        let mut text: String = String::from("");
        let expected: &str = "┃Sum.                                       ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        sum_title(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_addition_of_multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_breakdown_addition_of_multiplication_product_one_digit() {
        // Arrange
        let multiplicand: usize = 2;
        let multiplier: usize = 3;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 0];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_product_two_digits() {
        // Arrange
        let multiplicand: usize = 9;
        let multiplier: usize = 8;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![2, 7];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_three_digits() {
        // Arrange
        let multiplicand: usize = 37;
        let multiplier: usize = 8;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_three_digits_switch() {
        // Arrange
        let multiplicand: usize = 8;
        let multiplier: usize = 37;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 13, 2, 0];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_six_digit() {
        // Arrange
        let multiplicand: usize = 123;
        let multiplier: usize = 456;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 8, 10, 15, 4, 0];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_eleven_digits_multiplier_is_greater() {
        // Arrange
        let multiplicand: usize = 78924358;
        let multiplier: usize = 357;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn test_breakdown_addition_of_multiplication_with_eleven_digits_multiplier_is_less() {
        // Arrange
        let multiplicand: usize = 357;
        let multiplier: usize = 78924358;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    // # -----------------------------------------------------------------------
    // # Function: long_sum
    // # -----------------------------------------------------------------------
    #[test]
    fn test_long_sum_with_one_digit() {
        // Arrange
        let multiplicand: usize = 3;
        let multiplier: usize = 2;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   ┃ 2 C\n\
                              ┣━━━┷━━━┫\n\
                              ┃Pro.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃ 0 │ 6 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_long_sum_with_two_digits() {
        // Arrange
        let multiplicand: usize = 9;
        let multiplier: usize = 9;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 1 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃ 8 │   ┃ 2 C\n\
                              ┣━━━┷━━━┫\n\
                              ┃Pro.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃ 8 │ 1 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_long_sum_with_three_digits() {
        // Arrange
        let multiplicand: usize = 37;
        let multiplier: usize = 5;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │ 5 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 1 │   │   ┃ 3 C\n\
                              ┣━━━┷━━━┷━━━┫\n\
                              ┃Pro.       ┃\n\
                              ┣━━━┯━━━┯━━━┫\n\
                              ┃ 1 │ 8 │ 5 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_long_sum_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │ 8 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 1 │ 3 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 2 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   ┃ 4 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Pro.           ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 0 │ 3 │ 3 │ 8 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_long_sum_with_eleven_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 246802468;
        let multiplier: usize = 357;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │   │   │   │   │   │   │   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 2 │ 0 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 9 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 6 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 1 │ 4 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 7 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 2 │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 1 │ 9 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │   │   │   │   │   │   │   │   │   ┃ 10 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │   │   │   │   │   │   │   │   │   │   ┃ 11 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Sub 1.                                         ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 0 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 1 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 7 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 4 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 8 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 1 │ 1 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 7 │   │   │   │   │   │   │   │   │   ┃ 10 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │   │   │   │   │   │   │   │   │   │   ┃ 11 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Pro.                                           ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 0 │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_long_sum_with_eleven_digits_multiplicand_is_less() {
        // Arrange
        let multiplicand: usize = 357;
        let multiplier: usize = 246802468;
        let mut text: String = String::from("");
        let expected: &str = "┃   │   │   │   │   │   │   │   │   │   │   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 2 │ 0 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 9 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 6 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 1 │ 4 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 7 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 2 │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 1 │ 9 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │   │   │   │   │   │   │   │   │   ┃ 10 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │   │   │   │   │   │   │   │   │   │   ┃ 11 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Sub 1.                                         ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 0 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 1 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 7 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 4 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 8 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 1 │ 1 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 7 │   │   │   │   │   │   │   │   │   ┃ 10 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │   │   │   │   │   │   │   │   │   │   ┃ 11 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Pro.                                           ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 0 │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ P\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: author
    // # -----------------------------------------------------------------------
    #[test]
    fn test_author_information() {
        // Arrange
        let mut text: String = String::from("");
        let expected: &str = "\n\
                              ---\n\
                              Author: Israel Roldan\n\
                              E-mail: israel.alberto.rv@gmail.com\n\
                              License: GPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        // Action
        author(&mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: break_down_addition
    // # -----------------------------------------------------------------------
    #[test]
    fn test_break_down_addition_result_two_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![6, 0];
        let expected: Vec<usize> = vec![6, 0];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_two_digits_without_zero() {
        // Arrange
        let value: Vec<usize> = vec![2, 4];
        let expected: Vec<usize> = vec![2, 4];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_three_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![2, 9, 0];
        let expected: Vec<usize> = vec![2, 9, 0];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_three_digits_without_zero() {
        // Arrange
        let value: Vec<usize> = vec![5, 8, 2];
        let expected: Vec<usize> = vec![5, 8, 2];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_four_digits_with_zero() {
        // Arrange
        let value: Vec<usize> = vec![4, 8, 4, 0];
        let expected: Vec<usize> = vec![4, 8, 4, 0];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_four_digits_with_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![4, 11, 6, 0];
        let expected: Vec<usize> = vec![4, 1, 7, 0];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_four_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![6, 12, 6, 2];
        let expected: Vec<usize> = vec![6, 2, 7, 2];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_nine_digits_with_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![1, 10, 19, 27, 27, 27, 26, 17, 8];
        let expected: Vec<usize> = vec![1, 0, 10, 8, 9, 9, 8, 9, 9];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_nine_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![5, 10, 10, 10, 5, 16, 4, 0];
        let expected: Vec<usize> = vec![5, 0, 1, 1, 6, 6, 5, 0];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn test_break_down_addition_result_eleven_digits_without_zero_and_carry() {
        // Arrange
        let value: Vec<usize> = vec![5, 12, 17, 14, 13, 8, 11, 26, 12, 10, 1];
        let expected: Vec<usize> = vec![5, 2, 8, 5, 4, 9, 1, 7, 4, 1, 2];

        // Action
        let result: Vec<usize> = break_down_addition(&value);

        // Assert
        assert_eq!(expected, result);
    }
}
