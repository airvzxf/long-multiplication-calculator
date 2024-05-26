use std::fs::File;
use std::io::Write;

use crate::generate;

/// Return the table of the long multiplication.
///
/// It generates the complete table for the
/// long multiplication and returns it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let multiplicand: String = String::from("5");
/// let multiplier: String = String::from("7");
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
///                       \n\
///                       ┏━━━━━━━┓\n\
///                       ┃Pos.   ┃\n\
///                       ┠┄┄┄┬┄┄┄┨\n\
///                       ┃ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┫\n\
///                       ┃Ops.   ┃\n\
///                       ┣━━━┯━━━┫\n\
///                       ┃   │ 5 ┃\n\
///                       ┃ x │ 7 ┃\n\
///                       ┣━━━┿━━━┫\n\
///                       ┃ 3 │   ┃ 1 ^\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 5 ┃ 1 R\n\
///                       ┣━━━┷━━━┫\n\
///                       ┃Sum.   ┃\n\
///                       ┣━━━┯━━━┫\n\
///                       ┃   │ 5 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃ 3 │   ┃ 2 C\n\
///                       ┣━━━┷━━━┫\n\
///                       ┃Pro.   ┃\n\
///                       ┣━━━┯━━━┫\n\
///                       ┃ 3 │ 5 ┃ P\n\
///                       ┗━━━┷━━━┛\n\
///                       \n\
///                       ---\n\
///                       Author: Israel Roldan\n\
///                       E-mail: israel.alberto.rv@gmail.com\n\
///                       License: GPL-3.0\n\
///                       Project: https://github.com/airvzxf/long-multiplication-calculator\n";
///
/// use long_multiplication_command_line::multiplication::get_table;
/// let text: String = get_table(&multiplicand, &multiplier);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: String = String::from("13597");
/// let multiplier: String = String::from("8642");
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
///                       \n\
///                       ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n\
///                       ┃Pos.                               ┃\n\
///                       ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
///                       ┃ 9 │ 8 │ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
///                       ┃Ops.                               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
///                       ┃   │   │   │   │ 1 │ 3 │ 5 │ 9 │ 7 ┃\n\
///                       ┃ x │   │   │   │   │ 8 │ 6 │ 4 │ 2 ┃\n\
///                       ┣━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┫\n\
///                       ┃   │   │   │ 0 │ 0 │ 1 │ 1 │ 1 │   ┃ 1 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │ 2 │ 6 │ 0 │ 8 │ 4 ┃ 1 R\n\
///                       ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
///                       ┃   │   │ 0 │ 1 │ 2 │ 3 │ 2 │   │   ┃ 2 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │ 4 │ 2 │ 0 │ 6 │ 8 │   ┃ 2 R\n\
///                       ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
///                       ┃   │ 0 │ 1 │ 3 │ 5 │ 4 │   │   │   ┃ 3 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │ 6 │ 8 │ 0 │ 4 │ 2 │   │   ┃ 3 R\n\
///                       ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
///                       ┃ 0 │ 2 │ 4 │ 7 │ 5 │   │   │   │   ┃ 4 ^\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 8 │ 4 │ 0 │ 2 │ 6 │   │   │   ┃ 4 R\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
///                       ┃Sum.                               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
///                       ┃   │   │   │   │   │   │   │   │ 4 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │   │   │ 1 │ 7 │   ┃ 2 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │   │ 1 │ 1 │   │   ┃ 3 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │ 2 │ 4 │   │   │   ┃ 4 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │ 1 │ 8 │   │   │   │   ┃ 5 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │ 2 │ 3 │   │   │   │   │   ┃ 6 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 1 │ 5 │   │   │   │   │   │   ┃ 7 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃ 1 │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃ 0 │   │   │   │   │   │   │   │   ┃ 9 C\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
///                       ┃Sub 1.                             ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
///                       ┃   │   │   │   │   │   │   │   │ 4 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │   │   │ 2 │   │   ┃ 3 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │   │   │ 5 │   │   │   ┃ 4 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │ 1 │ 0 │   │   │   │   ┃ 5 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │   │ 4 │   │   │   │   │   ┃ 6 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │   │ 7 │   │   │   │   │   │   ┃ 7 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 1 │   │   │   │   │   │   │   ┃ 8 C\n\
///                       ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
///                       ┃ 1 │   │   │   │   │   │   │   │   ┃ 9 C\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
///                       ┃Pro.                               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
///                       ┃ 1 │ 1 │ 7 │ 5 │ 0 │ 5 │ 2 │ 7 │ 4 ┃ P\n\
///                       ┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n\
///                       \n\
///                       ---\n\
///                       Author: Israel Roldan\n\
///                       E-mail: israel.alberto.rv@gmail.com\n\
///                       License: GPL-3.0\n\
///                       Project: https://github.com/airvzxf/long-multiplication-calculator\n";
///
/// use long_multiplication_command_line::multiplication::get_table;
/// let text: String = get_table(&multiplicand, &multiplier);
///
/// assert_eq!(expected, text);
/// ```
pub fn get_table(multiplicand: &String, multiplier: &String) -> String {
    let mut content: String = String::from("");

    generate::symbols(&mut content);
    generate::top_border(&multiplicand, &multiplier, &mut content);
    generate::position_title(&multiplicand, &multiplier, &mut content);
    generate::operation_title(&multiplicand, &multiplier, &mut content);
    generate::multiplication(&multiplicand, &multiplier, &mut content);
    generate::operations(&multiplicand, &multiplier, &mut content);
    generate::sum_title(&multiplicand, &multiplier, &mut content);
    generate::long_sum(&multiplicand, &multiplier, &mut content);
    generate::bottom_border(&multiplicand, &multiplier, &mut content);
    generate::author(&mut content);

    let content: String = content;
    return content;
}

