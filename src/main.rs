mod data {
    pub mod add_data;
    pub mod data;
    pub mod update_data;
}

mod prompts {
    pub mod search;
    pub mod todays_hot;
    pub mod trending;
}

// Renamed from vote to rating for clarity
mod vote {
    pub mod rate;
}

use colored::*;
use data::add_data;
use data::update_data;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use prompts::search;
use prompts::todays_hot;
use prompts::trending;
use vote::rate;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "Welcome to Resource CLI!".cyan().bold());

    let selections = vec![
        "Update Data",
        "Add Project",
        "Rate a Project",
        "Search",
        "Today's Hot",
        "Trending on GitHub",
        "Most Voted",
    ];

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact_opt()?;

    match selection_index {
        Some(0) => update_data::update_data_with_github_info().await?,
        Some(1) => {
            let username_repo: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the GitHub username/repo (e.g., username/repo)")
                .interact_text()?;
            add_data::add_project(&username_repo).await?;
        }
        Some(2) => rate::rate_project().await?,
        Some(3) => search::perform_search().await?,
        Some(4) => todays_hot::show_todays_hot().await?,
        Some(5) => trending::show_trending().await?,
        Some(6) => {
            println!("Most Voted functionality coming soon...");
        },
        None => println!("{}", "No selection made, exiting.".red()),
        _ => println!("{}", "Invalid option.".red()),
    }

    Ok(())
}
