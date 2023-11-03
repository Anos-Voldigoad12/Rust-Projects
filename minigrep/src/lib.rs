use std::env;
use std::error::Error;
use std::fs; //File Stream
pub fn run(config: Config) -> Result< (), Box<dyn Error> >
{
	let file_content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.search_string, &file_content)
    } else {
        search_case_sensitive(&config.search_string, &file_content)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
pub struct Config
{
	pub search_string: String,
	pub file_path: String,
	pub ignore_case: bool,
}
impl Config
{
	pub fn build(args: &[String])-> Result<Config, &'static str>
	{
		if args.len()!=3
		{
			return Err("Invalid Usage!\nUsage: -- <query> <file_path>");
		}
		else
		{
			//args[0] : path of the binary 
			let search_string = args[1].clone();
			let file_path = args[2].clone();
			//Checks if the env var IGNORE_CASE is set to true
			let ignore_case = env::var("IGNORE_CASE").is_ok();
			
			Ok( Config{ search_string, file_path, ignore_case } )
		}
	}
}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() //The lines method returns an iterator.
    {
    	if line.contains(query)
    	{
    		results.push(line);
    	}
    }
    
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() //The lines method returns an iterator.
    {
    	if line.to_lowercase().contains(&query.to_lowercase())
    	{
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
