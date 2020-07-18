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
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn client_url_test() {
        let client = chuck::Client::new();
        assert_eq!("https://api.chucknorris.io/jokes", client.url);
    }

    #[tokio::test]
    async fn get_facts_no_category() {
        let mock_server = MockServer::start().await;

        let payload = r#"
        {
            "icon_url" : "https://assets.chucknorris.host/img/avatar/chuck-norris.png",
            "id" : "1111",
            "url" : "",
            "value" : "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move."
        }"#
        .as_bytes()
        .to_owned();
        let response = ResponseTemplate::new(200).set_body_raw(payload, "application/json");
        let mock = Mock::given(method("GET"))
            .and(path("random"))
            .respond_with(response);
        mock_server.register(mock).await;

        let client = chuck::Client {
            url: &mock_server.uri(),
            cli: reqwest::Client::new(),
        };
        let value = client.get_chuck_facts("non existent field").await.unwrap();

        let expected_value = String::from(
            "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move.",
        );
        assert_eq!(value, expected_value);
    }
    #[tokio::test]
    async fn get_facts_category() {
        let category = "Movie";
        let mock_server = MockServer::start().await;

        let payload: Vec<u8> = r#"
        {
            "icon_url" : "https://assets.chucknorris.host/img/avatar/chuck-norris.png",
            "id" : "1111",
            "url" : "",
            "value" : "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move."
        }"#
        .as_bytes()
        .to_owned();
        let response = ResponseTemplate::new(200).set_body_raw(payload, "application/json");
        let mock = Mock::given(method("GET"))
            .and(path("random"))
            .and(query_param("category", category.to_ascii_lowercase()))
            .respond_with(response);
        mock_server.register(mock).await;

        let client = chuck::Client {
            url: &mock_server.uri(),
            cli: reqwest::Client::new(),
        };
        let value = client.get_chuck_facts(category).await.unwrap();

        let expected_value = String::from(
            "Chuck Norris has a Grizzly Bear rug laid out on his family room floor. The Grizzly Bear isn't dead...it's just too afraid to move.",
        );
        assert_eq!(value, expected_value);
    }
}
