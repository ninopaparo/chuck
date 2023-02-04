use anyhow::Result;
use chuck::chuck;
use dialoguer::Confirm;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    utils::show_info();
    loop {
        let category = utils::choose_category();
        println!(
            "{}\n",
            chuck::Client::new().get_chuck_facts(&category).await?
        );
        if !Confirm::new()
            .with_prompt("Do you want to read a new fact?")
            .interact()
            .unwrap()
        {
            println!("Bye!");
            break;
        }
    }
    Ok(())
}
