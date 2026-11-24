//! The long-multiplication process as data.
//!
//! [`LongMultiplication`] holds every step the table shows: the
//! partial products of each multiplier digit (units and carries), the
//! sum of each column, every `Sub n.` section and the product. The
//! Unicode table ([`LongMultiplication::write_table`]) and any other
//! format (for example the CLI's JSON) are written from it.
//!
//! Only the column sums and the product are stored, so the memory
//! used grows with the number of columns, not with the size of the
//! table. [`LongMultiplication::rows`] and
//! [`LongMultiplication::subtotals`] recompute each row or section
//! while they are iterated, which costs about as much as writing it.
//!
//! # Conventions
//!
//! Columns are numbered from the right, as in the `Pos.` row: column
//! 1 holds the units. Every slice indexed by column starts with
//! column 1 at index `0`.

use core::fmt;
use core::iter::FusedIterator;

use crate::breakdown::break_down_subtotal;
use crate::generate;
use crate::{Result, validate_input};

/// Every step of the long multiplication of two numbers.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::LongMultiplication;
///
/// let process = LongMultiplication::new("123", "456")?;
/// assert_eq!(process.columns(), 6);
/// // Column sums, column 1 (the units) first.
/// assert_eq!(process.sum(), &[8, 8, 10, 15, 4, 0]);
/// assert_eq!(process.subtotals().count(), 0);
/// // Product digits, column 1 first: 056088 read from the left.
/// assert_eq!(process.product(), &[8, 8, 0, 6, 5, 0]);
/// # Ok::<(), long_multiplication_core::CoreError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongMultiplication<'a> {
    multiplicand: &'a str,
    multiplier: &'a str,
    sum: Vec<usize>,
    subtotal_count: usize,
    product: Vec<u8>,
}

/// The partial product of one multiplier digit: rows `n ^` and `n R`.
///
/// `units[k]` and `carries[k]` come from the multiplicand digit `k`
/// counted from the right (`k = 0` is its units digit). The unit
/// lands in column `k + row` and the carry in column `k + row + 1`.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::LongMultiplication;
///
/// let process = LongMultiplication::new("123", "456")?;
/// let first = process.rows().next().expect("one row per multiplier digit");
/// // 3 x 6 = 18, 2 x 6 = 12, 1 x 6 = 6.
/// assert_eq!((first.row, first.multiplier_digit), (1, 6));
/// assert_eq!(first.units, vec![8, 2, 6]);
/// assert_eq!(first.carries, vec![1, 1, 0]);
/// # Ok::<(), long_multiplication_core::CoreError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialProduct {
    /// Row number, from 1 (the units digit of the multiplier).
    pub row: usize,
    /// The multiplier digit of this row.
    pub multiplier_digit: u8,
    /// Units of each digit product, multiplicand digit 0 (units) first.
    pub units: Vec<u8>,
    /// Carries of each digit product, multiplicand digit 0 (units) first.
    pub carries: Vec<u8>,
}

impl<'a> LongMultiplication<'a> {
    /// Compute the process for two numbers.
    ///
    /// Leading and trailing whitespace is ignored, as in
    /// [`validate_input`]; there is no limit on the number of digits.
    ///
    /// # Errors
    ///
    /// [`crate::CoreError::EmptyInput`] or
    /// [`crate::CoreError::InvalidDigit`] when an operand is not a
    /// number made of the digits 0-9.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::{CoreError, LongMultiplication};
    ///
    /// let process = LongMultiplication::new(" 5 ", "7")?;
    /// assert_eq!(process.multiplicand(), "5");
    /// assert!(matches!(LongMultiplication::new("5", "x"), Err(CoreError::InvalidDigit { .. })));
    /// # Ok::<(), CoreError>(())
    /// ```
    pub fn new(multiplicand: &'a str, multiplier: &'a str) -> Result<Self> {
        let multiplicand = validate_input(multiplicand, usize::MAX)?;
        let multiplier = validate_input(multiplier, usize::MAX)?;
        Ok(Self::from_digits(multiplicand, multiplier))
    }

