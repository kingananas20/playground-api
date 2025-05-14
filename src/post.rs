use crate::endpoints::Endpoints;
use reqwest::Client;
use serde::{Serialize, de::Deserialize};
use url::{ParseError, Url};

pub async fn post<T, U>(request: &T, endpoint: Endpoints) -> Result<U, reqwest::Error>
where
    T: Serialize,
    U: for<'de> Deserialize<'de>,
{
    let domain = std::env::var("PLAYGROUND_URL").unwrap();
    let url = get_url(endpoint).unwrap();
    let client = Client::new();
    let res = client.post(url).json(request).send().await?;

    if !res.status().is_success() {}

    let res = res.json::<U>().await?;
    Ok(res)
}

fn get_url(endpoint: Endpoints) -> Result<Url, ParseError> {
    let domain = std::env::var("PLAYGROUND_URL").unwrap();
    let base = Url::parse(&domain)?;
    let url = match endpoint {
        Endpoints::Execute => base.join("/execute"),
    }?;
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::post;
    use crate::endpoints::*;

    #[tokio::test]
    async fn success() {
        dotenv::dotenv().ok();
        let channel = "stable".to_string();
        let mode = "release".to_string();
        let edition = "2021".to_string();
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

        let response: ExecuteResponse = post(&request, Endpoints::Execute).await.unwrap();
        println!("{:?}", response);
    }
}
