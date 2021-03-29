use cli::get_program;
use printr::Arguments;
use std::env;
mod cli;
mod lib;

fn main() {
    get_program();
    let args: Vec<String> = env::args().collect();
    let args = Arguments::new(args);
    // println!("{:#?}", args);
}
