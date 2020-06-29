pub mod chuck {
    extern crate reqwest;
    extern crate serde;
    use serde::{Deserialize, Serialize};
    const BASE_URL: &'static str = "https://api.chucknorris.io/jokes";
    const RANDOM_URL: &'static str = "/random";

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Category {
        Animal,
        Career,
        Celebrity,
        Dev,
        Explicit,
        Fashion,
        Food,
        History,
        Money,
        Movie,
        Music,
        Political,
        Religion,
        Science,
        Sport,
        Travel,
    }

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
        pub async fn get_fact_by_category(&self, c: Category) -> Result<String, reqwest::Error> {
            let category: String = match c {
                Category::Animal => String::from("animal"),
                Category::Dev => String::from("dev"),
                Category::Career => String::from("career"),
                Category::Celebrity => String::from("celebrity"),
                Category::Explicit => String::from("explicit"),
                Category::Fashion => String::from("fashion"),
                Category::Food => String::from("food"),
                Category::History => String::from("history"),
                Category::Money => String::from("money"),
                Category::Movie => String::from("movie"),
                Category::Music => String::from("music"),
                Category::Political => String::from("political"),
                Category::Religion => String::from("religion"),
                Category::Science => String::from("science"),
                Category::Sport => String::from("sport"),
                Category::Travel => String::from("travel"),
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
