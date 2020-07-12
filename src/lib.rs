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
        pub async fn get_chuck_facts(&self, c: &str) -> Result<String, reqwest::Error> {
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

            let url: String;

            if category != "none" {
                url = format!("{}{}?category={}", &self.url, RANDOM_URL, category);
            } else {
                url = format!("{}{}", &self.url, RANDOM_URL);
            }

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

#[cfg(test)]
mod tests {
    use crate::chuck;
    use wiremock::matchers::{method, PathExactMatcher};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn client_url_test() {
        let client = chuck::Client::new();
        assert_eq!("https://api.chucknorris.io/jokes", client.url);
    }

    #[tokio::test]
    #[ignore]
    async fn check_category_test() {
        let category = "Movie";
        // Act
        let mock_server = MockServer::start().await;
        let json_data = json::parse(r#"
        {
            "icon_url" : "https://assets.chucknorris.host/img/avatar/chuck-norris.png",
            "id" : "D8o00FE0RNSNUlltwEPDVg",
            "url" : "",
            "value" : "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move."
        }"#).unwrap();
        let response = ResponseTemplate::new(200).set_body_json(json_data.dump());
        let mock = Mock::given(method("GET"))
            .and(PathExactMatcher::new(format!(
                "{}{}?category={}",
                &mock_server.uri(),
                "/random",
                category
            )))
            .respond_with(response);
        mock_server.register(mock).await;
        // Act
        let client = chuck::Client {
            url: &mock_server.uri(),
            cli: reqwest::Client::new(),
        };

        let r = client.get_chuck_facts(category).await.unwrap();
        // Assert
        let expected_value = "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move.";
        assert_eq!(r, expected_value);
    }
}
