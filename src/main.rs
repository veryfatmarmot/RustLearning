use std::{env, error::Error};
use minigrep::Config;


fn main() -> Result<(), Box<dyn Error>> {
    //Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    minigrep::run(config)
}
