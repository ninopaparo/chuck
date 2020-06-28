pub mod chuck {
    extern crate reqwest;
    extern crate serde;
    use serde::{Deserialize, Serialize};
    const BASE_URL: &'static str = "https://api.chucknorris.io/jokes/random";
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
            let url = BASE_URL;
            let cli = reqwest::Client::new();

            Client { url: &url, cli }
        }
        pub async fn get_random_joke(&self) -> Result<String, reqwest::Error> {
            let res = self
                .cli
                .get(self.url)
                .send()
                .await?
                .json::<self::Response>()
                .await?;
            Ok(res.value)
        }
    }
}