    /// Compute the process for operands already known to be digits.
    pub(crate) fn from_digits(multiplicand: &'a str, multiplier: &'a str) -> Self {
        let a = multiplicand.as_bytes();
        let b = multiplier.as_bytes();
        let mut sum: Vec<usize> = vec![0; a.len() + b.len()];

        // Digit `k` of the multiplicand times digit `r` of the
        // multiplier (both counted from the right): the unit goes to
        // column k + r, the carry to the next column.
        for (r, &multiplier_digit) in b.iter().rev().enumerate() {
            let y = digit(multiplier_digit);
            for (k, &multiplicand_digit) in a.iter().rev().enumerate() {
                let product = digit(multiplicand_digit) * y;
                sum[k + r] += product % 10;
                sum[k + r + 1] += product / 10;
            }
        }

        // The first carry pass is not shown; every pass that still
        // leaves a column above 9 is a `Sub n.` section.
        let mut stage: Vec<usize> = break_down_subtotal(&sum);
        let mut subtotal_count: usize = 0;
        while generate::needs_subtotal(&stage) {
            subtotal_count += 1;
            stage = break_down_subtotal(&stage);
        }
        let product: Vec<u8> = stage.iter().map(|&d| (d % 10) as u8).collect();

        Self { multiplicand, multiplier, sum, subtotal_count, product }
    }

    /// The multiplicand, without surrounding whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// assert_eq!(LongMultiplication::new("13597", "8642")?.multiplicand(), "13597");
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn multiplicand(&self) -> &'a str {
        self.multiplicand
    }

    /// The multiplier, without surrounding whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// assert_eq!(LongMultiplication::new("13597", "8642")?.multiplier(), "8642");
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn multiplier(&self) -> &'a str {
        self.multiplier
    }

    /// Number of columns of the table: the digits of both operands.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// assert_eq!(LongMultiplication::new("13597", "8642")?.columns(), 9);
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn columns(&self) -> usize {
        self.sum.len()
    }

    /// The partial product of each multiplier digit, row 1 first.
    ///
    /// Each row is computed while iterating; nothing is stored.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// let process = LongMultiplication::new("123", "456")?;
    /// let digits: Vec<u8> = process.rows().map(|row| row.multiplier_digit).collect();
    /// assert_eq!(digits, vec![6, 5, 4]);
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn rows(&self) -> impl ExactSizeIterator<Item = PartialProduct> + FusedIterator + '_ {
        let a = self.multiplicand.as_bytes();
        let b = self.multiplier.as_bytes();
        (0..b.len()).map(move |r| {
            let y = digit(b[b.len() - 1 - r]);
            let (units, carries): (Vec<u8>, Vec<u8>) = a
                .iter()
                .rev()
                .map(|&x| {
                    let product = digit(x) * y;
                    ((product % 10) as u8, (product / 10) as u8)
                })
                .unzip();
            PartialProduct { row: r + 1, multiplier_digit: y as u8, units, carries }
        })
    }

    /// The sum of each column, column 1 first (rows `n C`).
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// assert_eq!(LongMultiplication::new("99", "99")?.sum(), &[1, 10, 17, 8]);
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn sum(&self) -> &[usize] {
        &self.sum
    }

    /// Every `Sub n.` section, `Sub 1.` first, each one column 1 first.
    ///
    /// Carrying the tens of every column sum once gives the next
    /// stage. The first stage is shown only when a column is still
    /// above 9; otherwise it is already the product. Each section is
    /// computed while iterating.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// // 99 x 99: one carry pass already gives the product, no `Sub`.
    /// assert_eq!(LongMultiplication::new("99", "99")?.subtotals().count(), 0);
    ///
    /// let nines = "9".repeat(12);
    /// let process = LongMultiplication::new(&nines, &nines)?;
    /// assert_eq!(process.subtotals().len(), 10);
    /// assert!(process.subtotals().all(|stage| stage.iter().any(|&n| n > 9)));
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn subtotals(&self) -> Subtotals {
        Subtotals { next: break_down_subtotal(&self.sum), remaining: self.subtotal_count }
    }

    /// The product, one digit per column, column 1 first.
    ///
    /// It has [`columns`](Self::columns) digits, so it can start with
    /// zeros, as the `P` row of the table does.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::LongMultiplication;
    ///
    /// let process = LongMultiplication::new("5", "7")?;
    /// assert_eq!(process.product(), &[5, 3]);
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn product(&self) -> &[u8] {
        &self.product
    }

    /// Write the Unicode table, the same bytes as [`crate::get_table`].
    ///
    /// Any [`fmt::Write`] works: a `String`, or an adapter that sends
    /// the text straight to a file, so the table never has to fit in
    /// memory.
    ///
    /// # Errors
    ///
    /// Only the errors returned by `out`.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::{LongMultiplication, get_table};
    ///
    /// let mut text = String::new();
    /// LongMultiplication::new("13597", "8642")?.write_table(&mut text).expect("a String never fails");
    /// assert_eq!(text, get_table("13597", "8642"));
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn write_table<W: fmt::Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        generate::symbols(out)?;
        generate::top_border(self, out)?;
        generate::position_title(self, out)?;
        generate::operation_title(self, out)?;
        generate::multiplication(self, out)?;
        generate::operations(self, out)?;
        generate::sum_title(self, out)?;
        generate::long_sum(self, out)?;
        generate::bottom_border(self, out)?;
        generate::author(out)
    }

    /// Upper bound of the size, in bytes, of the Unicode table.
    ///
    /// Same value as [`crate::estimate_table_bytes`], without
    /// computing the column sums again.
    ///
    /// # Examples
    ///
    /// ```
    /// use long_multiplication_core::{LongMultiplication, get_table};
    ///
    /// let process = LongMultiplication::new("13597", "8642")?;
    /// assert!(process.estimate_table_bytes() >= get_table("13597", "8642").len());
    /// # Ok::<(), long_multiplication_core::CoreError>(())
    /// ```
    pub fn estimate_table_bytes(&self) -> usize {
        let length: usize = self.columns();

        // Rows: borders and titles (16), four per multiplier digit, the
        // column sums (2L - 1 plus the closing rule) and 2L + 2 per subtotal.
        let rows: usize = 16_usize
            .saturating_add(self.multiplier.len().saturating_mul(4))
            .saturating_add(length.saturating_mul(2))
            .saturating_add(
                self.subtotal_count.saturating_mul(length.saturating_mul(2).saturating_add(2)),
            );

        // The widest row is a rule: every character is 3 bytes in UTF-8.
        // Add room for the row suffix (` 123 ^`) and the newline.
        let row_bytes: usize =
            generate::inner_width(length).saturating_add(2).saturating_mul(3).saturating_add(24);

        rows.saturating_mul(row_bytes)
            .saturating_add(generate::SYMBOLS.len())
            .saturating_add(generate::AUTHOR.len())
    }
}

