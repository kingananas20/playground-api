use crate::{endpoints::*, error::Error};
use serde::{de::Deserialize, Serialize};
use url::{ParseError, Url};

/// A client for interacting with the Rust playground API.
///
/// Holds the base URL and the `reqwest::Client` struct for all requests.
#[derive(Clone)]
pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    /// Creates a new `Client` instance with the given base URL.
    ///
    /// Parses the provided string into a `Url`. Returns an error if the URL is invalid.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice representing the base URL of the Rust playground API.
    ///
    /// # Returns
    ///
    /// * `Result<Client, Error>` - On success, returns a `Client` configured with the parsed URL.
    ///   On failure, returns an `Error` if the URL string is invalid.
    pub fn new(url: &str) -> Result<Client, Error> {
        let url = Url::parse(url)?;
        let client = reqwest::Client::new();
        Ok(Client { url, client })
    }

    /// Sends a code execution request to the Rust playground and returns the result.
    ///
    /// This asynchronous method takes and [`ExecuteRequest`] struct containing the code
    /// execution parameters, sends it to the appropriate endpoint on the Rust playground
    /// via a POST request, and returns the execution result.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to an [`ExecuteRequest`] that includes the code snippet
    ///   and configuration options such as edition, crate type, and whether to run or compile.
    ///
    /// # Returns
    ///
    /// * `Result<ExecuteResponse, Error>` - On success, returns an [`ExecuteResponse`] containing
    ///   the output, errors, and status from the Rust playground. On failure, returns an [`Error`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails, if the response cannot be parsed,
    /// or if the playground service is unavailable.
    pub async fn execute(&self, request: &ExecuteRequest) -> Result<ExecuteResponse, Error> {
        self.post(request, Endpoints::Execute).await
    }

    /// Sends a code compilation request to the Rust playground and returns the result.
    ///
    /// This asynchronous method takes a [`CompileRequest`] containing the code and
    /// compilation parameters, sends it to the Rust playground's compile endpoint,
    /// and returns the compilation result.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`CompileRequest`] that includes the code and metadata
    ///   such as the toolchain edition, crate type, target, and any compiler settings.
    ///
    /// # Returns
    ///
    /// * `Result<CompileResponse, Error>` - On success, returns a [`CompileResponse`] containing
    ///   the compiler output, including success/failure status, messages, and possible warnings or errors.
    ///   On failure, returns an [`Error`] describing the issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, if the response cannot be parsed correctly,
    /// or if the playground service encounters an issue.
    pub async fn compile(&self, request: &CompileRequest) -> Result<CompileResponse, Error> {
        self.post(request, Endpoints::Compile).await
    }

    /// Sends a code formatting request to the Rust playground and returns the formatted result.
    ///
    /// This asynchronous method takes a [`FormatRequest`] containing the Rust code and formatting options,
    /// sends it to the Rust playground's format endpoint, and returns the formatted code or any errors.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`FormatRequest`] that includes the code to be formatted and
    ///   optional parameters like the edition to use.
    ///
    /// # Returns
    ///
    /// * `Result<FormatResponse, Error>` - On success, returns a [`FormatResponse`] containing the
    ///   formatted code or an error message if the code could not be formatted.
    ///   On failure, returns an [`Error`] representing issues like network failure or parsing problems.
    ///
    /// # Errors
    ///
    /// This function may return an error if the request fails, the response is invalid,
    /// or the Rust playground's formatting service encounters a problem.
    pub async fn format(&self, request: &FormatRequest) -> Result<FormatResponse, Error> {
        self.post(request, Endpoints::Format).await
    }

    /// Sends a Clippy linting request to the Rust playground and returns the analysis result.
    ///
    /// This asynchronous method takes a [`ClippyRequest`] containing the Rust code and configuration,
    /// sends it to the Rust playground's Clippy endpoint, and returns any linter warnings, errors,
    /// or suggestions provided by Clippy.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`ClippyRequest`] that includes the code to be analyzed
    ///   and optional parameters such as edition or crate type.
    ///
    /// # Returns
    ///
    /// * `Result<ClippyResponse, Error>` - On success, returns a [`ClippyResponse`] containing
    ///   Clippy's diagnostic output (warnings, errors, suggestions). On failure, returns an [`Error`]
    ///   describing what went wrong (e.g., network error, bad request, or service issue).
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be completed, the response is invalid,
    /// or the Clippy service is unavailable or encounters an internal error.
    pub async fn clippy(&self, request: &ClippyRequest) -> Result<ClippyResponse, Error> {
        self.post(request, Endpoints::Clippy).await
    }

    /// Sends a Miri request to the Rust playground and returns the result of interpreting the code.
    ///
    /// This asynchronous method takes a [`MiriRequest`] containing the Rust code and any
    /// interpreter-specific options, sends it to the Rust playground's Miri endpoint, and
    /// returns the result of running the interpreter on the code.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`MiriRequest`] that includes the code and metadata
    ///   such as edition, crate type, and other configuration options.
    ///
    /// # Returns
    ///
    /// * `Result<MiriResponse, Error>` - On success, returns a [`MiriResponse`] containing the
    ///   result of the interpretation. On failure, returns an [`Error`] describing the issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, if the response is invalid, or if the Miri service
    /// encounters an internal issue.
    pub async fn miri(&self, request: &MiriRequest) -> Result<MiriResponse, Error> {
        self.post(request, Endpoints::Miri).await
    }

    /// Sends a macro expansion request to the Rust playground and returns the result.
    ///
    /// This asynchronous method takes a [`MacroExpansionRequest`] with Rust code containing macros,
    /// sends it to the Rust playground's macro expansion endpoint, and returns the result
    /// of the expanded macros.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`MacroExpansionRequest`] that includes the code and any
    ///   configuration options like the edition to use.
    ///
    /// # Returns
    ///
    /// * `Result<MacroExpansionResponse, Error>` - On success, returns a [`MacroExpansionResponse`]
    ///   containing the macro-expanded version of the code. On failure, returns an [`Error`] describing
    ///   the issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, if the response is invalid, or if the macro expansion
    /// service encounters an issue.
    pub async fn macro_expansion(
        &self,
        request: &MacroExpansionRequest,
    ) -> Result<MacroExpansionResponse, Error> {
        self.post(request, Endpoints::MacroExpansion).await
    }

    /// Retrieves the list of available crates from the Rust playground.
    ///
    /// This asynchronous method sends a GET request to the crates endpoint
    /// and returns a list of crates supported by the playground environment.
    ///
    /// # Returns
    ///
    /// * `Result<CratesResponse, Error>` - On success, returns a [`CratesResponse`] containing
    ///   the names and versions of available crates. On failure, returns an [`Error`] describing
    ///   the problem.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, if the response cannot be parsed,
    /// or if the crates service is unavailable.
    pub async fn crates(&self) -> Result<CratesResponse, Error> {
        self.get(Endpoints::Crates).await
    }

    /// Retrieves the supported versions and metadata of the Rust playground.
    ///
    /// This asynchronous method sends a GET request to the versions endpoint and
    /// returns information about supported Rust versions, targets, and environments.
    ///
    /// # Returns
    ///
    /// * `Result<VersionsResponse, Error>` - On success, returns a [`VersionsResponse`]
    ///   containing version details. On failure, returns an [`Error`] describing what went wrong.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be completed, the response is malformed,
    /// or if the versions service is unavailable.
    pub async fn versions(&self) -> Result<VersionsResponse, Error> {
        self.get(Endpoints::Versions).await
    }

    /// Creates a GitHub Gist from the provided Rust playground code.
    ///
    /// This asynchronous method sends a [`GistCreateRequest`] to the Gist creation endpoint
    /// and returns a response containing the Gist URL or error information.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`GistCreateRequest`] that includes the code to be uploaded
    ///   as a Gist and any additional metadata like description or visibility.
    ///
    /// # Returns
    ///
    /// * `Result<GistResponse, Error>` - On success, returns a [`GistResponse`] containing
    ///   the Gist ID and URL. On failure, returns an [`Error`] describing what went wrong.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, if the response is malformed,
    /// or if the Gist service is unavailable.
    pub async fn gist_create(&self, request: &GistCreateRequest) -> Result<GistResponse, Error> {
        self.post(request, Endpoints::GistCreate).await
    }

    /// Retrieves an existing GitHub Gist from the Rust playground.
    ///
    /// This asynchronous method sends a GET request to the Gist retrieval endpoint
    /// using the provided Gist ID and returns the contents of the Gist.
    ///
    /// # Arguments
    ///
    /// * `id` - A `String` representing the unique identifier of the Gist to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<GistResponse, Error>` - On success, returns a [`GistResponse`] containing
    ///   the Gist's code and metadata. On failure, returns an [`Error`] describing the issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, if the response is invalid,
    /// or if the Gist could not be found.
    pub async fn gist_get(&self, id: String) -> Result<GistResponse, Error> {
        self.get(Endpoints::GistGet(id)).await
    }

    /// Sends a POST request with a serialized JSON payload to the specified endpoint,
    /// and deserializes the response into the expected type.
    ///
    /// Used internally to interact with Rust playground endpoints.
    async fn post<T, U>(&self, request: &T, endpoint: Endpoints) -> Result<U, Error>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        let url = self.get_url(endpoint)?;
        let res = self.client.post(url).json(request).send().await?;

        if !res.status().is_success() {
            return Err(Error::NoSuccess(res.status().as_u16()));
        }

        let res = res.json::<U>().await?;
        Ok(res)
    }

    /// Sends a GET request to the specified endpoint, and deserializes the response
    /// into the expected type.
    ///
    /// Used internally to interact with Rust playground endpoints.
    async fn get<U>(&self, endpoint: Endpoints) -> Result<U, Error>
    where
        U: for<'de> Deserialize<'de>,
    {
        let url = self.get_url(endpoint)?;
        let res = self.client.get(url).send().await?;

        if !res.status().is_success() {
            return Err(Error::NoSuccess(res.status().as_u16()));
        }

        let res = res.json::<U>().await?;
        Ok(res)
    }

    /// Takes an endpoint and returns the correct url.
    fn get_url(&self, endpoint: Endpoints) -> Result<Url, ParseError> {
        let url = match endpoint {
            Endpoints::Execute => self.url.join("execute"),
            Endpoints::Compile => self.url.join("compile"),
            Endpoints::Format => self.url.join("format"),
            Endpoints::Clippy => self.url.join("clippy"),
            Endpoints::Miri => self.url.join("miri"),
            Endpoints::Crates => self.url.join("meta/crates"),
            Endpoints::Versions => self.url.join("meta/versions"),
            Endpoints::MacroExpansion => self.url.join("macro-expansion"),
            Endpoints::GistCreate => self.url.join("meta/gist"),
            Endpoints::GistGet(id) => self.url.join(&format!("meta/gist/{}", id)),
        }?;
        Ok(url)
    }
}

