use crate::query::solution::solution_list::ArticleOrderByEnum;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "leetcode-graphql/schema.graphqls",
	query_path = "leetcode-graphql/solution.graphql",
	response_derives = "Debug"
)]
struct SolutionList;


#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionInfo {
	pub question_slug: String,
	pub uuid: String,
	pub title: String,
	pub slug: String,
	pub tags: Vec<String>,
	pub is_official: bool
}

pub async fn grab_solution_list(question_slug: String, crsf_token: String) -> Result<Vec<SolutionInfo>, Box<dyn Error>> {
	let variables = <SolutionList as GraphQLQuery>::Variables
	{
		question_slug: question_slug.clone(),
		skip: Some(
		0),
		first: Some(
		15),
		order_by: Some(ArticleOrderByEnum::HOT),
		user_input: None,
		tag_slugs: Some(vec![]),
		after: None,
		before: None,
		is_mine: None,
		last: None
	};

	let request_body = SolutionList::build_query(variables);

	let client = reqwest::Client::new();
	let res = client.post("https://leetcode.com/graphql")
		.header("X-CSRFToken", crsf_token)
		.json(&request_body).send().await?;
	let response_body: Response<<SolutionList as graphql_client::GraphQLQuery>::ResponseData> = res.json().await?;


	let response = response_body.data.ok_or("No data")?.ugc_article_solution_articles.ok_or("No questions")?;

	let solutions = response.edges.ok_or("No solutions")?
		.into_iter()
		.map(|s| -> Result<SolutionInfo, Box<dyn Error>>
					{
						let s = s.ok_or("Failed to get solution")?;
						let node = s.node.ok_or("No node")?;
						let tags = node.tags.ok_or("No tags")?.into_iter()
							.filter_map(|t| match t.name {
								Some(name) => Some(name),
								None => None,
							})
							.collect::<Vec<_>>();
						Ok(SolutionInfo {
							question_slug: question_slug.clone(),
							uuid: node.uuid.ok_or("No uuid")?,
							title: node.title.ok_or("No title")?,
							slug: node.slug.ok_or("No slug")?,
							tags,
							is_official: node.is_leetcode.ok_or("No is official")?,
						})
					},
		).collect::<Result<Vec<_>, _>>();
	solutions
}


#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_grab_solutions() {
		let solutions = grab_solution_list("two-sum".to_string(), "8vcQtTF9ZOHgg7XbKNsmYnjEPSvHTwElFtT3I2X3kacli5N7ZUKd5W339GBU7pAV".to_string()).await.expect("Failed to grab solutions");
		println!("{:?}", solutions);
	}
}
