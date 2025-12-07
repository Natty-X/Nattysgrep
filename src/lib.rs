#![doc = include_str!("../README.md")]

pub fn search<'a>(first: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(first) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(
    first: &str, 
    contents: &'a str,
) -> Vec<&'a str> {
    let first = first.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&first) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let first = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(first, contents));
    }

    #[test]
    fn case_insensitive() {
        let first = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(first, contents)
        );
    }
}
