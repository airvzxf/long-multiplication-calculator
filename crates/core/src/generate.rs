use core::fmt;

use crate::LongMultiplication;
use crate::length::get_number_length;

/// The legend printed before the table.
pub(crate) const SYMBOLS: &str = "Symbols\n\
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

/// The footer printed after the table.
pub(crate) const AUTHOR: &str = "\n\
                                 ---\n\
                                 Author: Israel Roldan\n\
                                 E-mail: israel.alberto.rv@gmail.com\n\
                                 License: AGPL-3.0\n\
                                 Project: https://github.com/airvzxf/long-multiplication-calculator\n";

/// Store the symbol description of the long multiplication.
///
/// It generates the table symbols for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// generate::symbols(&mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn symbols<W: fmt::Write + ?Sized>(out: &mut W) -> fmt::Result {
    out.write_str(SYMBOLS)
}

/// Store the top border of the long multiplication.
///
/// It generates the table top-border for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
///
/// Example #1
/// ```rust
/// let multiplicand: &str = "2";
/// let multiplier: &str = "5";
/// let mut text: String = String::from("");
/// let expected: &str = "┏━━━━━━━┓\n";
///
/// use long_multiplication_core::generate;
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::top_border(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::top_border(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn top_border<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    out.write_char('┏')?;
    repeat(out, '━', inner_width(process.columns()))?;
    out.write_str("┓\n")
}

/// Store the bottom border of the long multiplication.
///
/// It generates the table bottom-border for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
///
/// Example #1
/// ```rust
/// let multiplicand: &str = "2";
/// let multiplier: &str = "5";
/// let mut text: String = String::from("");
/// let expected: &str = "┗━━━┷━━━┛\n";
///
/// use long_multiplication_core::generate;
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::bottom_border(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::bottom_border(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn bottom_border<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    rule(out, process.columns(), ['┗', '━', '┷', '┛'])
}

/// Store the position title of the long multiplication.
///
/// It generates the table position-title for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::position_title(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::position_title(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn position_title<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    let length: usize = process.columns();
    let width: usize = cell_width(length);

    title(out, length, "Pos.")?;
    rule(out, length, ['┠', '┄', '┬', '┨'])?;

    // Column numbers, from the highest on the left to 1 on the right.
    out.write_char('┃')?;
    for n in 0..length {
        if n > 0 {
            out.write_char('│')?;
        }
        push_cell(out, width, Cell::Number(length - n))?;
    }
    out.write_str("┃\n")?;

    rule(out, length, ['┣', '━', '┷', '┫'])
}

/// Store the operation title of the long multiplication.
///
/// It generates the table operation-title for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::operation_title(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::operation_title(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn operation_title<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    let length: usize = process.columns();

    title(out, length, "Ops.")?;
    rule(out, length, ['┣', '━', '┯', '┫'])
}

/// Store the multiplication section of the long multiplication.
///
/// It generates the table multiplication-section for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::multiplication(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::multiplication(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn multiplication<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    let multiplicand: &str = process.multiplicand();
    let multiplier: &str = process.multiplier();
    let length: usize = process.columns();
    let width: usize = cell_width(length);

    // Multiplicand, right-aligned.
    let mut row = Row::new(out, width)?;
    row.empty(length - multiplicand.len())?;
    row.chars(multiplicand.chars())?;
    row.finish(format_args!(""))?;

    // Multiplier, right-aligned, with the `x` sign in the first cell.
    let mut row = Row::new(out, width)?;
    row.chars(['x'])?;
    row.empty(length - multiplier.len() - 1)?;
    row.chars(multiplier.chars())?;
    row.finish(format_args!(""))?;

    rule(out, length, ['┣', '━', '┿', '┫'])
}

/// Store the operations section of the long multiplication.
///
/// It generates the table operations-section for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::operations(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::operations(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn operations<W: fmt::Write + ?Sized>(
    process: &LongMultiplication,
    out: &mut W,
) -> fmt::Result {
    let step: usize = process.multiplicand().len();
    let length: usize = process.columns();
    let width: usize = cell_width(length);
    let rows: usize = process.multiplier().len();

    for partial in process.rows() {
        let iteration: usize = partial.row;

        // Carries of this row, shifted one column to the left.
        let mut row = Row::new(out, width)?;
        row.empty(length - step - iteration)?;
        row.digits(partial.carries.iter().rev().copied())?;
        row.empty(iteration)?;
        row.finish(format_args!(" {iteration} ^"))?;

        rule(out, length, ['┠', '┈', '┼', '┨'])?;

        // Units of this row.
        let mut row = Row::new(out, width)?;
        row.empty(length - step - iteration + 1)?;
        row.digits(partial.units.iter().rev().copied())?;
        row.empty(iteration - 1)?;
        row.finish(format_args!(" {iteration} R"))?;

        if iteration == rows {
            break;
        }
        rule(out, length, ['┠', '─', '┼', '┨'])?;
    }

    rule(out, length, ['┣', '━', '┷', '┫'])
}

