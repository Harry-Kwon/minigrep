use std::vec;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub min_line_len: Option<u64>,
    pub max_line_len: Option<u64>,
}

impl Config {
    // rough implementation of a arg parser
    // just use a library next time
    pub fn build(args: &[String]) -> Result<Config, &str> {
        let mut min_line_len: Option<u64> = None;
        let mut max_line_len: Option<u64> = None;

        let mut positional_args: Vec<&String> = vec![];
        let mut ignore_options = false;
        let mut i = 0;
        while i + 1 < args.len() {
            // get the next argument
            i += 1;
            let arg = &args[i];

            if arg == "--" {
                // set a flag to not parse optional arguments
                ignore_options = true;
                continue;
            }
            if arg.starts_with("--") && !ignore_options {
                // parse optional arguments
                match &arg[2..arg.len()] {
                    "min" => {
                        i += 1; // step forward to param value
                        if i >= args.len() {
                            return Err("--min must be followed by a non-negative integer");
                        }
                        let parse_result = args[i].parse::<u64>();
                        match parse_result {
                            Ok(value) => min_line_len = Some(value),
                            Err(_) => return Err("min line length must be a non-negative integer"),
                        }
                    }
                    "max" => {
                        i += 1; // step forward to param value
                        if i >= args.len() {
                            return Err("--min must be followed by a non-negative integer");
                        }
                        let parse_result = args[i].parse::<u64>();
                        match parse_result {
                            Ok(value) => max_line_len = Some(value),
                            Err(_) => return Err("max line length must be a non-negative integer"),
                        }
                    }
                    _ => (),
                }
            } else {
                //parse positional arguments
                positional_args.push(arg);
            }
        }

        // return Err if not enough positional arguments provided
        if positional_args.len() < 2 {
            return Err("not enough arguments");
        }

        return Ok(Config {
            query: positional_args[0].clone(),
            file_path: positional_args[1].clone(),
            min_line_len,
            max_line_len,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_optional_params() {
        let args: Vec<String> = vec!["minigrep", "query", "file_path"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(None, config.min_line_len);
        assert_eq!(None, config.max_line_len);
    }

    #[test]
    fn parse_one_optional_params() {
        let args: Vec<String> = vec!["minigrep", "--min", "5", "query", "file_path"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(Some(5), config.min_line_len);
        assert_eq!(None, config.max_line_len);
    }

    #[test]
    fn parse_multiple_optional_params() {
        let args: Vec<String> = vec![
            "minigrep",
            "--min",
            "5",
            "--max",
            "10",
            "query",
            "file_path",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(Some(5), config.min_line_len);
        assert_eq!(Some(10), config.max_line_len);
    }

    #[test]
    fn parse_optional_params_after_positional() {
        let args: Vec<String> = vec![
            "minigrep",
            "query",
            "file_path",
            "--min",
            "5",
            "--max",
            "10",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(Some(5), config.min_line_len);
        assert_eq!(Some(10), config.max_line_len);
    }

    #[test]
    fn parse_optional_params_interleaved_between_positional() {
        let args: Vec<String> = vec![
            "minigrep",
            "query",
            "--min",
            "5",
            "file_path",
            "--max",
            "10",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(Some(5), config.min_line_len);
        assert_eq!(Some(10), config.max_line_len);
    }

    #[test]
    fn parse_optional_params_interleaved_between_positional_first_before() {
        let args: Vec<String> = vec![
            "minigrep",
            "--min",
            "5",
            "query",
            "--max",
            "10",
            "file_path",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let config = Config::build(&args).unwrap();

        assert_eq!("query", config.query);
        assert_eq!("file_path", config.file_path);
        assert_eq!(Some(5), config.min_line_len);
        assert_eq!(Some(10), config.max_line_len);
    }
}
