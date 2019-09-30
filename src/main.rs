use reqwest;
use serde::Deserialize;
use serde_json;
use dialoguer::{Confirmation, FuzzySelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use console::Emoji;

#[derive(Deserialize, Debug, Clone)]
struct Language {
    #[serde(rename(deserialize = "urlParam"))]
    url_param: String,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Project {
    author: String,
    name: String,
    avatar: String,
    url: String,
    description: String,
    language: String,
    #[serde(rename(deserialize = "languageColor"))]
    language_color: String,
    stars: i32,
    forks: i32,
    #[serde(rename(deserialize = "currentPeriodStars"))]
    current_period_stars: i32,
    #[serde(rename(deserialize = "builtBy"))]
    built_by: Vec<BuiltBy>,
}

#[derive(Deserialize, Debug, Clone)]
struct BuiltBy {
    username: String,
    avatar: String,
    href: String,
}

// Global Constants
const TIME_RANGES: [&str; 3] = ["daily", "weekly", "monthly"];

// Emojis
static STAR: Emoji<'_, '_> = Emoji("⭐ ", "s: ");
static FORK: Emoji<'_, '_> = Emoji("🍴 ", "f: ");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://github-trending-api.now.sh/languages")
        .send()?
        .text()?;
    let mut languages: Vec<Language> = serde_json::from_str(&body)?;
    languages.push(Language { name: "All".to_owned(), url_param: "All".to_owned() });

    // Convert Vec<Language> vector into a hashmap and selection array.
    let language_items: Vec<String> = languages.iter()
        .cloned()
        .map(|lang| lang.name)
        .collect();

    let language_map: HashMap<String, String> = languages.into_iter()
        .map(|lang| (lang.name, lang.url_param))
        .collect();

    let language_confirmation = Confirmation::with_theme(&ColorfulTheme::default())
        .with_text("Would you like to filter by language?")
        .default(true)
        .interact()?;

    let picked_language = match language_confirmation {
        true => FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .paged(true)
            .items(&language_items[..])
            .interact()?,
        false => "All".to_owned(),
    };

    let time_range = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&TIME_RANGES)
        .interact()?;

    let trend_url = if &picked_language == "All" {
        format!("https://github-trending-api.now.sh/repositories?since={}", TIME_RANGES[time_range])
    } else {
        format!("https://github-trending-api.now.sh/repositories?language={}&since={}", language_map.get(&picked_language).unwrap(), TIME_RANGES[time_range])
    };

    println!("url: {}", trend_url);

    // Call trend api using constructed url
    let trends_body = client
        .get(&trend_url)
        .send()?
        .text()?;

    let trends: Vec<Project> = serde_json::from_str(&trends_body)?;

    // format trends data into digestable selections
    let trend_selections: Vec<String> = trends.iter()
        .cloned()
        .map(|proj| {
            format!(
                "{}\n  {}\n  {} {} {} {}\n  {}",
                proj.name,
                proj.author,
                STAR, proj.stars, FORK, proj.forks,
                proj.description
            )
        })
        .collect();

    let selected_project = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project to work on")
        .default(0)
        .paged(true)
        .lines_per_item(4)
        .items(&trend_selections)
        .interact()?;

    println!("{:?}", trends[selected_project]);

    Ok(())
}
