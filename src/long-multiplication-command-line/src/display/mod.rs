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
///                       Pro. = Product of the multiplication.\n\
///                       ^ = Carry-over.\n\
///                       n R = The row number.\n\
///                       n C = The column number of the sum of the rows.\n\
///                       * Replace 'n' for a number.\n\
///                       P = The product of multiplication.\n\
///                       V = Validate the product of multiplication.\n\
///                       \n";
/// let mut text: String = String::from("");
///
/// use long_multiplication_command_line::display;
/// display::symbols(&mut text);
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
    text.push_str("Pro. = Product of the multiplication.\n");
    text.push_str("^ = Carry-over.\n");
    text.push_str("n R = The row number.\n");
    text.push_str("n C = The column number of the sum of the rows.\n");
    text.push_str("* Replace 'n' for a number.\n");
    text.push_str("P = The product of multiplication.\n");
    text.push_str("V = Validate the product of multiplication.\n");
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
/// use long_multiplication_command_line::display;
/// display::top_border(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::top_border(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::bottom_border(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::bottom_border(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::position_title(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::position_title(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::operation_title(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::operation_title(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::multiplication(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::multiplication(multiplicand, multiplier, &mut text);
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
/// let expected: &str = "┃ 2 │   ┃ ^\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 7 ┃ 1 R\n\
///                       ┣━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::display;
/// display::operations(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 579;
/// let multiplier: usize = 48;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 4 │ 5 │ 7 │   ┃ ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │ 0 │ 6 │ 2 ┃ 1 R\n\
///                       ┠───┼───┼───┼───┼───┨\n\
///                       ┃ 2 │ 2 │ 3 │   │   ┃ ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 0 │ 8 │ 6 │   ┃ 2 R\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::display;
/// display::operations(multiplicand, multiplier, &mut text);
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
        text.push_str("┃ ^\n");

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
        text.push_str(" R");
        text.push('\n');

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
/// use long_multiplication_command_line::display;
/// display::sum_title(multiplicand, multiplier, &mut text);
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
/// use long_multiplication_command_line::display;
/// display::sum_title(multiplicand, multiplier, &mut text);
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
///                       ┃ 0 │ 6 ┃ P\n\
///                       ┠───┼───┨\n";
///
/// use long_multiplication_command_line::display;
/// display::long_sum(multiplicand, multiplier, &mut text);
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
///                       ┃ 0 │ 3 │ 3 │ 8 ┃ P\n\
///                       ┠───┼───┼───┼───┨\n";
///
/// use long_multiplication_command_line::display;
/// display::long_sum(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn long_sum(multiplicand: usize, multiplier: usize, text: &mut String) {
    let mut additions: Vec<usize> = break_down_addition_of_multiplication(multiplicand, multiplier);
    additions.reverse();

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
    let mut sum: usize = 0;
    let mut iteration: u32 = 0;
    for row in &additions {
        let expo = 10usize.pow(iteration);
        sum += row * expo;
        iteration += 1;
    }

    let sum_size: usize = get_number_length(sum);
    text.push('┃');
    for _ in 0..(length - sum_size) {
        text.push_str(" 0 ");
        text.push('│');
    }

    for i in sum.to_string().chars() {
        text.push(' ');
        text.push(i);
        text.push_str(" │");
    }
    text.pop();

    text.push_str("┃ P");
    text.push('\n');

    // Create second row for product
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
}

