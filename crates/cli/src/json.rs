//! The `--format json` output.
//!
//! Written by hand, one piece at a time, so a huge process never has
//! to fit in memory: every value is a number or a string of digits,
//! so nothing needs escaping. The layout is fixed (golden fixtures in
//! `tests/fixtures/*.json`):
//!
//! ```text
//! {
//!   "format_version": 1,
//!   "multiplicand": "123",
//!   "multiplier": "456",
//!   "columns": 6,
//!   "rows": [
//!     {"row": 1, "multiplier_digit": 6, "units": [8, 2, 6], "carries": [1, 1, 0]},
//!     ...
//!   ],
//!   "sum": [8, 8, 10, 15, 4, 0],
//!   "subtotals": [],
//!   "product": "056088"
//! }
//! ```
//!
//! Every array indexed by column starts with column 1 (the units);
//! `units[k]` and `carries[k]` belong to multiplicand digit `k`
//! counted from the right. `product` is the `P` row of the table,
//! leading zeros included.

use long_multiplication_core::LongMultiplication;
use std::fmt::{self, Write};

/// Version of the JSON layout; bumped on any incompatible change.
pub const FORMAT_VERSION: u32 = 1;

/// Write the whole process as JSON, ending with a newline.
pub fn write_json<W: Write + ?Sized>(process: &LongMultiplication, out: &mut W) -> fmt::Result {
    writeln!(out, "{{")?;
    writeln!(out, "  \"format_version\": {FORMAT_VERSION},")?;
    writeln!(out, "  \"multiplicand\": \"{}\",", process.multiplicand())?;
    writeln!(out, "  \"multiplier\": \"{}\",", process.multiplier())?;
    writeln!(out, "  \"columns\": {},", process.columns())?;

    writeln!(out, "  \"rows\": [")?;
    let rows = process.rows();
    let last = rows.len();
    for row in rows {
        write!(
            out,
            "    {{\"row\": {}, \"multiplier_digit\": {}, \"units\": ",
            row.row, row.multiplier_digit
        )?;
        array(out, row.units.iter())?;
        out.write_str(", \"carries\": ")?;
        array(out, row.carries.iter())?;
        out.write_str(if row.row == last { "}\n" } else { "},\n" })?;
    }
    writeln!(out, "  ],")?;

    out.write_str("  \"sum\": ")?;
    array(out, process.sum().iter())?;
    out.write_str(",\n")?;

    let subtotals = process.subtotals();
    if subtotals.len() == 0 {
        writeln!(out, "  \"subtotals\": [],")?;
    } else {
        writeln!(out, "  \"subtotals\": [")?;
        let mut remaining = subtotals.len();
        for stage in subtotals {
            remaining -= 1;
            out.write_str("    ")?;
            array(out, stage.iter())?;
            out.write_str(if remaining == 0 { "\n" } else { ",\n" })?;
        }
        writeln!(out, "  ],")?;
    }

    out.write_str("  \"product\": \"")?;
    for digit in process.product().iter().rev() {
        out.write_char(char::from(b'0' + digit % 10))?;
    }
    out.write_str("\"\n}\n")
}

/// Upper bound of the size, in bytes, of [`write_json`]'s output.
///
/// Cheap: it uses the stored column sums, never the rows. Every
/// `Sub n.` value is at most the largest column sum (a column keeps
/// its unit, at most 9, plus a tenth of its neighbour).
pub fn estimate_json_bytes(process: &LongMultiplication) -> usize {
    let a = process.multiplicand().len();
    let b = process.multiplier().len();
    let columns = process.columns();
    let largest = process.sum().iter().copied().max().unwrap_or(0).max(9);
    // Digits of the largest value plus ", ".
    let value = digits(largest).saturating_add(2);

    // `    {"row": R, "multiplier_digit": D, "units": [..], "carries": [..]},\n`
    let row = digits(b).saturating_add(70).saturating_add(a.saturating_mul(6));
    let rows = row.saturating_mul(b);
    let sum = columns.saturating_mul(value).saturating_add(16);
    let subtotal = columns.saturating_mul(value).saturating_add(8);
    let subtotals = subtotal.saturating_mul(process.subtotals().len()).saturating_add(32);

    // Header, operands, column count, product and closing lines.
    let fixed = 160_usize
        .saturating_add(a)
        .saturating_add(b)
        .saturating_add(digits(columns))
        .saturating_add(columns);

    fixed.saturating_add(rows).saturating_add(sum).saturating_add(subtotals)
}

fn array<'a, W: Write + ?Sized, T: fmt::Display + 'a>(
    out: &mut W,
    values: impl Iterator<Item = &'a T>,
) -> fmt::Result {
    out.write_char('[')?;
    for (index, value) in values.enumerate() {
        if index > 0 {
            out.write_str(", ")?;
        }
        write!(out, "{value}")?;
    }
    out.write_char(']')
}

/// Number of decimal digits of `number`.
fn digits(number: usize) -> usize {
    number.checked_ilog10().map_or(1, |log| log as usize + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json(a: &str, b: &str) -> String {
        let mut text = String::new();
        write_json(&LongMultiplication::new(a, b).unwrap(), &mut text).unwrap();
        text
    }

    #[test]
    fn matches_the_documented_layout() {
        let expected = "{\n  \"format_version\": 1,\n  \"multiplicand\": \"123\",\n  \"multiplier\": \"456\",\n  \"columns\": 6,\n  \"rows\": [\n    {\"row\": 1, \"multiplier_digit\": 6, \"units\": [8, 2, 6], \"carries\": [1, 1, 0]},\n    {\"row\": 2, \"multiplier_digit\": 5, \"units\": [5, 0, 5], \"carries\": [1, 1, 0]},\n    {\"row\": 3, \"multiplier_digit\": 4, \"units\": [2, 8, 4], \"carries\": [1, 0, 0]}\n  ],\n  \"sum\": [8, 8, 10, 15, 4, 0],\n  \"subtotals\": [],\n  \"product\": \"056088\"\n}\n";
        assert_eq!(json("123", "456"), expected);
    }

    #[test]
    fn lists_every_subtotal() {
        let text = json("9999", "9999");
        assert!(text.contains("  \"subtotals\": [\n    ["), "{text}");
        assert!(text.contains("]\n  ],\n  \"product\": \"99980001\""), "{text}");
    }

    #[test]
    fn estimate_is_an_upper_bound() {
        let nines = "9".repeat(60);
        let cases: [(&str, &str); 6] = [
            ("0", "0"),
            ("5", "7"),
            ("123", "456"),
            ("13597", "8642"),
            ("1", "98765432109876543210"),
            (&nines, &nines),
        ];
        for (a, b) in cases {
            let process = LongMultiplication::new(a, b).unwrap();
            let real = json(a, b).len();
            let estimate = estimate_json_bytes(&process);
            assert!(estimate >= real, "{a} x {b}: estimate {estimate} < real {real}");
            assert!(
                estimate <= real * 3 + 200,
                "{a} x {b}: estimate {estimate} too loose for {real}"
            );
        }
    }
}
