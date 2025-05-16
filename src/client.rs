use crate::{endpoints::*, error::Error};
use serde::{de::Deserialize, Serialize};
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

    pub async fn compile(&self, _request: &CompileRequest) -> Result<CompileResponse, Error> {
        todo!();
        #[allow(unreachable_code)]
        self.post(_request, Endpoints::Compile).await
    }

    pub async fn format(&self, request: &FormatRequest) -> Result<FormatResponse, Error> {
        self.post(request, Endpoints::Format).await
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
            Endpoints::Compile => base.join("/compile"),
            Endpoints::Format => base.join("/format"),
        }?;
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::endpoints::*;

    const URL: &str = "https://play.rust-lang.org/";

    const CHANNEL: Channel = Channel::Stable;
    const MODE: Mode = Mode::Release;
    const EDITION: Edition = Edition::Edition2024;
    const CRATE_TYPE: CrateType = CrateType::Binary;
    const CODE: &str = "fn main() { println!(\"Hello, world!\"); }";
    const _TARGET: CompileTarget = CompileTarget::LlvmIr;

    #[tokio::test]
    async fn execute() {
        let request = ExecuteRequest {
            channel: CHANNEL,
            mode: MODE,
            edition: EDITION,
            crate_type: CRATE_TYPE,
            tests: false,
            backtrace: false,
            code: CODE.to_string(),
        };

        let client = Client::new(URL);
        let response = client.execute(&request).await.unwrap();
        println!("{:?}", response);
    }

    async fn _compile() {
        let request = CompileRequest {
            target: _TARGET,
            channel: CHANNEL,
            mode: MODE,
            edition: EDITION,
            crate_type: CRATE_TYPE,
            code: CODE.to_string(),
            tests: false,
            backtrace: false,
        };

        let client = Client::new(URL);
        let response = client.compile(&request).await;
        assert!(response.is_ok());
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn format() {
        let req = FormatRequest::new(
            Channel::Stable,
            CrateType::Binary,
            Edition::Edition2024,
            "fn main(){            \n\n   println!(\"Hello, world!\"); \n }\n".to_string(),
        );

        let client = Client::new(URL);
        let res = client.format(&req).await;
        assert!(res.is_ok());
        println!("{:?}", res.unwrap());
    }
}
