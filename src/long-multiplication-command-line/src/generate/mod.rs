use crate::breakdown::{break_down_addition, break_down_multiplication, break_down_subtotal};
use crate::length::{get_number_length, get_numbers_length, get_strings_length};

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
/// let multiplicand: String = String::from("2");
/// let multiplier: String = String::from("5");
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━┓\n";
///
/// use long_multiplication_command_line::generate;
/// generate::top_border(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: String = String::from("2");
/// let multiplier: String = String::from("75");
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━━━━━┓\n";
///
/// use long_multiplication_command_line::generate;
/// generate::top_border(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn top_border(multiplicand: &String, multiplier: &String, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);

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
/// let multiplicand: String = String::from("7");
/// let multiplier: String = String::from("8");
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.   ┃\n\
///                       ┠┄┄┄┬┄┄┄┨\n\
///                       ┃ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::position_title(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: String = String::from("123");
/// let multiplier: String = String::from("456");
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.                   ┃\n\
///                       ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
///                       ┃ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::position_title(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn position_title(multiplicand: &String, multiplier: &String, text: &mut String) {
    let length: usize = get_strings_length(&multiplicand, &multiplier);

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
        let number: usize = length + 1 - n;
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
/// let multiplicand: String = String::from("73");
/// let multiplier: String = String::from("4");
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.       ┃\n\
///                       ┣━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operation_title(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: String = String::from("123");
/// let multiplier: String = String::from("45");
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_command_line::generate;
/// generate::operation_title(&multiplicand, &multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn operation_title(multiplicand: &String, multiplier: &String, text: &mut String) {
    let length: usize = get_strings_length(&multiplicand, &multiplier);

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
    let max_group_rows: usize = operation_unit.len() / step;
    let mut iteration: usize = 1;
    for start in (0..operation_unit.len()).step_by(step) {
        let start: usize = start;
        let end: usize = start + step;
        let slice: &[usize] = &operation_carry[start..end];

        // Create first row
        text.push('┃');
        let start_spaces: usize = length - step - iteration;
        for _ in 0..start_spaces {
            text.push_str("   │");
        }
        for n in slice {
            text.push(' ');
            text.push_str(&*n.to_string());
            text.push(' ');
            text.push('│');
        }
        let end_spaces: usize = iteration;
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

        // Create third row
        let slice: &[usize] = &operation_unit[start..end];
        let start_spaces: usize = length - step - iteration + 1;
        text.push('┃');
        for _ in 0..start_spaces {
            text.push_str("   │");
        }
        for n in slice {
            text.push(' ');
            text.push_str(&*n.to_string());
            text.push(' ');
            text.push('│');
        }
        let end_spaces: usize = iteration - 1;
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

    // Create the final row
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
    let additions: Vec<usize> = break_down_addition(multiplicand, multiplier);

    let length: usize = get_numbers_length(multiplicand, multiplier);
    generate_rows_with_numbers(&additions, length, text);

    let mut sub_addition: Vec<usize> = break_down_subtotal(&additions);
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
        generate_rows_with_numbers(&sub_addition, length, text);
        sub_addition = break_down_subtotal(&sub_addition);
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

fn generate_rows_with_numbers(numbers: &Vec<usize>, length: usize, text: &mut String) {
    let mut iteration: usize = 0;

    for row in numbers {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let multiplicand: String = String::from("2");
        let multiplier: String = String::from("4");
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━┓\n";

        // Action
        top_border(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_three_digits() {
        // Arrange
        let multiplicand: String = String::from("12");
        let multiplier: String = String::from("3");
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━┓\n";

        // Action
        top_border(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_five_digits() {
        // Arrange
        let multiplicand: String = String::from("345");
        let multiplier: String = String::from("12");
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━━━━━━━━━┓\n";

        // Action
        top_border(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_top_border_size_twelve_digits() {
        // Arrange
        let multiplicand: String = String::from("123456");
        let multiplier: String = String::from("123456");
        let mut text: String = String::from("");
        let expected: &str = "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n";

        // Action
        top_border(&multiplicand, &multiplier, &mut text);

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
        let multiplicand: String = String::from("6");
        let multiplier: String = String::from("3");
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.   ┃\n\
                              ┠┄┄┄┬┄┄┄┨\n\
                              ┃ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┫\n";

        // Action
        position_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_three_digits() {
        // Arrange
        let multiplicand: String = String::from("18");
        let multiplier: String = String::from("6");
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.       ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┫\n";

        // Action
        position_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_five_digits() {
        // Arrange
        let multiplicand: String = String::from("78");
        let multiplier: String = String::from("327");
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.               ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        position_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_position_title_size_eleven_digits() {
        // Arrange
        let multiplicand: String = String::from("123456");
        let multiplier: String = String::from("54321");
        let mut text: String = String::from("");
        let expected: &str = "┃Pos.                                       ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 11│ 10│ 9 │ 8 │ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";

        // Action
        position_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: operation_title
    // # -----------------------------------------------------------------------
    #[test]
    fn test_operation_title_size_two_digits() {
        // Arrange
        let multiplicand: String = String::from("9");
        let multiplier: String = String::from("1");
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.   ┃\n\
                              ┣━━━┯━━━┫\n";

        // Action
        operation_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_three_digits() {
        // Arrange
        let multiplicand: String = String::from("53");
        let multiplier: String = String::from("4");
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.       ┃\n\
                              ┣━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_five_digits() {
        // Arrange
        let multiplicand: String = String::from("53");
        let multiplier: String = String::from("618");
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(&multiplicand, &multiplier, &mut text);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_operation_title_size_eleven_digits() {
        // Arrange
        let multiplicand: String = String::from("654321");
        let multiplier: String = String::from("12345");
        let mut text: String = String::from("");
        let expected: &str = "┃Ops.                                       ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n";

        // Action
        operation_title(&multiplicand, &multiplier, &mut text);

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
}
