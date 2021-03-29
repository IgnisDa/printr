use clap::{load_yaml, App};

pub fn get_program() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    println!("{:#?}", matches);
}
