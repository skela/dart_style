use std::path::PathBuf;

use clap::Parser;

mod config;
mod formatter;

fn main()
{
	let args = Arguments::parse();	
	let config = config::load(&args.path.as_path());	
	
	if args.path.is_dir()
	{
		let res = std::fs::read_dir(&args.path);
		match res
		{
			Ok(paths) =>
			{
				for pr in paths
				{
					match pr
					{
						Ok(entry) =>
						{							
							format_file(args.verbose,config,&entry.path());
						}
						Err(err) =>
						{
							println!("Failed to check item in folder - {}",err);
						}
					}
				}
			}
			Err(error) =>
			{
				println!("Failed to list contents of folder at path `{}`\nReason: {}",args.path.display(),error);
			}
		}
	}
	else
	{
		format_file(args.verbose,config,&args.path)
	}
}

fn format_file(verbose:bool,config:config::Config,path:&PathBuf)
{
	let res = std::fs::read_to_string(path);

	match res
	{
		Ok(content) => 
		{
			let formatter = formatter::Formatter { verbose:verbose, config, };
			formatter.format(content);
		}

		Err(error) => 
		{
			println!("Error: Unable to read file `{}`\nReason: {}",path.display(),error);
		}
	}
}


#[derive(Parser)]
#[clap(version, about, long_about = None)]
/// A blazing fast code formatter for Dart
struct Arguments
{
	#[clap(short,long)]
	/// Output more detailed extra information
	verbose: bool,

	#[clap(parse(from_os_str))]
	/// Path to input file
	path: std::path::PathBuf,

	#[clap(short,long,parse(from_os_str))]
	/// Path to output folder
	output: std::path::PathBuf,
}
