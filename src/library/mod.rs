use std::{fs::read_to_string, process};

fn run(printr: Printr) {}

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
    // the possible values are red (-1), blue (0), green (1), yellow, cyan, None
    color: Option<String>,
    // the final sentiment of the `string`
    sentiment: Sentiment,
    // whether to print to stderr
    error: bool,
}

impl Printr {
    pub fn new(
        interpretations: bool,
        newline: bool,
        plain: bool,
        spaces: bool,
        file: Option<String>,
        color: Option<String>,
        string: Option<String>,
        error: bool,
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
        let sentiment = Sentiment::new(string.clone());
        let new_color: Option<String>;
        if !plain && color.is_none() {
            // if the printing mode is not plain and no color is explicitly defined, we
            // perform sentiment analysis and determine the color of the output
            new_color = Some(determine_color(&sentiment));
        } else {
            // this means that user has defined a color mode and so we leave the option
            // alone
            new_color = color
        };
        Self {
            interpretations,
            newline,
            spaces,
            string,
            color: new_color,
            sentiment,
            error,
        }
    }
}

fn determine_color(sentiment: &Sentiment) -> String {
    let polarity = sentiment.clone().get_polarity();
    if polarity == 1 {
        "green".to_string()
    } else if polarity == -1 {
        "red".to_string()
    } else {
        "blue".to_string()
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
    fn new(string: String) -> Self {
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

mod tests;
