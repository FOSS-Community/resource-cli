use crate::data::data::{read_data, Resource};
use colored::Colorize;
use dialoguer::Input;
use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct GithubRepoSearchResult {
    items: Vec<GithubRepo>,
}

#[derive(Deserialize, Debug)]
struct GithubRepo {
    full_name: String,
    description: Option<String>
}

pub async fn perform_search() -> Result<(), Box<dyn Error>> {
    let search_query: String = Input::new()
        .with_prompt("Enter search term")
        .interact_text()?;

    println!("Searching in local data...");
    let local_resources = read_data()?;
    let local_results: Vec<&Resource> = local_resources
        .iter()
        .filter(|res| {
            res.name.contains(&search_query)
                || res.description.contains(&search_query)
                || res.keywords.iter().any(|k| k.contains(&search_query))
        })
        .collect();

    if !local_results.is_empty() {
        println!("Local results found:");
        for res in &local_results {
            println!("{} - {}", res.name, res.description);
        }
    } else {
        println!("No local results found. Searching on GitHub...");
        search_github(&search_query).await?;
    }
    println!("{}", "Search Results:".underline().cyan());
    for res in local_results {
        println!("{}", "Name:".green().bold());
        println!("{}", res.name);
        println!("{}", "Description:".green().bold());
        println!("{}", res.description);
        println!("{}", "Repo Link:".green().bold());
        println!("{}", res.link);
        println!("{}", "GitHub Stars:".green().bold());
        println!("{}", res.github_stars);
        println!("{}", "Community Ratings:".green().bold());
        println!("{}", res.community_ratings);
        println!("-----------------------------");
    }

    Ok(())
}

async fn search_github(query: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/search/repositories?q={}", query);
    let resp = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await?;

    let search_result = resp.json::<GithubRepoSearchResult>().await?;

    if !search_result.items.is_empty() {
        println!("GitHub results found:");
        for repo in search_result.items {
            println!(
                "{} - {}",
                repo.full_name,
                repo.description.unwrap_or_else(|| "No description".into())
            );
        }
    } else {
        println!("No results found on GitHub.");
    }

    Ok(())
}
