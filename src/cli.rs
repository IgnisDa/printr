use clap::{load_yaml, App, ArgMatches};

pub fn get_program() -> ArgMatches {
    let yaml = load_yaml!("cli.yaml");
    App::from(yaml).get_matches()
}