impl Default for Client {
    /// Creates a `Client` instance with the following url <https://play.rust-lang.org/>
    fn default() -> Self {
        let client = reqwest::Client::new();
        Self {
            url: Url::parse("https://play.rust-lang.org/").unwrap(),
            client,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::endpoints::*;

    #[tokio::test]
    async fn execute() {
        let req = ExecuteRequest::default();

        let client = Client::default();
        let res = client.execute(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn compile() {
        let req = CompileRequest::default();

        let client = Client::default();
        let res = client.compile(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn format() {
        let req = FormatRequest::default();

        let client = Client::default();
        let res = client.format(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn clippy() {
        let req = ClippyRequest::default();

        let client = Client::default();
        let res = client.clippy(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn miri() {
        let req = MiriRequest::default();

        let client = Client::default();
        let res = client.miri(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn macro_expansion() {
        let req = MacroExpansionRequest::default();

        let client = Client::default();
        let res = client.macro_expansion(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn crates() {
        let client = Client::default();
        let res = client.crates().await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn version() {
        let client = Client::default();
        let res = client.versions().await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    #[ignore = "don't flood rust playground gist with useless gists"]
    async fn gist_create() {
        let req = GistCreateRequest::new("fn main() { println!(\"Hello, world!\"); }".to_owned());

        let client = Client::default();
        let res = client.gist_create(&req).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn gist_get() {
        let id = "ba5e40fb63e78da440797e921bbf2aa6".to_owned();

        let client = Client::default();
        let res = client.gist_get(id).await;

        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
