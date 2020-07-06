pub mod chuck {
    extern crate reqwest;
    extern crate serde;
    use serde::{Deserialize, Serialize};
    const BASE_URL: &'static str = "https://api.chucknorris.io/jokes";
    const RANDOM_URL: &'static str = "/random";

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        icon_url: String,
        id: String,
        url: String,
        value: String,
    }

    pub struct Client<'a> {
        pub url: &'a str,
        pub cli: reqwest::Client,
    }

    impl<'a> Client<'a> {
        pub fn new() -> Self {
            Client {
                url: &BASE_URL,
                cli: reqwest::Client::new(),
            }
        }
        pub async fn get_random_joke(&self) -> Result<String, reqwest::Error> {
            let url = format!("{}{}", BASE_URL, RANDOM_URL);
            Ok(self
                .cli
                .get(&url)
                .send()
                .await?
                .json::<self::Response>()
                .await?
                .value)
        }
        pub async fn get_fact_by_category(&self, c: &str) -> Result<String, reqwest::Error> {
            let category: String = match c {
                "Animal" => String::from("animal"),
                "Dev" => String::from("dev"),
                "Career" => String::from("career"),
                "Celebrity" => String::from("celebrity"),
                "Explicit" => String::from("explicit"),
                "Fashion" => String::from("fashion"),
                "Food" => String::from("food"),
                "History" => String::from("history"),
                "Money" => String::from("money"),
                "Movie" => String::from("movie"),
                "Music" => String::from("music"),
                "Political" => String::from("political"),
                "Religion" => String::from("religion"),
                "Science" => String::from("science"),
                "Sport" => String::from("sport"),
                "Travel" => String::from("travel"),
                _ => String::from("none"),
            };
            let url = format!("{}{}?category={}", BASE_URL, RANDOM_URL, category);
            let res = self
                .cli
                .get(&url)
                .send()
                .await?
                .json::<self::Response>()
                .await?;
            Ok(res.value)
        }
    }
}
