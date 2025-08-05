#![feature(more_qualified_paths)]

pub mod query;


#[cfg(test)]
mod tests {
	use std::result;
	use super::*;
	#[tokio::test]
	async fn test_grab_questions() {
		let result = query::problem::grab_problem_list().await.expect("Failed to grab questions");
		println!("{:?}", result);
	}
}
