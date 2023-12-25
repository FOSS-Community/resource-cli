use super::data::{read_data, write_data};
use reqwest;
use std::error::Error;

#[derive(serde::Deserialize, Debug)]
struct GithubRepo {
    full_name: String,
    stargazers_count: u64,
    description: String,
}

pub async fn update_data_with_github_info() -> Result<(), Box<dyn Error>> {
    let mut resources = read_data()?;

    let client = reqwest::Client::new();

    for resource in &mut resources {
        if let Some(api_url) = convert_to_api_url(&resource.link) {
            if let Ok(repo_info) = fetch_github_repo_info(&client, &api_url).await {
                resource.name = repo_info.full_name;
                resource.description = repo_info.description;
                resource.github_stars = repo_info.stargazers_count;
            }
        }
    }

    write_data(&resources)?;
    Ok(())
}

async fn fetch_github_repo_info(client: &reqwest::Client, api_url: &str) -> Result<GithubRepo, Box<dyn Error>> {
    let resp = client.get(api_url)
        .header("User-Agent", "MyRustApp") 
        .send()
        .await?;

    resp.json::<GithubRepo>().await.map_err(Into::into)
}

fn convert_to_api_url(github_url: &str) -> Option<String> {
    let parts: Vec<&str> = github_url.split('/').collect();
    if parts.len() >= 2 {
        let owner = parts[parts.len() - 2];
        let repo = parts[parts.len() - 1];
        Some(format!("https://api.github.com/repos/{}/{}", owner, repo))
    } else {
        None
    }
}
