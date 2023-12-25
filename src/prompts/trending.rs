use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct GithubRepo {
    full_name: String,
    stargazers_count: u64,
    description: Option<String>,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct GithubSearchResult {
    items: Vec<GithubRepo>,
}

pub async fn show_trending() -> Result<(), Box<dyn Error>> {
    let url = "https://api.github.com/search/repositories?q=stars:>1&sort=stars&order=desc";

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "MyRustApp")
        .send()
        .await?;

    let search_result = resp.json::<GithubSearchResult>().await?;

    println!("Trending Repositories on GitHub:");
    for (i, repo) in search_result.items.iter().take(3).enumerate() {
        println!(
            "{}. {} - {} stars\n   {}\n   {}",
            i + 1,
            repo.full_name,
            repo.stargazers_count,
            repo.description.clone().unwrap_or_else(|| "No description provided.".to_string()),
            repo.html_url
        );
    }

    Ok(())
}
