use printr::{app::app, run, Color, Format, Printr};

fn main() {
    let matches = app().get_matches();
    let string = matches
        .values_of("STRING")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<String>>());
    let newline = matches.is_present("newline");
    let spaces = matches.is_present("spaces");
    let disable_interpretation = matches.is_present("disable_interpretation");
    let enable_interpretation = matches.is_present("enable_interpretation");
    let interpretations = if disable_interpretation {
        false
    } else {
        enable_interpretation
    };
    let file = match matches.value_of("input-file") {
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
    // println!("{:#?}", &printr);
    match error {
        true => print!("{}", printr.get_output_string()),
        false => eprint!("{}", printr.get_output_string()),
    }
}
