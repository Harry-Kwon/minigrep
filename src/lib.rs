use std::error::Error;
use std::fs;
use std::iter::Filter;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    search(&config.query, &contents, None)
        .iter()
        .for_each(|line| println!("{line}"));

    Ok(())
}

type FilterStrategy = dyn Fn(&str, &str) -> bool;
struct FilterStrategyBuilder {
    strategy: Box<FilterStrategy>
}

impl FilterStrategyBuilder {
    fn new(f: Option<Box<FilterStrategy>>) -> FilterStrategyBuilder{
        FilterStrategyBuilder { strategy: f.unwrap_or(Box::new(|_, _| true)) }
    }

    fn build(&self) -> Box<FilterStrategy> {
        self.strategy
    }

    fn and(&self, f: Box<FilterStrategy>) -> &FilterStrategyBuilder{
        self.strategy = Box::new(|query, line| ((self.strategy)(query, line) && f(query, line)));
        self
    }

    fn or(&self, f: Box<FilterStrategy>) -> &FilterStrategyBuilder {
        self.strategy = Box::new(|query, line| ((self.strategy)(query, line) || f(query, line)));
        self
    }
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    filter_strategy: Option<Box<FilterStrategy>>,
) -> Vec<&'a str> {
    // unwrap the optional filter_strategy and
    let filter_strategy = filter_strategy.unwrap_or(Box::new(|q: &str, line: &str| line.contains(q)));

    contents
        .split("\n")
        .filter(|line| filter_strategy(query, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, None)
        );
    }

    fn build_filter() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
this line is over twenty characters long I think
Pick three.";
        let filter: Box<FilterStrategy> = FilterStrategyBuilder::new(None)
            .and(Box::new(|_query, line| line.len() > 5))
            .and(Box::new(|_query, line| line.len() < 20))
            .build();

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search(query, contents, Some(filter))
        );
    }
}
