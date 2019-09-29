use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
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
    let languages: Vec<Language> = serde_json::from_str(&body)?;
    println!("languages: {:?}", languages);

    Ok(())
}
