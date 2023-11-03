use std::process;
use std::env;
use minigrep::Config;
fn main() {
	let args: Vec<String> = env::args().collect();
	
	//env::args() creates an iterator for the cmd line arguments,
	//collect() collects the elemets of an itertor into a
	//collection
	let config = Config::build(&args).unwrap_or_else(|err|
			{
				//For printing to Err stream
				eprintln!("\nProblem parsing arguments: {err}");
				//For exiting the current process
        		process::exit(1);
			}
		);	
	//unwrap_or_else(): Tries to Unwrap first but if it finds an error
	//					executes the statements in the function parameter
	// |err| : is a clojure 
	
    println!("\nSearching for \"{}\" in {}...",config.search_string,config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}

