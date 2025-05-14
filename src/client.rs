use crate::{endpoints::*, error::Error};
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

    pub async fn execute(&self, request: &ExecuteRequest) -> Result<ExecuteResponse, Error> {
        self.post(request, Endpoints::Execute).await
    }

    pub async fn compile(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn format(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn clippy(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn miri(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn macro_expansion(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn crates(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn versions(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn gist_create(&self) -> Result<(), Error> {
        todo!()
    }

    pub async fn gist_get(&self) -> Result<(), Error> {
        todo!()
    }

    async fn post<T, U>(&self, request: &T, endpoint: Endpoints) -> Result<U, Error>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let url = self.get_url(endpoint).unwrap();
        let client = reqwest::Client::new();
        let res = client.post(url).json(request).send().await?;

        if !res.status().is_success() {
            return Err(Error::NoSuccess(res.status().as_u16()));
        }

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
        let response: ExecuteResponse = client.execute(&request).await.unwrap();
        assert!(response.success);
        println!("{:?}", response);
    }
}
