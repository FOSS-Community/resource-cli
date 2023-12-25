use crate::data::data::{read_data, write_data, Resource};
use crate::data::add_data::add_project;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use async_recursion::async_recursion;
use std::error::Error;

#[async_recursion]
pub async fn rate_project() -> Result<(), Box<dyn Error>> {
    let project_name_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the name or part of the name of the project you want to rate")
        .interact_text()?;

    let search_term = project_name_input.trim().to_lowercase();
    let mut local_resources = read_data()?;

    let mut matching_projects: Vec<&mut Resource> = local_resources.iter_mut()
        .filter(|p| 
            p.name.to_lowercase().contains(&search_term) ||
            p.description.to_lowercase().contains(&search_term) ||
            p.keywords.iter().any(|k| k.to_lowercase().contains(&search_term))
        )
        .collect();

    if !matching_projects.is_empty() {
        let selections: Vec<String> = matching_projects.iter().map(|p| p.name.clone()).collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a project to rate")
            .items(&selections)
            .default(0)
            .interact_opt()?;

        if let Some(index) = selection {
            // Correctly borrow a mutable reference
            let project = matching_projects.get_mut(index).unwrap();
            let rating_input: String = Input::new()
                .with_prompt("Rate the project (0-10):")
                .interact_text()?;
            let rating: f64 = rating_input.trim().parse().unwrap_or(0.0);

            // Update the community ratings
            project.community_ratings = (project.community_ratings + rating) / 2.0;
            write_data(&local_resources)?;
            println!("Project rated successfully!");
        } else {
            println!("No project selected.");
        }
    } else {
        println!("No matching projects found in local data.");
        let confirm_add: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to search this project on GitHub and add it? (y/N)")
            .default("N".to_string())
            .interact_text()?;

        if confirm_add.to_lowercase() == "y" {
            add_project(&project_name_input).await?;
            rate_project().await?;
        } else {
            println!("No action taken.");
        }
    }

    Ok(())
}
