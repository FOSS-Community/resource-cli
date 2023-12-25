use serde::{Serialize, Deserialize};
use std::{fs, error::Error, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub description: String,
    pub link: String,
    pub github_stars: u64,
    pub community_ratings: f64,
    pub date: String,
    pub keywords: Vec<String>,
}

pub fn read_data() -> Result<Vec<Resource>, Box<dyn Error>> {
    let path = Path::new("src/data/data.json");
    let data = fs::read_to_string(path)?;
    let resources: Vec<Resource> = serde_json::from_str(&data)?;
    Ok(resources)
}

pub fn write_data(resources: &[Resource]) -> Result<(), Box<dyn Error>> {
    let updated_data = serde_json::to_string_pretty(&resources)?;
    fs::write("src/data/data.json", updated_data)?;
    Ok(())
}
