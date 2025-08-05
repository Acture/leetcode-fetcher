use std::error::Error;
use graphql_client::{GraphQLQuery, Response};
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "leetcode-graphql/schema.graphqls",
	query_path = "leetcode-graphql/problem.graphql",
	response_derives = "Debug"
)]
struct ProblemList;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemInfo {
	pub frontend_question_id: usize,
	pub title: String,
	pub title_slug: String,
	pub difficulty: String,
}

pub async fn grab_problem_list() -> Result<Vec<ProblemInfo>, Box<dyn Error>> {
	let variables = <ProblemList as GraphQLQuery>::Variables {
		category_slug: Some("all-code-essentials".to_string()),
		skip: Some(0),
		limit: Some(100),
		filters: None,
		search_keyword: None,
		sort_by: None,
	};

	let request_body = ProblemList::build_query(variables);

	let client = reqwest::Client::new();
	let res = client.post("https://leetcode.com/graphql").json(&request_body).send().await?;
	let response_body: Response<<ProblemList as graphql_client::GraphQLQuery>::ResponseData> = res.json().await?;

	let questions = response_body.data.ok_or("No data")?.problemset_question_list_v2.ok_or("No questions")?;
	questions.questions.into_iter().map(
		|q| -> Result<ProblemInfo, Box<dyn Error>> {
			Ok(ProblemInfo {
				title: q.title.ok_or("No title")?,
				frontend_question_id: q.question_frontend_id.ok_or("No question id")?.parse::<usize>()?,
				title_slug: q.title_slug.ok_or("No title slug")?,
				difficulty: q.difficulty.ok_or("No difficulty")?,
			})
		}
	).collect::<Result<Vec<ProblemInfo>, _>>()
}


#[cfg(test)]
mod tests {
	use std::result;
	use super::*;
	#[tokio::test]
	async fn test_grab_questions() {
		let result = grab_problem_list().await.expect("Failed to grab questions");
		println!("{:?}", result);
	}
}
