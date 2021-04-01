use cli::get_program;
use library::{run, Color, Format, Printr};

mod cli;
mod library;

fn main() {
    let matches = get_program();
    let string = matches
        .values_of("STRING")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<String>>());
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
        Some(c) => match c {
            "blue" => Some(Color::Blue),
            "red" => Some(Color::Red),
            "yellow" => Some(Color::Yellow),
            "green" => Some(Color::Green),
            "cyan" => Some(Color::Cyan),
            _ => None,
        },
        None => None,
    };
    let error = matches.is_present("error");
    let format = match matches.value_of("formatting") {
        Some(f) => match f {
            "bold" => Some(Format::Bold),
            "dimmed" => Some(Format::Dimmed),
            "underline" => Some(Format::Underline),
            "strikethrough" => Some(Format::Strikethrough),
            _ => None,
        },
        None => None,
    };
    let mut printr = Printr::new(
        interpretations,
        newline,
        plain,
        spaces,
        file,
        color,
        string,
        format,
    );
    run(&mut printr);
    match error {
        true => println!("{}", printr.get_output_string()),
        false => eprintln!("{}", printr.get_output_string()),
    }
}
