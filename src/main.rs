use clap::Parser;
use dotenv::dotenv;
use weather_rs::run;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    q: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();
    let res = run(args.q).await;

    if let Ok(e) = res {
        println!("{}", e);
    } else {
        println!("Error: {}", res.unwrap_err());
    }
    Ok(())
}
