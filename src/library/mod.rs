use std::{fs::read_to_string, process};

fn run() {}

#[derive(Debug, PartialEq)]
pub struct Printr {
    // if `-E` is supplied, this will be `false`
    interpretations: bool,
    // if `-n` is supplied, this will be `true`
    newline: bool,
    // if `-s` is supplied, this will become `true`
    spaces: bool,
    // the input `STRING`, if the `-f` is supplied, this will contain the contents of the file
    string: String,
    // the color of the output, will be automatically guessed from the context if not supplied
    // can be set to `None` for plain output
    color: Option<String>,
}

impl Printr {
    pub fn new(
        interpretations: bool,
        newline: bool,
        _plain: bool,
        spaces: bool,
        file: Option<String>,
        string: Option<String>,
        color: Option<String>,
    ) -> Self {
        let string = match file {
            Some(f) => {
                let contents = read_to_string(&f).unwrap_or_else(|err| {
                    eprintln!("Could not find file: {}", err);
                    process::exit(1);
                });
                contents
            }
            None => match string {
                Some(s) => s,
                None => String::new(),
            },
        };
        Self {
            interpretations,
            newline,
            spaces,
            string,
            color,
        }
    }
}

mod tests;
