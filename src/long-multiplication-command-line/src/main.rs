use long_multiplication_command_line::display;

fn main() {
    // let multiplicand: usize = 13;
    // let multiplier: usize = 26;
    let multiplicand: usize = 2222;
    let multiplier: usize = 2222;

    let mut content = String::from("");
    // TODO: Refactor to only call one method to execute all the output.
    display::symbols(&mut content);
    display::top_border(multiplicand, multiplier, &mut content);
    display::position_title(multiplicand, multiplier, &mut content);
    display::operation_title(multiplicand, multiplier, &mut content);
    display::multiplication(multiplicand, multiplier, &mut content);
    display::operations(multiplicand, multiplier, &mut content);
    display::sum_title(multiplicand, multiplier, &mut content);
    display::long_sum(multiplicand, multiplier, &mut content);
    display::product_validation(multiplicand, multiplier, &mut content);
    display::bottom_border(multiplicand, multiplier, &mut content);
    display::author(&mut content);
    println!("{content}");
}
