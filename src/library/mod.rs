use ansi_term::Colour::{Blue, Cyan, Green, Red, Yellow};
use ansi_term::Style;
use std::{fs::read_to_string, process};

pub fn run(printr: &mut Printr) {
    printr.determine_sentiment();
    printr.handle_spaces();
    printr.handle_interpretations();
    printr.determine_color();
    printr.handle_coloring();
    printr.handle_formatting();
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
        // we leave this function empty for now
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
    pub fn get_output_string(self) -> String {
        match self.output_string {
            Some(s) => return s.to_owned(),
            None => return "".to_string().to_owned(),
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
        if self.0 == self.1 {
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
mod tests;
