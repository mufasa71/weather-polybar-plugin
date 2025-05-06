use clap::Parser;
use dotenv::dotenv;
use weather_rs::{files::Cli, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Cli::parse();
    let res = run(args).await;

    if let Ok(e) = res {
        println!("{}", e);
    } else {
        println!("Error: {}", res.unwrap_err());
    }
    Ok(())
}
