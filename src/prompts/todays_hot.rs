use crate::data::data::read_data;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct GithubRepo {
    full_name: String,
    description: String,
}

pub async fn show_todays_hot() -> Result<(), Box<dyn Error>> {
    let local_resources = read_data()?;

    if let Some(top_project) = local_resources
        .iter()
        .max_by(|a, b| a.github_stars.cmp(&b.github_stars).then_with(|| a.community_ratings.partial_cmp(&b.community_ratings).unwrap_or(std::cmp::Ordering::Equal)))
    {
        println!("Today's Hot Project from local data:");
        println!("{} - {}", top_project.name, top_project.description);
    } else {
        println!("Fetching today's hot project from GitHub...");
        fetch_todays_hot_from_github().await?;
    }

    Ok(())
}

async fn fetch_todays_hot_from_github() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.github.com/search/repositories?q=stars:>1&sort=stars&order=desc";

    let resp = client
        .get(url)
        .header("User-Agent", "request")
        .send()
        .await?;

    let search_result = resp.json::<Vec<GithubRepo>>().await?;

    if let Some(hot_repo) = search_result.first() {
        println!("Today's Hot Project from GitHub:");
        println!("{} - {}", hot_repo.full_name, hot_repo.description);
    } else {
        println!("No hot projects found on GitHub.");
    }

    Ok(())
}
