use reqwest;
use serde::Deserialize;
use serde_json;
use dialoguer::{FuzzySelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;
use console::Emoji;
use ansi_term::Colour::{RGB, Black, Fixed};
use std::i64;

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
    language: Option<String>,
    #[serde(rename(deserialize = "languageColor"))]
    language_color: Option<String>,
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

    // Create list of languages to select from in FuzzySelect.
    let language_items: Vec<String> = languages.iter()
        .cloned()
        .map(|lang| lang.name)
        .collect();

    // Place "All" as first option.
    let mut final_items = Vec::with_capacity(language_items.len());
    final_items.push(language_items[language_items.len() - 1].clone());
    for (i, val) in language_items.iter().enumerate() {
        if i == language_items.len() - 1 {
            break;
        }
        final_items.push(val.clone())
    }

    // Convert Vec<Language> vector into a hashmap and selection array.
    let language_map: HashMap<String, String> = languages.into_iter()
        .map(|lang| (lang.name, lang.url_param))
        .collect();

    let picked_language = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a language to filter projects or select \"All\"")
        .default(0)
        .offset(2)
        .paged(true)
        .items(&final_items[..])
        .interact()?;

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
            match proj.language_color {
                Some(col) => {
                    let hex_str = col.trim_start_matches("#");
                    let r = i64::from_str_radix(&hex_str[0..2], 16).unwrap() as u8;
                    let g = i64::from_str_radix(&hex_str[2..4], 16).unwrap() as u8;
                    let b = i64::from_str_radix(&hex_str[4..6], 16).unwrap() as u8;
                    format!(
                        "{} {}\n  {}\n  {} {} {} {}\n  {}\n",
                        Fixed(112).paint(proj.name), Black.on(RGB(r, b, g)).paint(proj.language.unwrap()),
                        Fixed(8).paint(proj.author),
                        STAR, proj.stars, FORK, proj.forks,
                        proj.description
                    )
                },
                None => {
                    format!(
                        "{}\n  {}\n  {} {} {} {}\n  {}\n",
                        Fixed(112).paint(proj.name),
                        Fixed(8).paint(proj.author),
                        STAR, proj.stars, FORK, proj.forks,
                        proj.description
                    )
                }
            }
        })
        .collect();

    let selected_project = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project to work on")
        .default(0)
        .paged(true)
        .lines_per_item(5)
        .offset(1)
        .items(&trend_selections)
        .interact()?;

    println!("{} {}", trends[selected_project].name, trends[selected_project].url);

    Ok(())
}
