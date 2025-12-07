use std::env;
use std::process;
use std::fs;
use std::error::Error;
use nattysgrep::{search, search_case_insensitive};

fn main() {
   let args: Vec<String> = env::args().collect();


   let config = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
       });

   println!("searching for {}", config.first);
   println!("in file {}", config.second);
   

   if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);

   }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.second)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.first, &contents)
    } else {
        search(&config.first, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())

}

pub struct Config {
    pub first: String,
    pub second: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let first = args[1].clone();
        let second = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            first, 
            second, 
            ignore_case, 
        })
    }
}
