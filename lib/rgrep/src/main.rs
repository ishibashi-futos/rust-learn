extern crate rgrep;

use std::env;
use std::process;

use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(-1);
    });

    println!("Searching for {} in {}", config.query, config.filename);

    // Errorを返した場合はIfステート内の処理を行う
    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
