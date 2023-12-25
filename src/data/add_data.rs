use crate::data::data::{read_data, write_data, Resource};
use reqwest;
use serde::Deserialize;
use std::error::Error;
use chrono::Utc;
use dialoguer::Input;

#[derive(Deserialize, Debug)]
struct GithubRepo {
    full_name: String,
    stargazers_count: u64,
    description: String,
    topics: Vec<String>,
}

pub async fn add_project(username_repo: &str) -> Result<(), Box<dyn Error>> {
    let api_url = convert_to_api_url(username_repo);
    let client = reqwest::Client::new();

    let resp = client.get(&api_url)
        .header("User-Agent", "MyRustApp")  
        .send()
        .await?;

    if resp.status().is_success() {
        let repo_info = resp.json::<GithubRepo>().await?;
        let mut resources = read_data()?;

        let rating: f64 = Input::new()
            .with_prompt("Rate this project (0-10)")
            .interact_text()?;

        resources.push(Resource {
            name: repo_info.full_name,
            description: repo_info.description,
            link: format!("https://github.com/{}", username_repo),
            github_stars: repo_info.stargazers_count,
            community_ratings: rating,
            date: Utc::now().format("%Y-%m-%d").to_string(),
            keywords: repo_info.topics,
        });

        write_data(&resources)?;
        println!("Project added successfully!");
    } else {
        println!("Failed to fetch repository data. It might be private or does not exist.");
    }

    Ok(())
}

fn convert_to_api_url(username_repo: &str) -> String {
    format!("https://api.github.com/repos/{}", username_repo)
}
