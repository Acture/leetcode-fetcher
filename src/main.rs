#![feature(more_qualified_paths)]
use std::error::Error;
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "leetcode-graphql/schema.graphqls",
	query_path = "leetcode-graphql/question_list.graphql",
	response_derives = "Debug"
)]
struct ProblemList;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
	let mut res = client.post("https://leetcode.com/graphql").json(&request_body).send().await?;
	let response_body: Response<<ProblemList as graphql_client::GraphQLQuery>::ResponseData> = res.json().await?;
	println!("{:#?}", response_body);

	Ok(())
}
