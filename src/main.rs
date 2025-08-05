#![feature(more_qualified_paths)]

use std::collections::HashMap;
use clap::Parser;
use std::fs::{self, write, File};
use std::hash::RandomState;
use std::io::stdout;
use futures::future::join_all;
use leetcode_fetcher::query::problem::ProblemInfo;
use leetcode_fetcher::query::solution::SolutionInfo;

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
			let slugs: Vec<String> = (|| {
				if let Some(file) = args.file {
					let file = fs::File::open(file).expect("Failed to open file");
					return serde_json::from_reader::<File, Vec<ProblemInfo>>(file)
						.expect("Failed to read file").into_iter().map(|problem_info| problem_info.title_slug).collect::<Vec<_>>();
				};
				if let Some(question_slug) = args.question_slug {
					return vec![question_slug.clone()];
				}
				vec![]
			})();

			let tasks = slugs.into_iter()
				.map(|question_slug| async {
					let solutions = leetcode_fetcher::query::solution::grab_solution_list(
						question_slug.clone(),
						crfs_token.clone(),
					)
						.await
						.expect("Failed to grab solutions");
					(question_slug, solutions)
				})
				.collect::<Vec<_>>();
			let results: Vec<(String, Vec<SolutionInfo>)> = join_all(tasks).await;
			let solutions: HashMap<_, _, RandomState> = HashMap::from_iter(results);
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
