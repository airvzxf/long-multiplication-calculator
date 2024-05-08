use long_multiplication_command_line::generate;

fn main() {
    let multiplicand: usize = 2222;
    let multiplier: usize = 2222;

    let mut content = String::from("");
    // TODO: Refactor to only call one method to execute all the output.
    generate::symbols(&mut content);
    generate::top_border(multiplicand, multiplier, &mut content);
    generate::position_title(multiplicand, multiplier, &mut content);
    generate::operation_title(multiplicand, multiplier, &mut content);
    generate::multiplication(multiplicand, multiplier, &mut content);
    generate::operations(multiplicand, multiplier, &mut content);
    generate::sum_title(multiplicand, multiplier, &mut content);
    generate::long_sum(multiplicand, multiplier, &mut content);
    generate::product_validation(multiplicand, multiplier, &mut content);
    generate::bottom_border(multiplicand, multiplier, &mut content);
    generate::author(&mut content);
    println!("{content}");
}
