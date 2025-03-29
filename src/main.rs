use dotenv::dotenv;
use std::env;
use weather_polybar::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let q = args.get(1).ok_or("Expected argument 'q'")?;

    let res = run(q).await;

    if let Ok(e) = res {
        println!("{}", e);
    } else {
        println!("Error: {}", res.unwrap_err());
    }
    Ok(())
}
