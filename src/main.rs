mod data {
    pub mod data;
    pub mod update_data;
    pub mod add_data; 
}

use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let selections = vec![
        "Update Data",
        "Add Project",
        "Vote on Project",
        "Search",
        "Today's Hot",
        "Trending on GitHub",
        "Most Voted"
    ];

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact_opt()?;

    match selection_index {
        Some(0) => data::update_data::update_data_with_github_info().await?,
        Some(1) => {
            let username_repo: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the GitHub username/repo (e.g., username/repo)")
                .interact_text()?;
            data::add_data::add_project(&username_repo).await?;
        },
        Some(2) => println!("Voting functionality coming soon..."),
        Some(3) => println!("Search functionality coming soon..."),
        Some(4) => println!("Today's Hot functionality coming soon..."),
        Some(5) => println!("Trending on GitHub functionality coming soon..."),
        Some(6) => println!("Most Voted functionality coming soon..."),
        None => println!("No selection made, exiting."),
        _ => println!("Invalid option."),
    }

    Ok(())
}
