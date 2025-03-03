pub mod plugin;

use plugin::{output_compact, Weather};

pub async fn run(q: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res: Weather = client
        .get("https://weatherapi-com.p.rapidapi.com/current.json")
        .header("x-rapidapi-key", "")
        .header("x-rapidapi-host", "weatherapi-com.p.rapidapi.com")
        .query(&[("q", q)])
        .send()
        .await?
        .json()
        .await?;

    Ok(output_compact(&res))
}
