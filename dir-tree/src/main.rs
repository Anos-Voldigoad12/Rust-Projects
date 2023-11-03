use std::env;
use std::process::{self,Command};
fn main() {
	let args: Vec<String> = env::args().collect();
	let dir;
	if args.len()==1
	{
		dir = "./";
	}
	else if args.len()==2
	{
		dir = &args[1];
	}
	else
	{
		help();
		process::exit(1);
	}
	list_dir(dir.trim_end_matches("/").to_string().clone(),"|");
}
fn list_dir(dir: String, pre: &str)
{
	if dir.ends_with("/")
	{
		return;
	}
	
	let dir_list = Command::new("ls").arg(dir.to_owned()+"/").output()
							.unwrap_or_else(|err|
								{
									panic!("Unable to ls: {err}");
								}
							);						
	let parts: Vec<&str> = dir.trim().split("/").collect();
	print!("{}-{}",pre,parts.last().expect("!!"));
	
	if dir_list.status.code()==Some(0)
	{
		println!("/");
	}
	else
	{
		println!();
	}
	
	let list = &dir_list.stdout;
	let list = match String::from_utf8(list.to_vec())
						{
							Ok(s) => s,
							Err(e) => panic!("Found Error: {e}"),
						};
	let parts: Vec<&str> = list.trim().split("\n").collect();
	for part in parts
	{
		list_dir( (dir.clone()+"/"+part).clone()
				, &(pre.to_owned()+"  |") );
	}
}
fn help()
{
	println!("\nInvalid Usage!\n");
	println!("Usage: 1. To view dir tree of current directory: ./dir-tree");
	println!("Usage: 2. To view dir tree of specific directory: ./dir-tree <dir>");
	println!("\n");
}
