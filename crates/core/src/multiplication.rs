//! High-level orchestration of the long-multiplication table.

use crate::breakdown::{break_down_addition, break_down_subtotal};
use crate::generate::{self, inner_width, needs_subtotal};
use crate::{CoreError, Result};

/// Return the table of the long multiplication.
///
/// It generates the complete table for the
/// long multiplication and returns it in a text variable.
///
/// This is the public entry point of the core crate: it stitches the
/// per-section Unicode generators together in the same order the
/// school layout expects (symbols, position, operation, partial
/// products, sum, subtotals, product, author). I/O is intentionally
/// not performed here; see the `cli` and `wasm` crates for adapters.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::get_table;
///
/// let text: String = get_table("5", "7");
/// // 5 * 7 = 35; the product row reads "┃ 3 │ 5 ┃ P".
/// assert!(text.contains("3 │ 5 ┃ P"));
/// assert!(text.contains("Symbols"));
/// ```
pub fn get_table(multiplicand: &str, multiplier: &str) -> String {
    let mut content: String = String::with_capacity(estimate_table_bytes(multiplicand, multiplier));

    generate::symbols(&mut content);
    generate::top_border(multiplicand, multiplier, &mut content);
    generate::position_title(multiplicand, multiplier, &mut content);
    generate::operation_title(multiplicand, multiplier, &mut content);
    generate::multiplication(multiplicand, multiplier, &mut content);
    generate::operations(multiplicand, multiplier, &mut content);
    generate::sum_title(multiplicand, multiplier, &mut content);
    generate::long_sum(multiplicand, multiplier, &mut content);
    generate::bottom_border(multiplicand, multiplier, &mut content);
    generate::author(&mut content);

    content
}

/// Upper bound of the size, in bytes, of [`get_table`]'s output.
///
/// Computes the number of rows (including every `Sub n.` section)
/// without rendering the table, so it costs a tiny fraction of
/// [`get_table`]. The estimate is never below the real size and is
/// usually within 1.5x of it. Use it to refuse inputs whose table
/// would not fit in memory or on screen.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::{estimate_table_bytes, get_table};
///
/// let estimate = estimate_table_bytes("13597", "8642");
/// assert!(estimate >= get_table("13597", "8642").len());
///
/// // Nines need one `Sub n.` section per digit: the table grows fast.
/// let nines = "9".repeat(100);
/// assert!(estimate_table_bytes(&nines, &nines) > 50 * 1024 * 1024);
/// ```
pub fn estimate_table_bytes(multiplicand: &str, multiplier: &str) -> usize {
    let length: usize = multiplicand.len().saturating_add(multiplier.len());
    if multiplicand.is_empty() || multiplier.is_empty() {
        return 0;
    }

    // Number of `Sub n.` sections, exactly as `generate::long_sum` loops.
    let mut subtotals: usize = 0;
    let mut sums: Vec<usize> = break_down_subtotal(&break_down_addition(multiplicand, multiplier));
    while needs_subtotal(&sums) {
        subtotals += 1;
        sums = break_down_subtotal(&sums);
    }

    // Rows: borders and titles (16), four per multiplier digit, the
    // column sums (2L - 1 plus the closing rule) and 2L + 2 per subtotal.
    let rows: usize = 16_usize
        .saturating_add(multiplier.len().saturating_mul(4))
        .saturating_add(length.saturating_mul(2))
        .saturating_add(subtotals.saturating_mul(length.saturating_mul(2).saturating_add(2)));

    // The widest row is a rule: every character is 3 bytes in UTF-8.
    // Add room for the row suffix (` 123 ^`) and the newline.
    let row_bytes: usize =
        inner_width(length).saturating_add(2).saturating_mul(3).saturating_add(24);

    let mut frame: String = String::new();
    generate::symbols(&mut frame);
    generate::author(&mut frame);

    rows.saturating_mul(row_bytes).saturating_add(frame.len())
}

