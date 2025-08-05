#![feature(more_qualified_paths)]
mod lib;


#[tokio::main]
async fn main() {
	lib::grab_questions().await.expect("Failed to grab questions");
}
