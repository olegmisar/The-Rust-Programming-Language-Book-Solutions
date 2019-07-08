use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = args.next().ok_or_else(|| "Didn't get a query string")?;
        let filename = args.next().ok_or_else(|| "Didn't get a filename")?;

        Ok(Config { query, filename })
    }
}

pub fn run() {}
