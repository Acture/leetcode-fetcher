#![feature(more_qualified_paths)]
use clap::Parser;
use std::fs::{self, write};
use std::io::stdout;

mod cliargs;


#[tokio::main]
async fn main() {
	let args = cliargs::CliArgs::parse();
	match args.command {
		cliargs::Commands::FetchQuestionList(args) => {
			let questions = leetcode_fetcher::query::problem::grab_problem_list().await.expect("Failed to grab questions");
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
		cliargs::Commands::FetchSolutionList(args) => {
			let crfs_token = args.crfs_token
				.or_else(|| std::env::var("LEETCODE_CSRF").ok()) // 从环境变量 LEETCODE_CSRF 获取值
				.expect("crfs_token is required (either pass as an argument or set LEETCODE_CSRF in environment)");
			let solutions = leetcode_fetcher::query::solution::grab_solution_list(args.question_slug, crfs_token).await.expect("Failed to grab solutions");

			match args.outpath {
				Some(outpath) => {
					let file = fs::File::create_new(outpath).expect("Failed to create file");
					serde_json::to_writer_pretty(file, &solutions).expect("Failed to write to file");
				}
				_ => {
					serde_json::to_writer_pretty(stdout(), &solutions).expect("Failed to write to file");
				}
			}
		}
		_ => unimplemented!()
	}
}