/// Store the product-validation section of the long multiplication.
///
/// It generates the table product-validation-section for the
/// long multiplication and stores it in a text variable.
///
/// It does the math operation for the multiplication and shows
/// the verification product.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: usize = 3;
/// let multiplier: usize = 2;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 6 ┃ V\n";
///
/// use long_multiplication_command_line::display;
/// display::product_validation(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: usize = 13;
/// let multiplier: usize = 26;
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 3 │ 3 │ 8 ┃ V\n";
///
/// use long_multiplication_command_line::display;
/// display::product_validation(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn product_validation(multiplicand: usize, multiplier: usize, text: &mut String) {
    let length: usize = get_numbers_length(multiplicand, multiplier);
    let product: usize = multiplicand * multiplier;
    let product_size: usize = get_number_length(product);

    // Create first row for product
    text.push('┃');
    for _ in 0..(length - product_size) {
        text.push_str("   ");
        text.push('│');
    }

    for i in product.to_string().chars() {
        text.push(' ');
        text.push(i);
        text.push_str(" │");
    }
    text.pop();

    text.push_str("┃ V");
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
/// use long_multiplication_command_line::display;
/// display::author(&mut text);
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

    addition.reverse();
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

#[cfg(test)]
mod tests {
    use super::*;

    // # -----------------------------------------------------------------------
    // # Function: get number length
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
    // # Function: get numbers length
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
                              Pro. = Product of the multiplication.\n\
                              ^ = Carry-over.\n\
                              n R = The row number.\n\
                              n C = The column number of the sum of the rows.\n\
                              * Replace 'n' for a number.\n\
                              P = The product of multiplication.\n\
                              V = Validate the product of multiplication.\n\
                              \n";

