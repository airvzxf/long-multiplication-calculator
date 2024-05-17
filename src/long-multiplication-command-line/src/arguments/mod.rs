use clap::{Arg, ArgMatches, command};

pub struct Args {
    // The first coefficient of the multiplication.
    pub multiplicand: String,

    // The second coefficient of the multiplication.
    pub multiplier: String,

    // The output method.
    pub output: String,
}

pub fn get_args() -> Args {
    let matches: ArgMatches = command!()
        .about("\
            Create a table with the long-multiplication method given two values: \
            the multiplicand and the multiplier."
        )
        .arg(
            Arg::new("multiplicand")
                .required(true)
                .help("The first coefficient of the multiplication.")
        )
        .arg(
            Arg::new("multiplier")
                .required(true)
                .help("The second coefficient of the multiplication.")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(false)
                .default_value("display")
                .help("The options are: 'display', 'store' or 'both'.")
        )
        .get_matches();

    let multiplicand: String = unwrap_args(&matches, "multiplicand", false);
    let multiplier: String = unwrap_args(&matches, "multiplier", false);
    let output: String = unwrap_args(&matches, "output", true);

    return Args { multiplicand, multiplier, output };
}

fn unwrap_args(matches: &ArgMatches, id: &str, lowercase: bool) -> String {
    let value: String = matches.get_one::<String>(id).unwrap().to_string();

    if lowercase {
        return value.to_lowercase();
    }

    return value;
}
