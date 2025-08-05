#![feature(more_qualified_paths)]
use clap::Parser;
use std::fs::{self, write};
use std::io::stdout;

mod lib;
mod cliargs;


#[tokio::main]
async fn main() {
	let args = cliargs::CliArgs::parse();
	match args.command {
		cliargs::Commands::FetchQuestionList(args) => {
			let questions = lib::grab_questions().await.expect("Failed to grab questions");
			match args.outpath {
				Some(outpath) => {
					let file = fs::File::create_new(outpath).expect("Failed to create file");
					serde_json::to_writer_pretty(file, &questions).expect("Failed to write to file");
				}
				_ => {
					serde_json::to_writer_pretty(stdout(), &questions).expect("Failed to write to file");
				}
			}
		}
		_ => unimplemented!()
	}
}