        // Action
        symbols(&mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: top border
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
    // # Function: bottom border
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
    // # Function: operation title
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
    // # Function: operation title
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
    // # Function: multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_operations_with_three_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 25;
        let multiplier: usize = 3;
        let mut text: String = String::from("");
        let expected: &str = "┃ 0 │ 1 │   ┃ ^\n\
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
        let expected: &str = "┃   │ 1 │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 5 ┃ 1 R\n\
                              ┠───┼───┼───┨\n\
                              ┃ 0 │   │   ┃ ^\n\
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
        let expected: &str = "┃   │ 0 │ 1 │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │ 8 ┃ 1 R\n\
                              ┠───┼───┼───┼───┨\n\
                              ┃ 0 │ 0 │   │   ┃ ^\n\
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
        let expected: &str = "┃   │   │ 1 │ 2 │ 4 │ 5 │ 0 │ 1 │ 2 │ 4 │ 5 │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 4 │ 8 │ 2 │ 6 │ 0 │ 4 │ 8 │ 2 │ 6 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 1 │ 2 │ 3 │ 4 │ 0 │ 1 │ 2 │ 3 │ 4 │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 0 │ 1 │ 1 │ 2 │ 0 │ 0 │ 1 │ 1 │ 2 │   │   │   ┃ ^\n\
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
        let expected: &str = "┃   │   │   │   │   │   │   │   │ 2 │ 4 │ 5 │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 4 │ 0 │ 6 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │ 3 │ 4 │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 8 │ 0 │ 2 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │ 1 │ 2 │ 2 │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 2 │ 0 │ 8 │   │   ┃ 3 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │ 0 │ 1 │ 1 │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 6 │ 0 │ 4 │   │   │   ┃ 4 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │ 0 │ 0 │ 0 │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 0 │ 0 │ 0 │   │   │   │   ┃ 5 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │ 2 │ 4 │ 5 │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 4 │ 0 │ 6 │   │   │   │   │   ┃ 6 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │ 1 │ 3 │ 4 │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 8 │ 0 │ 2 │   │   │   │   │   │   ┃ 7 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 1 │ 2 │ 2 │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 2 │ 0 │ 8 │   │   │   │   │   │   │   ┃ 8 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 0 │ 1 │ 1 │   │   │   │   │   │   │   │   │   ┃ ^\n\
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
        let expected: &str = "┃   │   │   │   │   │   │   │   │   │   │   │   │ 2 │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │   │   │ 1 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 1 │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │   │ 4 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 0 │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │   │ 7 │   │   ┃ 3 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 0 │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │   │ 0 │   │   │   ┃ 4 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │   │ 0 │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │   │ 7 │   │   │   │   ┃ 5 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │   │ 4 │   │   │   │   │   ┃ 6 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │   │ 2 │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 1 │   │   │   │   │   │   ┃ 7 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │   │ 2 │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 8 │   │   │   │   │   │   │   ┃ 8 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │   │ 3 │   │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 5 │   │   │   │   │   │   │   │   ┃ 9 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │   │ 4 │   │   │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 2 │   │   │   │   │   │   │   │   │   ┃ 10 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │ 4 │   │   │   │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 9 │   │   │   │   │   │   │   │   │   │   ┃ 11 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 5 │   │   │   │   │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │   │   │   │   │   │   │   │   │   │   │   ┃ 12 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 6 │   │   │   │   │   │   │   │   │   │   │   │   │   ┃ ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 3 │   │   │   │   │   │   │   │   │   │   │   │   ┃ 13 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        operations(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: breakdown the multiplication
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
    // # Function: sum title
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
    // # Function: breakdown the addition of the multiplication
    // # -----------------------------------------------------------------------
    #[test]
    fn test_breakdown_addition_of_multiplication_product_one_digit() {
        // Arrange
        let multiplicand: usize = 2;
        let multiplier: usize = 3;
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![0, 6];

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
        let expected_addition: Vec<usize> = vec![7, 2];

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
        let expected_addition: Vec<usize> = vec![2, 9, 6];

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
        let expected_addition: Vec<usize> = vec![2, 9, 6];

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
        let expected_addition: Vec<usize> = vec![0, 2, 13, 8];

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
        let expected_addition: Vec<usize> = vec![0, 4, 15, 10, 8, 8];

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
        let expected_addition: Vec<usize> = vec![2, 6, 19, 25, 25, 8, 17, 24, 17, 10, 6];

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
        let expected_addition: Vec<usize> = vec![2, 6, 19, 25, 25, 8, 17, 24, 17, 10, 6];

        // Action
        addition = break_down_addition_of_multiplication(multiplicand, multiplier);

        // Assert
        assert_eq!(expected_addition, addition);
    }

    // # -----------------------------------------------------------------------
    // # Function: long sum
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
                              ┃ 0 │ 6 ┃ P\n\
                              ┠───┼───┨\n";

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
                              ┃ 8 │ 1 ┃ P\n\
                              ┠───┼───┨\n";

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
                              ┃ 1 │ 8 │ 5 ┃ P\n\
                              ┠───┼───┼───┨\n";

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
                              ┃ 0 │ 3 │ 3 │ 8 ┃ P\n\
                              ┠───┼───┼───┼───┨\n";

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
                              ┃Pro.                                           ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 0 │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ P\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n";

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
                              ┃Pro.                                           ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 0 │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ P\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┨\n";

        // Action
        long_sum(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: product validation
    // # -----------------------------------------------------------------------
    #[test]
    fn test_product_validation_with_one_digit() {
        // Arrange
        let multiplicand: usize = 3;
        let multiplier: usize = 2;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 6 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_product_validation_with_two_digits() {
        // Arrange
        let multiplicand: usize = 9;
        let multiplier: usize = 9;
        let mut text: String = String::from("");
        let expected: &str = "┃ 8 │ 1 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_product_validation_with_three_digits() {
        // Arrange
        let multiplicand: usize = 37;
        let multiplier: usize = 5;
        let mut text: String = String::from("");
        let expected: &str = "┃ 1 │ 8 │ 5 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_product_validation_with_four_digit() {
        // Arrange
        let multiplicand: usize = 13;
        let multiplier: usize = 26;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 3 │ 3 │ 8 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_product_validation_with_eleven_digits_multiplicand_is_greater() {
        // Arrange
        let multiplicand: usize = 246802468;
        let multiplier: usize = 357;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_product_validation_with_eleven_digits_multiplicand_is_less() {
        // Arrange
        let multiplicand: usize = 357;
        let multiplier: usize = 246802468;
        let mut text: String = String::from("");
        let expected: &str = "┃   │ 8 │ 8 │ 1 │ 0 │ 8 │ 4 │ 8 │ 1 │ 0 │ 7 │ 6 ┃ V\n";

        // Action
        product_validation(multiplicand, multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: symbols
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
}
