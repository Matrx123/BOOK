use std::env;
use std::process;
use minigrep::{ Config, run };

fn main() {
   // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem {:?}", err);
        process::exit(1);
    });

    println!("Searching for {:?} in {:?}", config.query, config.filepath);

    if let Err(e) = run(config) {
        println!("Error occured {:?}", e);
        process::exit(1);
    }
}
