use crate::breakdown::{break_down_addition, break_down_multiplication, break_down_subtotal};
use crate::length::{get_number_length, get_string_length, get_strings_length};

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
/// let expected: &str = "Symbols\n\
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
/// use long_multiplication_core::generate;
/// generate::symbols(&mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn symbols(text: &mut String) {
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
/// let multiplicand: &str = "2";
/// let multiplier: &str = "5";
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━┓\n";
///
/// use long_multiplication_core::generate;
/// generate::top_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "2";
/// let multiplier: &str = "75";
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━━━━━┓\n";
///
/// use long_multiplication_core::generate;
/// generate::top_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn top_border(multiplicand: &str, multiplier: &str, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);

    text.push('┏');
    for _ in 0..inner_width(length) {
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
/// let multiplicand: &str = "2";
/// let multiplier: &str = "5";
/// let mut text: String = String::from("");
/// let expected: &str = "┗━━━┷━━━┛\n";
///
/// use long_multiplication_core::generate;
/// generate::bottom_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "12";
/// let multiplier: &str = "57";
/// let mut text: String = String::from("");
/// let expected: &str = "┗━━━┷━━━┷━━━┷━━━┛\n";
///
/// use long_multiplication_core::generate;
/// generate::bottom_border(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn bottom_border(multiplicand: &str, multiplier: &str, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);
    rule(text, length, ['┗', '━', '┷', '┛']);
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
/// let multiplicand: &str = "7";
/// let multiplier: &str = "8";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.   ┃\n\
///                       ┠┄┄┄┬┄┄┄┨\n\
///                       ┃ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::position_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "123";
/// let multiplier: &str = "456";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Pos.                   ┃\n\
///                       ┠┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┨\n\
///                       ┃ 6 │ 5 │ 4 │ 3 │ 2 │ 1 ┃\n\
///                       ┣━━━┷━━━┷━━━┷━━━┷━━━┷━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::position_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn position_title(multiplicand: &str, multiplier: &str, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);
    let width: usize = cell_width(length);

    title(text, length, "Pos.");
    rule(text, length, ['┠', '┄', '┬', '┨']);

    // Column numbers, from the highest on the left to 1 on the right.
    text.push('┃');
    for n in 0..length {
        if n > 0 {
            text.push('│');
        }
        push_cell(text, width, &(length - n).to_string());
    }
    text.push('┃');
    text.push('\n');

    rule(text, length, ['┣', '━', '┷', '┫']);
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
/// let multiplicand: &str = "73";
/// let multiplier: &str = "4";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.       ┃\n\
///                       ┣━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::operation_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "123";
/// let multiplier: &str = "45";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Ops.               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::operation_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn operation_title(multiplicand: &str, multiplier: &str, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);

    title(text, length, "Ops.");
    rule(text, length, ['┣', '━', '┯', '┫']);
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
/// let multiplicand: &str = "3";
/// let multiplier: &str = "5";
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 3 ┃\n\
///                       ┃ x │ 5 ┃\n\
///                       ┣━━━┿━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::multiplication(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "12";
/// let multiplier: &str = "345";
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │   │   │ 1 │ 2 ┃\n\
///                       ┃ x │   │ 3 │ 4 │ 5 ┃\n\
///                       ┣━━━┿━━━┿━━━┿━━━┿━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::multiplication(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn multiplication(multiplicand: &str, multiplier: &str, text: &mut String) {
    let multiplicand_len: usize = get_string_length(multiplicand);
    let multiplier_len: usize = get_string_length(multiplier);
    let length: usize = multiplicand_len + multiplier_len;
    let width: usize = cell_width(length);

    // Multiplicand, right-aligned.
    let mut row = Row::new(text, width);
    row.empty(length - multiplicand_len);
    row.chars(multiplicand.chars());
    row.finish("");

    // Multiplier, right-aligned, with the `x` sign in the first cell.
    let mut row = Row::new(text, width);
    row.chars(['x']);
    row.empty(length - multiplier_len - 1);
    row.chars(multiplier.chars());
    row.finish("");

    rule(text, length, ['┣', '━', '┿', '┫']);
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
/// let multiplicand: &str = "9";
/// let multiplier: &str = "3";
/// let mut text: String = String::from("");
/// let expected: &str = "┃ 2 │   ┃ 1 ^\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃   │ 7 ┃ 1 R\n\
///                       ┣━━━┷━━━┫\n";
///
///
/// use long_multiplication_core::generate;
/// generate::operations(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "579";
/// let multiplier: &str = "48";
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
///
/// use long_multiplication_core::generate;
/// generate::operations(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn operations(multiplicand: &str, multiplier: &str, text: &mut String) {
    let multiplicand_len: usize = get_string_length(multiplicand);
    let length: usize = get_strings_length(multiplicand, multiplier);
    let width: usize = cell_width(length);

    let (operation_unit, operation_carry) = break_down_multiplication(multiplicand, multiplier);

    let step: usize = multiplicand_len;
    let max_group_rows: usize = operation_unit.len() / step;
    for (index, start) in (0..operation_unit.len()).step_by(step).enumerate() {
        let iteration: usize = index + 1;
        let end: usize = start + step;

        // Carries of this row, shifted one column to the left.
        let mut row = Row::new(text, width);
        row.empty(length - step - iteration);
        row.digits(&operation_carry[start..end]);
        row.empty(iteration);
        row.finish(&format!(" {iteration} ^"));

        rule(text, length, ['┠', '┈', '┼', '┨']);

        // Units of this row.
        let mut row = Row::new(text, width);
        row.empty(length - step - iteration + 1);
        row.digits(&operation_unit[start..end]);
        row.empty(iteration - 1);
        row.finish(&format!(" {iteration} R"));

        if iteration == max_group_rows {
            break;
        }
        rule(text, length, ['┠', '─', '┼', '┨']);
    }

    rule(text, length, ['┣', '━', '┷', '┫']);
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
/// let multiplicand: &str = "13";
/// let multiplier: &str = "8";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Sum.       ┃\n\
///                       ┣━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::sum_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "951";
/// let multiplier: &str = "46";
/// let mut text: String = String::from("");
/// let expected: &str = "┃Sum.               ┃\n\
///                       ┣━━━┯━━━┯━━━┯━━━┯━━━┫\n";
///
/// use long_multiplication_core::generate;
/// generate::sum_title(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn sum_title(multiplicand: &str, multiplier: &str, text: &mut String) {
    let length: usize = get_strings_length(multiplicand, multiplier);

    title(text, length, "Sum.");
    rule(text, length, ['┣', '━', '┯', '┫']);
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
/// let multiplicand: &str = "3";
/// let multiplier: &str = "2";
/// let mut text: String = String::from("");
/// let expected: &str = "┃   │ 6 ┃ 1 C\n\
///                       ┠┈┈┈┼┈┈┈┨\n\
///                       ┃ 0 │   ┃ 2 C\n\
///                       ┣━━━┷━━━┫\n\
///                       ┃Pro.   ┃\n\
///                       ┣━━━┯━━━┫\n\
///                       ┃ 0 │ 6 ┃ P\n";
///
/// use long_multiplication_core::generate;
/// generate::long_sum(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
///
/// Example #2
/// ```rust
/// let multiplicand: &str = "13";
/// let multiplier: &str = "26";
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
/// use long_multiplication_core::generate;
/// generate::long_sum(multiplicand, multiplier, &mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn long_sum(multiplicand: &str, multiplier: &str, text: &mut String) {
    let additions: Vec<usize> = break_down_addition(multiplicand, multiplier);

    let length: usize = get_strings_length(multiplicand, multiplier);
    let width: usize = cell_width(length);
    generate_rows_with_numbers(&additions, length, text);

    let mut sub_addition: Vec<usize> = break_down_subtotal(&additions);
    let mut sub_index: usize = 0;
    while needs_subtotal(&sub_addition) {
        sub_index += 1;
        rule(text, length, ['┣', '━', '┷', '┫']);
        title(text, length, &format!("Sub {sub_index}."));
        rule(text, length, ['┣', '━', '┯', '┫']);

        generate_rows_with_numbers(&sub_addition, length, text);
        sub_addition = break_down_subtotal(&sub_addition);
    }

    rule(text, length, ['┣', '━', '┷', '┫']);
    title(text, length, "Pro.");
    rule(text, length, ['┣', '━', '┯', '┫']);

    // The product, most significant digit first.
    sub_addition.reverse();
    let mut row = Row::new(text, width);
    row.digits(&sub_addition);
    row.finish(" P");
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
///                       License: AGPL-3.0\n\
///                       Project: https://github.com/airvzxf/long-multiplication-calculator\n";
///
/// use long_multiplication_core::generate;
/// generate::author(&mut text);
///
/// assert_eq!(expected, text);
/// ```
pub fn author(text: &mut String) {
    text.push('\n');
    text.push_str("---\n");
    text.push_str("Author: Israel Roldan\n");
    text.push_str("E-mail: israel.alberto.rv@gmail.com\n");
    text.push_str("License: AGPL-3.0\n");
    text.push_str("Project: https://github.com/airvzxf/long-multiplication-calculator\n");
}

fn generate_rows_with_numbers(numbers: &[usize], length: usize, text: &mut String) {
    let width: usize = cell_width(length);

    for (column, number) in numbers.iter().enumerate() {
        // The sum of column `column` is written so that its units
        // digit lands in that column; its tens spill to the left.
        let digits: String = number.to_string();
        let mut row = Row::new(text, width);
        row.empty(length - column - digits.len());
        row.chars(digits.chars());
        row.empty(column);
        row.finish(&format!(" {} C", column + 1));

        if column + 1 == length {
            break;
        }
        rule(text, length, ['┠', '┈', '┼', '┨']);
    }
}

/// Width of every cell, in characters.
///
/// Three characters fit every column number up to 999. Wider tables
/// get wider cells so that the `Pos.` row stays aligned; tables of up
/// to 999 columns are unchanged.
pub(crate) fn cell_width(length: usize) -> usize {
    get_number_length(length).max(3)
}

/// Number of characters between the left and right outer borders.
pub(crate) fn inner_width(length: usize) -> usize {
    length * cell_width(length) + length - 1
}

/// Whether a list of column sums still holds a value of two or more
/// digits, so that one more `Sub n.` section is needed.
pub(crate) fn needs_subtotal(numbers: &[usize]) -> bool {
    numbers.iter().any(|&number| number > 9)
}

/// Push a horizontal rule: `[left, fill, separator, right]`.
fn rule(text: &mut String, length: usize, [left, fill, separator, right]: [char; 4]) {
    let width: usize = cell_width(length);
    text.push(left);
    for n in 0..length {
        if n > 0 {
            text.push(separator);
        }
        for _ in 0..width {
            text.push(fill);
        }
    }
    text.push(right);
    text.push('\n');
}

/// Push a section title such as `┃Pos.   ┃`.
fn title(text: &mut String, length: usize, label: &str) {
    text.push('┃');
    text.push_str(label);
    for _ in label.chars().count()..inner_width(length) {
        text.push(' ');
    }
    text.push('┃');
    text.push('\n');
}

/// Push the content of one cell, padded to `width` characters.
///
/// A short value keeps one space on each side (` 7 `); a value that
/// is one character narrower than the cell is right-aligned (` 42`);
/// a value as wide as the cell fills it (`100`).
fn push_cell(text: &mut String, width: usize, content: &str) {
    let len: usize = content.chars().count();
    let right: usize = usize::from(len + 2 <= width);
    let left: usize = width.saturating_sub(len + right);
    for _ in 0..left {
        text.push(' ');
    }
    text.push_str(content);
    for _ in 0..right {
        text.push(' ');
    }
}

/// Builder for one table row: cells joined by `│`, framed by `┃`.
struct Row<'a> {
    text: &'a mut String,
    width: usize,
    first: bool,
}

