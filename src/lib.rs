use ansi_term::Colour::{Blue, Cyan, Green, Red, Yellow};
use ansi_term::Style;
use std::{f32::EPSILON, fs::read_to_string, process};
// use regex::Regex;

pub fn run(printr: &mut Printr) {
    printr.run_all_handles()
}

#[derive(Debug, PartialEq)]
pub struct Config {
    // if `-E` is supplied, this will be `false`
    interpretations: bool,
    // if `-n` is supplied, this will be `true`
    newline: bool,
    // if `-s` is supplied, this will become `true`
    spaces: bool,
    // the color of the output, will be automatically guessed from the context if not supplied
    // can be set to `None` for plain output
    // the possible values are red (-1), blue (0), green (1), yellow, cyan, None
    color: Option<Color>,
    // the formatting to be applied to the output string
    format: Option<Format>,
    // whether the output should be completely plain
    plain: bool,
}

impl Config {
    pub fn new(
        interpretations: bool,
        newline: bool,
        spaces: bool,
        plain: bool,
        color: Option<Color>,
        format: Option<Format>,
    ) -> Self {
        Self {
            interpretations,
            newline,
            spaces,
            plain,
            color,
            format,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Printr {
    // the input `STRING`, if the `-f` is supplied, this will contain the contents of the file
    string: Vec<String>,
    // the final sentiment of the `string`
    sentiment: Option<Sentiment>,
    // the output string that will be displayed
    output_string: Option<String>,
    // configuration
    config: Config,
}

impl Printr {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interpretations: bool,
        newline: bool,
        plain: bool,
        spaces: bool,
        file: Option<String>,
        color: Option<Color>,
        string: Option<Vec<String>>,
        format: Option<Format>,
    ) -> Self {
        let string = match file {
            Some(f) => {
                let contents = read_to_string(&f).unwrap_or_else(|err| {
                    eprintln!("Could not find file: {}", err);
                    process::exit(1);
                });
                vec![contents]
            }
            None => match string {
                Some(s) => s,
                None => vec![String::new()],
            },
        };
        let config = Config::new(interpretations, newline, spaces, plain, color, format);
        Self {
            string,
            sentiment: None,
            config,
            output_string: None,
        }
    }
    // we perform sentiment analysis
    pub fn determine_sentiment(&mut self) {
        let sentiment = Sentiment::new(self.string.clone());
        self.sentiment = Some(sentiment);
    }
    // we handle the `-s` option here
    pub fn handle_spaces(&mut self) {
        if self.config.spaces {
            self.output_string = Some(self.string.join(""))
        } else {
            self.output_string = Some(self.string.join(" "))
        }
    }
    // we handle the `-e` and `-E` options here
    pub fn handle_interpretations(&mut self) {
        if self.config.interpretations {
            // let re = Regex::new(r"\\").unwrap();
            // let new_str = self.output_string.clone().unwrap();
            // let new_str = re.replace_all(&new_str, r"\").to_string();
            // self.output_string = Some(new_str);
        }
    }
    // we determine the color that should be applied to the output
    pub fn determine_color(&mut self) {
        if !self.config.plain && self.config.color.is_none() {
            self.config.color = Some(determine_color(&self.sentiment.clone().unwrap()));
        }
    }
    // we handle the `-c` option here
    pub fn handle_coloring(&mut self) {
        self.output_string = match self.config.color {
            Some(Color::Blue) => Some(Blue.paint(self.output_string.clone().unwrap()).to_string()),
            Some(Color::Red) => Some(Red.paint(self.output_string.clone().unwrap()).to_string()),
            Some(Color::Green) => {
                Some(Green.paint(self.output_string.clone().unwrap()).to_string())
            }
            Some(Color::Yellow) => Some(
                Yellow
                    .paint(self.output_string.clone().unwrap())
                    .to_string(),
            ),
            Some(Color::Cyan) => Some(Cyan.paint(self.output_string.clone().unwrap()).to_string()),
            None => self.output_string.clone(),
        };
    }
    // we handle the `-f` option here
    pub fn handle_formatting(&mut self) {
        self.output_string = match self.config.format {
            Some(Format::Bold) => Some(
                Style::new()
                    .bold()
                    .paint(self.output_string.clone().unwrap())
                    .to_string(),
            ),
            Some(Format::Underline) => Some(
                Style::new()
                    .underline()
                    .paint(self.output_string.clone().unwrap())
                    .to_string(),
            ),
            Some(Format::Dimmed) => Some(
                Style::new()
                    .dimmed()
                    .paint(self.output_string.clone().unwrap())
                    .to_string(),
            ),
            Some(Format::Strikethrough) => Some(
                Style::new()
                    .strikethrough()
                    .paint(self.output_string.clone().unwrap())
                    .to_string(),
            ),
            None => self.output_string.clone(),
        }
    }
    // we handle the `-n` option here
    pub fn handle_newline(&mut self) {
        if !self.config.newline {
            self.output_string = Some(format!("{}\n", self.output_string.clone().unwrap()));
        }
    }
    pub fn run_all_handles(&mut self) {
        self.determine_sentiment();
        self.handle_spaces();
        self.handle_interpretations();
        self.determine_color();
        self.handle_coloring();
        self.handle_newline();
        self.handle_formatting();
    }
    pub fn get_output_string(self) -> String {
        match self.output_string {
            Some(s) => s,
            None => "".to_string(),
        }
    }
}

fn determine_color(sentiment: &Sentiment) -> Color {
    let polarity = sentiment.clone().get_polarity();
    if polarity == 1 {
        Color::Green
    } else if polarity == -1 {
        Color::Red
    } else {
        Color::Blue
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Sentiment(
    // the positivity score of the sentence being analysed
    f32,
    // the negativity score of the sentence being analysed
    f32,
);

impl Sentiment {
    fn new(string: Vec<String>) -> Self {
        let string = string.join(" ");
        let analyser = sentiment::analyze(string);
        Self(analyser.positive.score, analyser.negative.score)
    }
    fn get_polarity(self) -> i8 {
        if (self.0 - self.1).abs() < EPSILON {
            0
        } else if self.0 > self.1 {
            1
        } else {
            -1
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Cyan,
}

#[derive(Debug, PartialEq)]
pub enum Format {
    Bold,
    Underline,
    Strikethrough,
    Dimmed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, io::Write};
    use tempfile::NamedTempFile;

    // we keep using the word "zealous" here since we know it's
    // [afinn](https://github.com/fnielsen/afinn) score is 2.0 and can be used consistently for
    // tests

    #[test]
    fn test_new_being_created_with_filename_supplied() -> Result<(), Box<dyn Error>> {
        let mut file = NamedTempFile::new()?;
        let content = "zealous";
        write!(file, "{}", content)?;
        let mut printr = Printr::new(
            true,
            true,
            false,
            false,
            Some(file.path().to_str().unwrap().to_string()),
            None,
            None,
            None,
        );
        printr.run_all_handles();
        assert_eq!(
            printr,
            Printr {
                string: vec![content.to_string()],
                sentiment: Some(Sentiment(2.0, 0.0)),
                output_string: Some("\u{1b}[32mzealous\u{1b}[0m".to_string()),
                config: Config {
                    color: Some(Color::Green),
                    format: None,
                    interpretations: true,
                    newline: true,
                    plain: false,
                    spaces: false
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_new_being_created_with_input_string_supplied() -> Result<(), Box<dyn Error>> {
        let content = String::from("zealous");
        let mut printr = Printr::new(
            true,
            true,
            false,
            false,
            None,
            None,
            Some(vec![content.clone()]),
            Some(Format::Bold),
        );
        printr.run_all_handles();
        assert_eq!(
            printr,
            Printr {
                string: vec![content.clone()],
                sentiment: Some(Sentiment(2.0, 0.0)),
                output_string: Some("\u{1b}[1m\u{1b}[32mzealous\u{1b}[0m\u{1b}[0m".to_string()),
                config: Config {
                    interpretations: true,
                    newline: true,
                    color: Some(Color::Green),
                    spaces: false,
                    plain: false,
                    format: Some(Format::Bold),
                }
            }
        );
        Ok(())
    }
}