/// Check that the table for these operands fits in `max_bytes`.
///
/// Returns the estimate from [`estimate_table_bytes`] on success.
///
/// # Errors
///
/// Returns [`CoreError::OutputTooLarge`] when the estimate exceeds
/// `max_bytes`.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::{CoreError, check_output_size};
///
/// assert!(check_output_size("13597", "8642", 1024 * 1024).is_ok());
///
/// let nines = "9".repeat(200);
/// assert!(matches!(
///     check_output_size(&nines, &nines, 32 * 1024 * 1024),
///     Err(CoreError::OutputTooLarge { .. })
/// ));
/// ```
pub fn check_output_size(multiplicand: &str, multiplier: &str, max_bytes: usize) -> Result<usize> {
    let estimated: usize = estimate_table_bytes(multiplicand, multiplier);
    if estimated > max_bytes {
        return Err(CoreError::OutputTooLarge { estimated, max: max_bytes });
    }
    Ok(estimated)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Small deterministic generator, so the tests need no extra crate.
    fn pseudo_random_numbers(count: usize) -> Vec<String> {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        (0..count)
            .map(|_| {
                let len = (next() % 40 + 1) as usize;
                (0..len).map(|_| char::from(b'0' + (next() % 10) as u8)).collect()
            })
            .collect()
    }

    fn display_width(line: &str) -> usize {
        // Every table row ends with its right border; the suffix after
        // it (` 1 ^`, ` 2 C`, ` P`) is not part of the frame.
        let end = line.rfind(['┓', '┃', '┨', '┫', '┛']).unwrap_or(line.len());
        line[..end].chars().count()
    }

    fn assert_table_invariants(multiplicand: &str, multiplier: &str) {
        let table = get_table(multiplicand, multiplier);

        let rows: Vec<&str> =
            table.lines().filter(|line| line.starts_with(['┏', '┃', '┠', '┣', '┗'])).collect();
        let width = display_width(rows[0]);
        for row in &rows {
            assert_eq!(
                display_width(row),
                width,
                "misaligned row in {multiplicand} x {multiplier}: {row}"
            );
        }

        let product: String = rows
            .iter()
            .find(|row| row.ends_with("┃ P"))
            .map(|row| row.chars().filter(char::is_ascii_digit).collect())
            .unwrap_or_default();
        let expected = schoolbook_product(multiplicand, multiplier);
        assert_eq!(product, expected, "wrong product for {multiplicand} x {multiplier}");

        let estimate = estimate_table_bytes(multiplicand, multiplier);
        assert!(estimate >= table.len(), "estimate {estimate} < real {}", table.len());
        assert!(estimate <= table.len() * 2, "estimate {estimate} too loose for {}", table.len());
    }

    /// Reference product with leading zeros, padded to len(a) + len(b).
    fn schoolbook_product(a: &str, b: &str) -> String {
        let a: Vec<u32> = a.bytes().rev().map(|d| u32::from(d - b'0')).collect();
        let b: Vec<u32> = b.bytes().rev().map(|d| u32::from(d - b'0')).collect();
        let mut out = vec![0u32; a.len() + b.len()];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] += x * y;
            }
        }
        for i in 0..out.len() - 1 {
            out[i + 1] += out[i] / 10;
            out[i] %= 10;
        }
        out.iter().rev().map(|d| char::from_digit(*d, 10).unwrap()).collect()
    }

    #[test]
    fn tables_are_aligned_and_correct_for_many_inputs() {
        let numbers = pseudo_random_numbers(120);
        for pair in numbers.chunks(2) {
            assert_table_invariants(&pair[0], &pair[1]);
        }
    }

    #[test]
    fn tables_with_many_subtotals_stay_aligned() {
        // 25 nines need more than ten `Sub n.` sections.
        let nines = "9".repeat(25);
        assert!(get_table(&nines, &nines).contains("┃Sub 12."));
        assert_table_invariants(&nines, &nines);
    }

    #[test]
    fn tables_with_more_than_999_columns_stay_aligned() {
        let a = "1".repeat(600);
        let b = "2".repeat(500);
        let table = get_table(&a, &b);
        assert!(table.contains("┃1100│1099│"), "position row uses 4-character cells");
        assert!(table.contains("│ 999│"), "3-digit positions are right-aligned");
        assert!(table.contains("│  1 ┃"), "1-digit positions keep a trailing space");
        assert_table_invariants(&a, &b);
    }

    #[test]
    fn check_output_size_rejects_and_accepts() {
        let nines = "9".repeat(60);
        let estimate = estimate_table_bytes(&nines, &nines);
        assert_eq!(check_output_size(&nines, &nines, estimate), Ok(estimate));
        assert_eq!(
            check_output_size(&nines, &nines, estimate - 1),
            Err(CoreError::OutputTooLarge { estimated: estimate, max: estimate - 1 })
        );
    }

    // # -----------------------------------------------------------------------
    // # Function: get table
    // # -----------------------------------------------------------------------
    #[test]
    fn test_get_table_product_one_digits() {
        let multiplicand: &str = "3";
        let multiplier: &str = "2";
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
                              License: AGPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        let text: String = get_table(multiplicand, multiplier);

        assert_eq!(expected, text);
    }

    #[test]
    fn test_get_table_product_two_digits() {
        let multiplicand: &str = "5";
        let multiplier: &str = "7";
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
                              License: AGPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        let text: String = get_table(multiplicand, multiplier);

        assert_eq!(expected, text);
    }

    #[test]
    fn test_get_table_product_nine_digits() {
        let multiplicand: &str = "13597";
        let multiplier: &str = "8642";
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
                              License: AGPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        let text: String = get_table(multiplicand, multiplier);

        assert_eq!(expected, text);
    }
}
