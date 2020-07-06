extern crate reqwest;
extern crate serde;
use ::chuck::chuck;
type CustomResult = Result<(), Box<dyn std::error::Error>>;
use clap::{load_yaml, App};

#[tokio::main]
async fn main() -> CustomResult {
    let yaml = load_yaml!("args.yml");
    let matches = App::from(yaml).get_matches();

    let category = matches.value_of("category").unwrap_or("none");

    let r = chuck::Client::new().get_chuck_facts(category).await?;
    println!("{}", r);

    Ok(())
}
