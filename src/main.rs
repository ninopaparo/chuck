extern crate reqwest;
extern crate serde;
use ::chuck::chuck;
type CustomResult = Result<(), Box<dyn std::error::Error>>;
#[tokio::main]
async fn main() -> CustomResult {
    let r = chuck::Client::new().get_random_joke().await?;
    println!("{}", r);
    let r2 = chuck::Client::new()
        .get_fact_by_category(chuck::Category::Food)
        .await?;
    println!("{}", r2);
    Ok(())
}
