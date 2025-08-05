use std::path::PathBuf;
use clap::ArgGroup;
use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]

pub enum Commands {
	FetchQuestionList(FetchQuestionListArgs),
}

#[derive(Debug, Parser)]
pub struct CliArgs {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Args, Debug, Clone)]
pub struct FetchQuestionListArgs {
	#[arg(short, long, default_value = None)]
	pub outpath: Option<PathBuf>,
}
