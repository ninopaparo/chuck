extern crate reqwest;
extern crate serde;
use chuck::chuck;
type CustomResult = Result<(), Box<dyn std::error::Error>>;
#[tokio::main]
async fn main() -> CustomResult {
    let r = chuck::Client::new().get_random_joke().await?;
    println!("{}", r);
    Ok(())
}