/// Iterator over the `Sub n.` sections; see [`LongMultiplication::subtotals`].
#[derive(Debug, Clone)]
pub struct Subtotals {
    next: Vec<usize>,
    remaining: usize,
}

impl Iterator for Subtotals {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let following =
            if self.remaining == 0 { Vec::new() } else { break_down_subtotal(&self.next) };
        Some(core::mem::replace(&mut self.next, following))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for Subtotals {}
impl FusedIterator for Subtotals {}

/// Value of an ASCII digit.
fn digit(byte: u8) -> usize {
    usize::from(byte.wrapping_sub(b'0'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn operand(max_len: usize) -> impl Strategy<Value = String> {
        proptest::collection::vec(0u8..10, 1..=max_len)
            .prop_map(|digits| digits.into_iter().map(|d| char::from(b'0' + d)).collect())
    }

    /// Units and carries of every row, row 1 first and each row in
    /// multiplicand order (left to right): the layout of the table.
    fn unit_and_carry_rows(multiplicand: &str, multiplier: &str) -> (Vec<usize>, Vec<usize>) {
        let process = LongMultiplication::new(multiplicand, multiplier).unwrap();
        let mut units = Vec::new();
        let mut carries = Vec::new();
        for row in process.rows() {
            units.extend(row.units.iter().rev().map(|&d| usize::from(d)));
            carries.extend(row.carries.iter().rev().map(|&d| usize::from(d)));
        }
        (units, carries)
    }

    proptest! {
        /// Rows are the digit products, the column sums add up the rows,
        /// and the subtotals carry the sums until the product.
        #[test]
        fn rows_sums_subtotals_and_product_agree(a in operand(40), b in operand(40)) {
            let process = LongMultiplication::new(&a, &b).expect("digits");
            let (a_digits, b_digits) = (a.as_bytes(), b.as_bytes());
            let mut sum = vec![0usize; a.len() + b.len()];
            for row in process.rows() {
                let y = usize::from(b_digits[b.len() - row.row] - b'0');
                prop_assert_eq!(usize::from(row.multiplier_digit), y);
                for k in 0..a.len() {
                    let product = usize::from(a_digits[a.len() - 1 - k] - b'0') * y;
                    prop_assert_eq!(usize::from(row.units[k]), product % 10);
                    prop_assert_eq!(usize::from(row.carries[k]), product / 10);
                    sum[k + row.row - 1] += usize::from(row.units[k]);
                    sum[k + row.row] += usize::from(row.carries[k]);
                }
            }
            prop_assert_eq!(process.sum(), sum.as_slice());

            let mut stage = break_down_subtotal(process.sum());
            for subtotal in process.subtotals() {
                prop_assert!(subtotal.iter().any(|&n| n > 9));
                prop_assert_eq!(&subtotal, &stage);
                stage = break_down_subtotal(&stage);
            }
            prop_assert!(stage.iter().all(|&n| n < 10));
            let product: Vec<u8> = stage.iter().map(|&d| d as u8).collect();
            prop_assert_eq!(process.product(), product.as_slice());
        }
    }

    #[test]
    fn rows_and_subtotals_report_their_length() {
        let nines = "9".repeat(25);
        let process = LongMultiplication::new(&nines, &nines[..3]).expect("digits");
        assert_eq!(process.rows().len(), 3);
        let process = LongMultiplication::new(&nines, &nines).expect("digits");
        let mut subtotals = process.subtotals();
        let total = subtotals.len();
        assert!(subtotals.next().is_some());
        assert_eq!(subtotals.len(), total - 1);
        assert_eq!(subtotals.by_ref().count(), total - 1);
        assert_eq!(subtotals.next(), None);
    }

    #[test]
    fn new_rejects_what_validate_input_rejects() {
        assert!(LongMultiplication::new("", "1").is_err());
        assert!(LongMultiplication::new("1", "1.5").is_err());
        assert!(LongMultiplication::new("12", "-3").is_err());
    }

    #[test]
    fn write_table_propagates_writer_errors() {
        struct Full(usize);
        impl fmt::Write for Full {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                if self.0 < s.len() {
                    return Err(fmt::Error);
                }
                self.0 -= s.len();
                Ok(())
            }
        }
        let process = LongMultiplication::new("13597", "8642").expect("digits");
        assert!(process.write_table(&mut Full(500)).is_err());
        assert!(process.write_table(&mut Full(usize::MAX)).is_ok());
    }

    // # -----------------------------------------------------------------------
    // # LongMultiplication::rows (cases of the former break_down_multiplication)
    // # -----------------------------------------------------------------------
    #[test]
    fn rows_with_three_digits_multiplicand_is_greater() {
        let multiplicand: &str = "25";
        let multiplier: &str = "3";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 5];
        let expected_carry: Vec<usize> = vec![0, 1];

        (operation_unit, operation_carry) = unit_and_carry_rows(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn rows_with_three_digits_multiplier_is_greater() {
        let multiplicand: &str = "3";
        let multiplier: &str = "25";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![5, 6];
        let expected_carry: Vec<usize> = vec![1, 0];

        (operation_unit, operation_carry) = unit_and_carry_rows(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn rows_with_four_digit() {
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 8, 2, 6];
        let expected_carry: Vec<usize> = vec![0, 1, 0, 0];

        (operation_unit, operation_carry) = unit_and_carry_rows(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    #[test]
    fn rows_with_six_digit() {
        let multiplicand: &str = "123";
        let multiplier: &str = "456";
        let operation_unit: Vec<usize>;
        let operation_carry: Vec<usize>;
        let expected_unit: Vec<usize> = vec![6, 2, 8, 5, 0, 5, 4, 8, 2];
        let expected_carry: Vec<usize> = vec![0, 1, 1, 0, 1, 1, 0, 0, 1];

        (operation_unit, operation_carry) = unit_and_carry_rows(multiplicand, multiplier);

        assert_eq!(expected_unit, operation_unit);
        assert_eq!(expected_carry, operation_carry);
    }

    // # -----------------------------------------------------------------------
    // # LongMultiplication::sum (cases of the former break_down_addition)
    // # -----------------------------------------------------------------------
    #[test]
    fn sum_product_one_digit() {
        let multiplicand: &str = "2";
        let multiplier: &str = "3";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 0];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_product_two_digits() {
        let multiplicand: &str = "9";
        let multiplier: &str = "8";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![2, 7];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_three_digits() {
        let multiplicand: &str = "37";
        let multiplier: &str = "8";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_three_digits_switch() {
        let multiplicand: &str = "8";
        let multiplier: &str = "37";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 9, 2];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_four_digit() {
        let multiplicand: &str = "13";
        let multiplier: &str = "26";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 13, 2, 0];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_six_digit() {
        let multiplicand: &str = "123";
        let multiplier: &str = "456";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![8, 8, 10, 15, 4, 0];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_eleven_digits_multiplier_is_greater() {
        let multiplicand: &str = "78924358";
        let multiplier: &str = "357";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }

    #[test]
    fn sum_with_eleven_digits_multiplier_is_less() {
        let multiplicand: &str = "357";
        let multiplier: &str = "78924358";
        let addition: Vec<usize>;
        let expected_addition: Vec<usize> = vec![6, 10, 17, 24, 17, 8, 25, 25, 19, 6, 2];

        addition = LongMultiplication::new(multiplicand, multiplier).unwrap().sum().to_vec();

        assert_eq!(expected_addition, addition);
    }
}
