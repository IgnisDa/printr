use cli::get_program;
use library::Printr;

mod cli;
mod library;

fn main() {
    let matches = get_program();
    let string: Option<String> = match matches.values_of("STRING") {
        Some(values) => {
            let mut temp: Vec<String> = vec![];
            for val in values {
                temp.push(val.to_string())
            }
            Some(temp.join(" "))
        }
        None => None,
    };
    let newline = matches.is_present("newline");
    let spaces = matches.is_present("spaces");
    let disable_interpretation = matches.is_present("disable_interpretation");
    let enable_interpretation = matches.is_present("enable_interpretation");
    let interpretations = if disable_interpretation {
        false
    } else if enable_interpretation {
        true
    } else {
        false
    };
    let file = match matches.value_of("file") {
        Some(f) => Some(f.to_string()),
        None => None,
    };
    let plain = matches.is_present("plain");
    let color = match matches.value_of("color") {
        Some(c) => Some(c.to_string()),
        None => None,
    };
    let error = matches.is_present("error");
    let printr = Printr::new(
        interpretations,
        newline,
        plain,
        spaces,
        file,
        color,
        string,
        error,
    );
    println!("{:#?}", printr);
}
