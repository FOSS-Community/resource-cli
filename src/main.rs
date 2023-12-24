use dialoguer::{theme::ColorfulTheme, Input, Select};
use colored::*;
use serde::{Deserialize, Serialize};
use std::{fs, error::Error};

#[derive(Serialize, Deserialize, Debug)]
struct Resource {
    name: String,
    description: String,
    link: String,
    github_stars: u64,
    community_ratings: Vec<f64>,
    date: String,
    keywords: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_theme = ColorfulTheme::default();

    let selections = vec!["Today's Hot", "Trending on GitHub", "Most Voted", "Recent Listing", "Custom Search"];
    let selection_index = Select::with_theme(&my_theme)
        .with_prompt("Choose a category to list resources or search")
        .default(0)
        .items(&selections[..])
        .interact_opt()?;

    let data = fs::read_to_string("data/data.json")?;
    let resources: Vec<Resource> = serde_json::from_str(&data)?;

    match selection_index {
        Some(index) => {
            if index == selections.len() - 1 { // If 'Custom Search' is selected
                let search_query: String = Input::with_theme(&my_theme)
                    .with_prompt("Enter search term")
                    .interact_text()?;
                search_resources(&resources, &search_query);
            } else {
                list_resources(&resources, &selections[index]);
            }
        }
        None => println!("{}", "No selection made, exiting.".red()),
    }

    Ok(())
}

fn list_resources(resources: &[Resource], category: &str) {
    println!("{}", format!("Listing resources for category: {}", category).underline());
    for resource in resources {
        println!(
            "{}\n{}\n{}\n{} Stars: {}\nDate: {}\n-----",
            "Name:".yellow(),
            resource.name.cyan(),
            resource.description,
            "GitHub".bright_purple(),
            resource.github_stars.to_string().bright_yellow(),
            resource.date.green()
        );
    }
}

fn search_resources(resources: &[Resource], query: &str) {
    println!("{}", format!("Searching resources for: {}", query).underline().yellow());
    for resource in resources.iter().filter(|res| res.name.contains(query) || res.description.contains(query) || res.keywords.contains(&query.to_string())) {
        println!(
            "{}\n{}\n{}\n{} Stars: {}\nDate: {}\n-----",
            "Name:".yellow(),
            resource.name.cyan(),
            resource.description,
            "GitHub".bright_purple(),
            resource.github_stars.to_string().bright_yellow(),
            resource.date.green()
        );
    }
}
