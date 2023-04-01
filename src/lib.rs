use std::error::Error;
use std::fs;
use std::rc::Rc;

pub mod config;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut filter_builder = FilterStrategyBuilder::new(Some(Rc::new(|query, line| line.contains(query))));

    if let Some(v) = config.min_line_len {
        filter_builder.and(Rc::new(move |_, line| line.len() >= v.try_into().unwrap()));
    }

    if let Some(v) = config.max_line_len {
        filter_builder.and(Rc::new(move |_, line| line.len() >= v.try_into().unwrap()));
    }

    let filter = filter_builder.build();

    search(&config.query, &contents, Some(filter))
        .iter()
        .for_each(|line| println!("{line}"));

    Ok(())
}

// |query, line| -> filter_result
type FilterStrategy = dyn Fn(&str, &str) -> bool;
struct FilterStrategyBuilder {
    strategy: Rc<FilterStrategy>,
}

impl FilterStrategyBuilder {
    fn new(f: Option<Rc<FilterStrategy>>) -> FilterStrategyBuilder {
        FilterStrategyBuilder {
            strategy: f.unwrap_or(Rc::new(|_, _| true)),
        }
    }

    fn build(&self) -> Rc<FilterStrategy> {
        self.strategy.clone()
    }

    fn and(&mut self, f: Rc<FilterStrategy>) -> &mut FilterStrategyBuilder {
        let s = self.strategy.clone();
        self.strategy =
            Rc::new(move |query, line| (s(query, line) && f(query, line)));
        self
    }

    fn or(&mut self, f: Rc<FilterStrategy>) -> &mut FilterStrategyBuilder {
        let s = self.strategy.clone();
        self.strategy =
            Rc::new(move |query, line| (s(query, line) || f(query, line)));
        self
    }
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    filter_strategy: Option<Rc<FilterStrategy>>,
) -> Vec<&'a str> {
    // unwrap the optional filter_strategy and
    let filter_strategy =
        filter_strategy.unwrap_or(Rc::new(|q: &str, line: &str| line.contains(q)));

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

    #[test]
    fn build_and_filter() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
this line is over twenty characters long I think
Pick three.";

        let mut builder = FilterStrategyBuilder::new(None);

        builder.and(Rc::new(|_query, line| line.len() > 5));

        builder.and(Rc::new(|_query, line| line.len() < 25));

        let filter = builder.build();

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search(query, contents, Some(filter))
        );
    }

    #[test]
    fn build_or_filter() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
this line is over twenty characters long I think
Pick three.";
        let filter: Rc<FilterStrategy> = FilterStrategyBuilder::new(None)
            .and(Rc::new(|_query, line| line.len() > 5))
            .and(Rc::new(|_query, line| line.len() < 25))
            .or(Rc::new(|query, line| line.contains(query)))
            .build();

        assert_eq!(
            vec!["Rust:", "safe, fast, productive.", "Pick three."],
            search(query, contents, Some(filter))
        );
    }
}
