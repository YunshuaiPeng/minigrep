use std::error::Error;
use std::fs;
use std::env::Args;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for result in search(&config.query, &contents) {
        println!("{}", result);
    }
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("参数不够");
        // }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少参数 query"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少参数 filename"),
        };

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| { line.contains(query) }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
