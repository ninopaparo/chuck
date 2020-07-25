use chuck::chuck;
use dialoguer::Confirm;

mod utils;
type CustomResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> CustomResult {
    utils::utils::show_info();
    loop {
        let category = utils::utils::choose_category();
        println!(
            "{}\n",
            chuck::Client::new().get_chuck_facts(&category).await?
        );
        if !Confirm::new()
            .with_prompt("Do you want to read a new fact?")
            .interact()
            .unwrap()
        {
            break;
        }
    }
    Ok(())
}
