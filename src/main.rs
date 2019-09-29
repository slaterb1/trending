use reqwest;
use serde::Deserialize;
use serde_json;
use dialoguer::{Confirmation, FuzzySelect, Select, theme::ColorfulTheme};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
struct Language {
    #[serde(rename(deserialize = "urlParam"))]
    url_param: String,
    name: String,
}

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
            .paged(true)
            .items(&language_items[..])
            .interact()?,
        false => "All".to_owned(),
    };

    println!("Selected language: {}", picked_language);

    Ok(())
}
