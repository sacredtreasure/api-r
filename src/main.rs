use std::fmt::format;

use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "ankitects",
        repo = "anki"
    );
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "api-r for repo-stargazers")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("\n{:#?}", users);
    Ok(())
}
