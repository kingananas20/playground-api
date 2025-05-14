use crate::endpoints::Endpoints;
use serde::{Serialize, de::Deserialize};
use url::{ParseError, Url};

pub struct Client {
    url: String,
}

impl Client {
    pub fn new(url: &str) -> Client {
        let url = url.to_string();
        Client { url }
    }

    pub async fn post<T, U>(&self, request: &T, endpoint: Endpoints) -> Result<U, reqwest::Error>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let url = self.get_url(endpoint).unwrap();
        let client = reqwest::Client::new();
        let res = client.post(url).json(request).send().await?;

        if !res.status().is_success() {}

        let res = res.json::<U>().await?;
        Ok(res)
    }

    fn get_url(&self, endpoint: Endpoints) -> Result<Url, ParseError> {
        let base = Url::parse(&self.url)?;
        let url = match endpoint {
            Endpoints::Execute => base.join("/execute"),
        }?;
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::endpoints::*;

    #[tokio::test]
    async fn success() {
        dotenv::dotenv().ok();
        let channel = "stable".to_string();
        let mode = "release".to_string();
        let edition = "2024".to_string();
        let crate_type = "bin".to_string();
        let code = "fn main() { println!(\"Hello, world!\"); }".to_string();

        let request = ExecuteRequest {
            channel,
            mode,
            edition,
            crate_type,
            tests: false,
            backtrace: false,
            code,
        };

        let client = Client::new("https://play.rust-lang.org/");
        let response: ExecuteResponse = client.post(&request, Endpoints::Execute).await.unwrap();
        assert!(response.success);
        println!("{:?}", response);
    }
}
