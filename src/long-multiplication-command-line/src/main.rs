use long_multiplication_command_line::arguments::{Args, get_args};
use long_multiplication_command_line::multiplication;

fn main() {
    // TODO: #1 - Do I need to convert mutable variables to shadowing variables?
    let args: Args = get_args();
    let multiplicand: String = args.multiplicand;
    let multiplier: String = args.multiplier;
    let output: String = args.output;
    let content: String = multiplication::get_table(&multiplicand, &multiplier);

    if output == "display" || output == "both" {
        multiplication::display(&content);
    }

    if output == "store" || output == "both" {
        let file_path: String = args.file;
        multiplication::store(&content, &file_path);
    }
}