/// Store the sum title of the long multiplication.
///
/// It generates the table sum-title for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::sum_title(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::sum_title(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn sum_title<W: fmt::Write + ?Sized>(process: &LongMultiplication, out: &mut W) -> fmt::Result {
    let length: usize = process.columns();

    title(out, length, "Sum.")?;
    rule(out, length, ['┣', '━', '┯', '┫'])
}

/// Store the long-sum section of the long multiplication.
///
/// It generates the table long-sum-section for the
/// long multiplication and stores it in a text variable.
///
/// It means that sums the rows for each column.
///
/// # Examples
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::long_sum(&process, &mut text).expect("a String never fails");
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
/// use long_multiplication_core::LongMultiplication;
/// let process = LongMultiplication::new(multiplicand, multiplier).expect("digits");
/// generate::long_sum(&process, &mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn long_sum<W: fmt::Write + ?Sized>(process: &LongMultiplication, out: &mut W) -> fmt::Result {
    let length: usize = process.columns();
    let width: usize = cell_width(length);
    generate_rows_with_numbers(process.sum(), length, out)?;

    for (index, subtotal) in process.subtotals().enumerate() {
        rule(out, length, ['┣', '━', '┷', '┫'])?;
        title_sub(out, length, index + 1)?;
        rule(out, length, ['┣', '━', '┯', '┫'])?;

        generate_rows_with_numbers(&subtotal, length, out)?;
    }

    rule(out, length, ['┣', '━', '┷', '┫'])?;
    title(out, length, "Pro.")?;
    rule(out, length, ['┣', '━', '┯', '┫'])?;

    // The product, most significant digit first.
    let mut row = Row::new(out, width)?;
    row.digits(process.product().iter().rev().copied())?;
    row.finish(format_args!(" P"))
}

/// Store the author section of the long multiplication.
///
/// It generates the table author-section for the
/// long multiplication and stores it in a text variable.
///
/// # Examples
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
/// generate::author(&mut text).expect("a String never fails");
///
/// assert_eq!(expected, text);
/// ```
pub fn author<W: fmt::Write + ?Sized>(out: &mut W) -> fmt::Result {
    out.write_str(AUTHOR)
}

fn generate_rows_with_numbers<W: fmt::Write + ?Sized>(
    numbers: &[usize],
    length: usize,
    out: &mut W,
) -> fmt::Result {
    let width: usize = cell_width(length);

    for (column, &number) in numbers.iter().enumerate() {
        // The sum of column `column` is written so that its units
        // digit lands in that column; its tens spill to the left.
        let digits: usize = get_number_length(number);
        let mut row = Row::new(out, width)?;
        row.empty(length - column - digits)?;
        row.number_digits(number, digits)?;
        row.empty(column)?;
        row.finish(format_args!(" {} C", column + 1))?;

        if column + 1 == length {
            break;
        }
        rule(out, length, ['┠', '┈', '┼', '┨'])?;
    }
    Ok(())
}

pub(crate) fn cell_width(length: usize) -> usize {
    get_number_length(length).max(3)
}

pub(crate) fn inner_width(length: usize) -> usize {
    length * cell_width(length) + length - 1
}

pub(crate) fn needs_subtotal(numbers: &[usize]) -> bool {
    numbers.iter().any(|&number| number > 9)
}

/// Write `ch` `count` times. The fills used by the table are written
/// from static runs, a few calls instead of one per character (a
/// streaming writer pays for every call).
fn repeat<W: fmt::Write + ?Sized>(out: &mut W, ch: char, count: usize) -> fmt::Result {
    const SPACES: &str = "                ";
    const HEAVY: &str = "━━━━━━━━━━━━━━━━";
    const DASH: &str = "┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄";
    const DOTS: &str = "┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈";
    const LIGHT: &str = "────────────────";
    const RUN: usize = 16;

    let run: &str = match ch {
        ' ' => SPACES,
        '━' => HEAVY,
        '┄' => DASH,
        '┈' => DOTS,
        '─' => LIGHT,
        _ => {
            for _ in 0..count {
                out.write_char(ch)?;
            }
            return Ok(());
        }
    };
    let size: usize = ch.len_utf8();
    let mut left: usize = count;
    while left > 0 {
        let n: usize = left.min(RUN);
        out.write_str(&run[..n * size])?;
        left -= n;
    }
    Ok(())
}

fn rule<W: fmt::Write + ?Sized>(
    out: &mut W,
    length: usize,
    [left, fill, separator, right]: [char; 4],
) -> fmt::Result {
    let width: usize = cell_width(length);
    out.write_char(left)?;
    for n in 0..length {
        if n > 0 {
            out.write_char(separator)?;
        }
        repeat(out, fill, width)?;
    }
    out.write_char(right)?;
    out.write_char('\n')
}

