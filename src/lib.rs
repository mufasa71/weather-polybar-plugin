pub mod files;

use files::{get_config, output_compact, Weather};

pub async fn run(q: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let config = get_config();
    let res: Weather = client
        .get("https://weatherapi-com.p.rapidapi.com/current.json")
        .header("x-rapidapi-key", config.api_key)
        .header("x-rapidapi-host", "weatherapi-com.p.rapidapi.com")
        .query(&[("q", q)])
        .send()
        .await?
        .json()
        .await?;

    Ok(output_compact(&res))
}
