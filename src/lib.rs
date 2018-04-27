use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config, &'static str >{
        if args.len() < 3 {
            return Err("not enough ");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(),Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    // #[should_panic]
    fn test_search_case_insensitive(){
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}