fn title<W: fmt::Write + ?Sized>(out: &mut W, length: usize, label: &str) -> fmt::Result {
    out.write_char('┃')?;
    out.write_str(label)?;
    repeat(out, ' ', inner_width(length).saturating_sub(label.chars().count()))?;
    out.write_str("┃\n")
}

/// The `Sub n.` title, without building the label in a `String`.
fn title_sub<W: fmt::Write + ?Sized>(out: &mut W, length: usize, index: usize) -> fmt::Result {
    // "Sub " + digits + "."
    let label_len: usize = 5 + get_number_length(index);
    out.write_char('┃')?;
    write!(out, "Sub {index}.")?;
    repeat(out, ' ', inner_width(length).saturating_sub(label_len))?;
    out.write_str("┃\n")
}

/// Content of one cell.
enum Cell {
    Char(char),
    Number(usize),
}

fn push_cell<W: fmt::Write + ?Sized>(out: &mut W, width: usize, content: Cell) -> fmt::Result {
    let len: usize = match content {
        Cell::Char(_) => 1,
        Cell::Number(number) => get_number_length(number),
    };
    let right: usize = usize::from(len + 2 <= width);
    let left: usize = width.saturating_sub(len + right);
    repeat(out, ' ', left)?;
    match content {
        Cell::Char(ch) => out.write_char(ch)?,
        Cell::Number(number) => write!(out, "{number}")?,
    }
    repeat(out, ' ', right)
}

struct Row<'a, W: fmt::Write + ?Sized> {
    out: &'a mut W,
    width: usize,
    first: bool,
}

impl<'a, W: fmt::Write + ?Sized> Row<'a, W> {
    fn new(out: &'a mut W, width: usize) -> Result<Self, fmt::Error> {
        out.write_char('┃')?;
        Ok(Self { out, width, first: true })
    }

    fn separator(&mut self) -> fmt::Result {
        if !self.first {
            self.out.write_char('│')?;
        }
        self.first = false;
        Ok(())
    }

    fn empty(&mut self, count: usize) -> fmt::Result {
        for _ in 0..count {
            self.separator()?;
            repeat(self.out, ' ', self.width)?;
        }
        Ok(())
    }

    fn chars(&mut self, chars: impl IntoIterator<Item = char>) -> fmt::Result {
        for ch in chars {
            self.cell(ch)?;
        }
        Ok(())
    }

    /// One cell holding `ch`, with its separator, in a single write.
    fn cell(&mut self, ch: char) -> fmt::Result {
        let mut buffer = [0u8; 64];
        let separator: usize = if self.first { 0 } else { '│'.len_utf8() };
        let size: usize = separator + self.width + ch.len_utf8();
        if size > buffer.len() || self.width == 0 {
            self.separator()?;
            return push_cell(self.out, self.width, Cell::Char(ch));
        }
        let right: usize = usize::from(3 <= self.width);
        let left: usize = self.width - 1 - right;
        let mut at: usize = 0;
        if separator > 0 {
            at += '│'.encode_utf8(&mut buffer[at..]).len();
        }
        buffer[at..at + left].fill(b' ');
        at += left;
        at += ch.encode_utf8(&mut buffer[at..]).len();
        buffer[at..at + right].fill(b' ');
        at += right;
        self.first = false;
        match core::str::from_utf8(&buffer[..at]) {
            Ok(text) => self.out.write_str(text),
            Err(_) => Err(fmt::Error),
        }
    }

    fn digits(&mut self, digits: impl IntoIterator<Item = u8>) -> fmt::Result {
        self.chars(digits.into_iter().map(|digit| char::from(b'0' + digit % 10)))
    }

    /// The `count` decimal digits of `number`, one per cell.
    fn number_digits(&mut self, number: usize, count: usize) -> fmt::Result {
        let mut divisor: usize = 1;
        for _ in 1..count {
            divisor *= 10;
        }
        for _ in 0..count {
            self.cell(char::from(b'0' + ((number / divisor) % 10) as u8))?;
            divisor /= 10;
        }
        Ok(())
    }

    /// Close the row and append `suffix` (for example ` 1 ^`).
    fn finish(self, suffix: fmt::Arguments<'_>) -> fmt::Result {
        self.out.write_char('┃')?;
        self.out.write_fmt(suffix)?;
        self.out.write_char('\n')
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
        symbols(&mut text).unwrap();

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
        top_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        top_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        top_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        top_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        bottom_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        bottom_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        bottom_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        bottom_border(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        position_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        position_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        position_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        position_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        operation_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        operation_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        operation_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        operation_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        multiplication(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text)
            .unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        operations(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        sum_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        sum_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        sum_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        sum_title(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        long_sum(&LongMultiplication::from_digits(multiplicand, multiplier), &mut text).unwrap();

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
        author(&mut text).unwrap();

        // Assert
        assert_eq!(expected, text);
    }
}