/// Display the table of the long multiplication.
///
/// It displays the complete table for the
/// long multiplication and returns it in a text variable.
///
/// Examples
/// --------
///
/// Example #1
/// ```rust
/// let content: String = String::from("This is a text for test.");
///
/// use long_multiplication_command_line::multiplication::display;
/// display(&content);
/// ```
pub fn display(content: &String) {
    println!("{content}");
}

/// Store the table of the long multiplication.
///
/// It stores the complete table for the
/// long multiplication as a file in your local machine.
///
/// Examples
/// --------
///
/// Example #1
/// ```text
/// let content: String = String::from("This text will be stored.");
/// let file_path: String = String::from("/home/USER_NAME/test-store-doc-01.txt");
///
/// use long_multiplication_command_line::multiplication::store;
/// store(&content, &file_path);
/// ```
pub fn store(content: &String, file_path: &String) {
    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes())
        }
        Err(_err) => panic!("ERROR: the file '{file_path}' cannot be created.\nDetails: {_err:?}"),
    }.expect("ERROR: trying to write the content in the file.");
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    // # -----------------------------------------------------------------------
    // # Function: get table
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_table_product_one_digits() {
        // Arrange
        let multiplicand: String = String::from("3");
        let multiplier: String = String::from("2");
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
                              \n\
                              ┏━━━━━━━┓\n\
                              ┃Pos.   ┃\n\
                              ┠┄┄┄┬┄┄┄┨\n\
                              ┃ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┫\n\
                              ┃Ops.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃   │ 3 ┃\n\
                              ┃ x │ 2 ┃\n\
                              ┣━━━┿━━━┫\n\
                              ┃ 0 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃   │ 6 ┃ 1 R\n\
                              ┣━━━┷━━━┫\n\
                              ┃Sum.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃   │ 6 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   ┃ 2 C\n\
                              ┣━━━┷━━━┫\n\
                              ┃Pro.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃ 0 │ 6 ┃ P\n\
                              ┗━━━┷━━━┛\n\
                              \n\
                              ---\n\
                              Author: Israel Roldan\n\
                              E-mail: israel.alberto.rv@gmail.com\n\
                              License: GPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        // Action
        let text: String = get_table(&multiplicand, &multiplier);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_get_table_product_two_digits() {
        // Arrange
        let multiplicand: String = String::from("5");
        let multiplier: String = String::from("7");
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
                              \n\
                              ┏━━━━━━━┓\n\
                              ┃Pos.   ┃\n\
                              ┠┄┄┄┬┄┄┄┨\n\
                              ┃ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┫\n\
                              ┃Ops.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃   │ 5 ┃\n\
                              ┃ x │ 7 ┃\n\
                              ┣━━━┿━━━┫\n\
                              ┃ 3 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃   │ 5 ┃ 1 R\n\
                              ┣━━━┷━━━┫\n\
                              ┃Sum.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃   │ 5 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┨\n\
                              ┃ 3 │   ┃ 2 C\n\
                              ┣━━━┷━━━┫\n\
                              ┃Pro.   ┃\n\
                              ┣━━━┯━━━┫\n\
                              ┃ 3 │ 5 ┃ P\n\
                              ┗━━━┷━━━┛\n\
                              \n\
                              ---\n\
                              Author: Israel Roldan\n\
                              E-mail: israel.alberto.rv@gmail.com\n\
                              License: GPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        // Action
        let text: String = get_table(&multiplicand, &multiplier);

        // Assert
        assert_eq!(expected, text);
    }

    #[test]
    fn test_get_table_product_nine_digits() {
        // Arrange
        let multiplicand: String = String::from("13597");
        let multiplier: String = String::from("8642");
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
                              \n\
                              ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n\
                              ┃Pos.                               ┃\n\
                              ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
                              ┃ 9 │ 8 │ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Ops.                               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃   │   │   │   │ 1 │ 3 │ 5 │ 9 │ 7 ┃\n\
                              ┃ x │   │   │   │   │ 8 │ 6 │ 4 │ 2 ┃\n\
                              ┣━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┿━━━┫\n\
                              ┃   │   │   │ 0 │ 0 │ 1 │ 1 │ 1 │   ┃ 1 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 2 │ 6 │ 0 │ 8 │ 4 ┃ 1 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │   │ 0 │ 1 │ 2 │ 3 │ 2 │   │   ┃ 2 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 4 │ 2 │ 0 │ 6 │ 8 │   ┃ 2 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃   │ 0 │ 1 │ 3 │ 5 │ 4 │   │   │   ┃ 3 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 6 │ 8 │ 0 │ 4 │ 2 │   │   ┃ 3 R\n\
                              ┠───┼───┼───┼───┼───┼───┼───┼───┼───┨\n\
                              ┃ 0 │ 2 │ 4 │ 7 │ 5 │   │   │   │   ┃ 4 ^\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 8 │ 4 │ 0 │ 2 │ 6 │   │   │   ┃ 4 R\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Sum.                               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃   │   │   │   │   │   │   │   │ 4 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 1 │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 1 │ 1 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │ 2 │ 4 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 1 │ 8 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 2 │ 3 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 1 │ 5 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 1 │ 0 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 0 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Sub 1.                             ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃   │   │   │   │   │   │   │   │ 4 ┃ 1 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │   │ 7 │   ┃ 2 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │   │ 2 │   │   ┃ 3 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │   │   │ 5 │   │   │   ┃ 4 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 1 │ 0 │   │   │   │   ┃ 5 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │   │ 4 │   │   │   │   │   ┃ 6 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │   │ 7 │   │   │   │   │   │   ┃ 7 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃   │ 1 │   │   │   │   │   │   │   ┃ 8 C\n\
                              ┠┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┼┈┈┈┨\n\
                              ┃ 1 │   │   │   │   │   │   │   │   ┃ 9 C\n\
                              ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n\
                              ┃Pro.                               ┃\n\
                              ┣━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┫\n\
                              ┃ 1 │ 1 │ 7 │ 5 │ 0 │ 5 │ 2 │ 7 │ 4 ┃ P\n\
                              ┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n\
                              \n\
                              ---\n\
                              Author: Israel Roldan\n\
                              E-mail: israel.alberto.rv@gmail.com\n\
                              License: GPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        // Action
        let text: String = get_table(&multiplicand, &multiplier);

        // Assert
        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: store
    // # -----------------------------------------------------------------------
    #[test]
    fn test_store_successful() {
        // Arrange
        let expected: String = String::from("This is a text for the content.");
        let file_path: String = String::from("/tmp/test-storage-01.txt");
        let mut file: File;
        let mut content: String = String::new();

        // Action
        store(&expected, &file_path);

        // Assert
        file = File::open(file_path).expect("Unable to open the file.");
        file.read_to_string(&mut content).expect("Unable to read the file.");
        assert_eq!(expected, content);
    }

    #[test]
    #[should_panic(expected = "ERROR: the file \
    '/tmp/USER_NAME/test-storage-02.txt' cannot be created.\n\
    Details: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }")]
    fn test_store_panic_file() {
        // Arrange
        let expected: String = String::from("This is a text for the content.");
        let file_path: String = String::from("/tmp/USER_NAME/test-storage-02.txt");

        // Action
        store(&expected, &file_path);
    }

    // #[test]
    // TODO: Find a way to test the error when write the content.
    // fn test_store_panic_write_content() {
    // }
}