impl<'a> Row<'a> {
    fn new(text: &'a mut String, width: usize) -> Self {
        text.push('┃');
        Self { text, width, first: true }
    }

    fn separator(&mut self) {
        if !self.first {
            self.text.push('│');
        }
        self.first = false;
    }

    fn empty(&mut self, count: usize) {
        for _ in 0..count {
            self.separator();
            for _ in 0..self.width {
                self.text.push(' ');
            }
        }
    }

    fn chars(&mut self, chars: impl IntoIterator<Item = char>) {
        let mut buffer = [0u8; 4];
        for ch in chars {
            self.separator();
            push_cell(self.text, self.width, ch.encode_utf8(&mut buffer));
        }
    }

    fn digits(&mut self, digits: &[usize]) {
        self.chars(digits.iter().map(|&digit| {
            char::from_digit(u32::try_from(digit % 10).unwrap_or(0), 10).unwrap_or('?')
        }));
    }

    /// Close the row and append `suffix` (for example ` 1 ^`).
    fn finish(self, suffix: &str) {
        self.text.push('┃');
        self.text.push_str(suffix);
        self.text.push('\n');
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
        let expected: &str = "Symbols\n\
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
        let multiplicand: &str = "2";
        let multiplier: &str = "4";
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
        let multiplicand: &str = "12";
        let multiplier: &str = "3";
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
        let multiplicand: &str = "345";
        let multiplier: &str = "12";
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
        let multiplicand: &str = "123456";
        let multiplier: &str = "123456";
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
        let multiplicand: &str = "7";
        let multiplier: &str = "3";
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
        let multiplicand: &str = "8";
        let multiplier: &str = "43";
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
        let multiplicand: &str = "519";
        let multiplier: &str = "43";
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
        let multiplicand: &str = "12";
        let multiplier: &str = "1234567890";
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
        let multiplicand: &str = "6";
        let multiplier: &str = "3";
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
        let multiplicand: &str = "18";
        let multiplier: &str = "6";
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
        let multiplicand: &str = "78";
        let multiplier: &str = "327";
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
        let multiplicand: &str = "123456";
        let multiplier: &str = "54321";
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
        let multiplicand: &str = "9";
        let multiplier: &str = "1";
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
        let multiplicand: &str = "53";
        let multiplier: &str = "4";
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
        let multiplicand: &str = "53";
        let multiplier: &str = "618";
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
        let multiplicand: &str = "654321";
        let multiplier: &str = "12345";
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
        let multiplicand: &str = "8";
        let multiplier: &str = "4";
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
        let multiplicand: &str = "2";
        let multiplier: &str = "37";
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
        let multiplicand: &str = "81";
        let multiplier: &str = "925";
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
        let multiplicand: &str = "12345";
        let multiplier: &str = "654321";
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
        let multiplicand: &str = "1234";
        let multiplier: &str = "5";
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
        let multiplicand: &str = "8765";
        let multiplier: &str = "1234";
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
        let multiplicand: &str = "25";
        let multiplier: &str = "3";
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
        let multiplicand: &str = "3";
        let multiplier: &str = "25";
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
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
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
        let multiplicand: &str = "246802468";
        let multiplier: &str = "357";
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
        let multiplicand: &str = "357";
        let multiplier: &str = "246802468";
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
        let multiplicand: &str = "7";
        let multiplier: &str = "9876543210123";
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
        let multiplicand: &str = "4";
        let multiplier: &str = "2";
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
        let multiplicand: &str = "19";
        let multiplier: &str = "5";
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
        let multiplicand: &str = "73";
        let multiplier: &str = "438";
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
        let multiplicand: &str = "123456";
        let multiplier: &str = "54321";
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
        let multiplicand: &str = "3";
        let multiplier: &str = "2";
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
        let multiplicand: &str = "9";
        let multiplier: &str = "9";
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
        let multiplicand: &str = "37";
        let multiplier: &str = "5";
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
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
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
        let multiplicand: &str = "246802468";
        let multiplier: &str = "357";
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
        let multiplicand: &str = "357";
        let multiplier: &str = "246802468";
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
                              License: AGPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        // Action
        author(&mut text);

        // Assert
        assert_eq!(expected, text);
    }
}
