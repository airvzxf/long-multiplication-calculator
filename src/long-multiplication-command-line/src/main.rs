use long_multiplication_command_line::arguments::{Args, get_args};
use long_multiplication_command_line::multiplication;

fn main() {
    // TODO: #1 - The coefficients are strings.
    //       They need to convert to usize and multiply one number by one number.
    // TODO: #2 - Do I need to convert mutable variables to shadowing variables?
    let args: Args = get_args();
    let multiplicand: String = args.multiplicand;
    let multiplier: String = args.multiplier;

    multiplication::display(multiplicand, multiplier);
}
