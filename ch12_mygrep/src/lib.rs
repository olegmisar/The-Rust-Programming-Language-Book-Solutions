use std::{env, error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    search(&config.query, &contents)
        .iter()
        .for_each(|s| println!("{}", s));

    Ok(())
}


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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    #[test]
    fn search_in_string() {
        assert_eq!(search("Who", POEM), vec!["I'm nobody! Who are you?"]);
        assert_eq!(
            search("nobody", POEM),
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"]
        );

        assert_eq!(
            search("NOboDy", POEM),
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            "Search is not case insensitive",
        );

        assert_eq!(search("NOT IN THE POEM", POEM), Vec::<&str>::new());

    }
}
