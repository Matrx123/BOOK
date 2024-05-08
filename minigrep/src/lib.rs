use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments!!"));
        }
        let ignore_case = env::var("IC").is_ok();
        return Ok(Config {
            query: args[1].clone(),
            filepath: args[2].clone(),
            ignore_case,
        });
    }
    // enhanced
    pub fn build(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
 
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't received the file path"),
        };

        let ignore_case = env::var("IC").is_ok();
        Ok( Config{ query, filepath, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    let lines = if config.ignore_case {
        search_case_insensitive(&content, &config.query)
    } else {
        search(&content, &config.query)
    };

    println!("\nFound {:?} result(s)\n", lines.len());
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(content: &'a str, query: &'a str) -> Vec<&'a str> {
    // let mut collection = Vec::new();
    // let lines = content.lines();

    // for line in lines {
    //     if line.trim().contains(query) {
    //         collection.push(line.trim());
    //     }
    // }
    // collection
    content.lines().filter(|line| line.trim().contains(query)).collect()
}

pub fn search_case_insensitive<'a>(content: &'a str, query: &'a str) -> Vec<&'a str> {
    // let mut collection = Vec::new();
    // let lines = content.lines();

    // for line in lines {
    //     if line.trim().to_uppercase().contains(&query.to_uppercase()) {
    //         collection.push(line.trim());
    //     }
    // }
    // collection
    content.lines().filter(|line| line.trim().to_uppercase().contains(&query.to_uppercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:\n safe, fast, productive \n Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(content, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let content = "Rust:\n safe, fast, productive \n Pick three.\nTrUsT me.";
        assert_eq!(vec!["Rust:", "TrUsT me."], search_case_insensitive(content, query));
    }
}
