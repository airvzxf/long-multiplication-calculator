//! High-level orchestration of the long-multiplication table.

use crate::generate;

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
    let mut content: String = String::from("");

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

/// Print the long-multiplication table to standard output.
///
/// Thin wrapper around [`get_table`] that writes the result to
/// `stdout`. Kept in the core crate because it has no dependency
/// beyond `std`; the `cli` crate may also use this helper.
///
/// # Examples
///
/// ```no_run
/// use long_multiplication_core::display;
///
/// display("3", "2");
/// ```
pub fn display(multiplicand: &str, multiplier: &str) {
    println!("{}", get_table(multiplicand, multiplier));
}

/// Write the long-multiplication table to a file.
///
/// The file is created (and overwritten if it exists). On error
/// the [`std::io::Error`] is propagated instead of being turned
/// into a `panic!` (the v1.0.0 binary panicked here, which is not
/// appropriate for a library).
///
/// # Errors
///
/// Returns any I/O error from `File::create` or `write_all`.
///
/// # Examples
///
/// ```no_run
/// use long_multiplication_core::store;
///
/// store("3", "2", "/tmp/long-multiplication.txt").unwrap();
/// ```
pub fn store(multiplicand: &str, multiplier: &str, file_path: &str) -> std::io::Result<()> {
    let content = get_table(multiplicand, multiplier);
    std::fs::write(file_path, &content)
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
                              License: GPL-3.0\n\
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
                              License: GPL-3.0\n\
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
                              License: GPL-3.0\n\
                              Project: https://github.com/airvzxf/long-multiplication-calculator\n";

        let text: String = get_table(multiplicand, multiplier);

        assert_eq!(expected, text);
    }

    // # -----------------------------------------------------------------------
    // # Function: store
    // # -----------------------------------------------------------------------
    #[test]
    fn test_store_successful() {
        // Arrange
        let file_path: &str = "/tmp/test-storage-01.txt";
        let _ = std::fs::remove_file(file_path);

        // Action
        store("3", "2", file_path).expect("store should succeed");

        // Assert
        let mut file = std::fs::File::open(file_path).expect("open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file");
        // 3 * 2 = 6; the table footer ends with "┃ 0 │ 6 ┃ P"
        assert!(content.contains("6 ┃ P"), "stored file missing product row: {content}");
        let _ = std::fs::remove_file(file_path);
    }

    #[test]
    fn test_store_fails_when_directory_missing() {
        // Arrange: a path whose parent does not exist.
        let file_path: &str = "/tmp/__no_such_dir__/test-storage-02.txt";

        // Action + Assert
        let result = store("3", "2", file_path);
        assert!(result.is_err(), "expected I/O error, got {result:?}");
    }
}
