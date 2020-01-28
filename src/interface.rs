use graphql_client::*;
use failure;
use reqwest;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/queries/get_list_query.graphql",
    response_derives = "Debug",
)]
pub struct UnionQuery;

pub fn perform_my_query(user: &str) -> Result<(), failure::Error> {

    // this is the important line
    let request_body = UnionQuery::build_query(union_query::Variables{
        user_name: Some(String::from(user)),
    });

    let client = reqwest::Client::new();
    let mut res = client.post("https://graphql.anilist.co").json(&request_body).send()?;
    let response_body: Response<union_query::ResponseData> = res.json()?;
    println!("{:#?}", response_body);
    Ok(())
}