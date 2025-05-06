pub mod files;

use files::{get_config, output_compact, Cli, Format, Weather};

pub async fn run(args: Cli) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let config = get_config();
    let mut query = vec![("q", args.q)];
    if args.aqi {
        query.push(("aqi", String::from("yes")));
    }
    let res: Weather = client
        .get("https://weatherapi-com.p.rapidapi.com/current.json")
        .header("x-rapidapi-key", config.api_key)
        .header("x-rapidapi-host", "weatherapi-com.p.rapidapi.com")
        .query(&query)
        .send()
        .await?
        .json()
        .await?;

    match args.format {
        Format::Json => Ok(serde_json::to_string(&res)?),
        Format::Compact => Ok(output_compact(&res)),
    }
}
