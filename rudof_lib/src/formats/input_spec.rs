#[allow(unused_imports)]
use crate::errors::InputSpecError;
#[cfg(target_family = "wasm")]
use crate::wasm_stubs::{Client, ClientBuilder, Response};
use either::Either;
#[cfg(not(target_family = "wasm"))]
use reqwest::blocking::{Client, ClientBuilder, Response};
use reqwest::header::{ACCEPT, HeaderValue, USER_AGENT};
use rudof_iri::IriS;
use std::io::Cursor;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader, StdinLock},
    path::{Path, PathBuf},
    str::FromStr,
};
use url::Url;

/// Specification for different input sources in Rudof.
///
/// Represents the various ways data can be provided to Rudof:
/// from files, standard input, URLs, or raw strings.
#[derive(Debug, Clone)]
pub enum InputSpec {
    /// Input from a file path on the filesystem
    Path(PathBuf),
    /// Input from standard input (stdin)
    Stdin,
    /// Input from a raw string in memory
    Str(String),
    /// Input from a URL (HTTP/HTTPS)
    Url(UrlSpec),
}

/// Specification for URL-based inputs with HTTP client configuration.
#[derive(Debug, Clone)]
pub struct UrlSpec {
    /// The URL to fetch data from
    url: Url,
    /// HTTP client for making requests
    client: Client,
}

/// Reader type that can handle input from multiple sources.
///
/// This type implements [`io::BufRead`] and uses Either types to represent
/// the different possible reader implementations (stdin, file, HTTP response, or string).
pub type InputSpecReader =
    Either<StdinLock<'static>, Either<BufReader<fs::File>, Either<BufReader<Response>, BufReader<Cursor<Vec<u8>>>>>>;

// ============================================================================
// InputSpec
// ============================================================================

impl InputSpec {
    /// Creates an InputSpec from a file path.
    pub fn path<P: AsRef<Path>>(path: P) -> InputSpec {
        InputSpec::Path(PathBuf::from(path.as_ref()))
    }

    /// Returns a human-readable name for the input source.
    pub fn source_name(&self) -> String {
        match self {
            InputSpec::Path(path_buf) => path_buf.display().to_string(),
            InputSpec::Stdin => "stdin".to_string(),
            InputSpec::Url(url_spec) => url_spec.to_string(),
            InputSpec::Str(_) => "string".to_string(),
        }
    }

    pub fn str(s: &str) -> InputSpec {
        InputSpec::Str(s.to_string())
    }

    /// Converts the InputSpec to an IRI (Internationalized Resource Identifier).
    ///
    /// File paths are converted to file:// URLs, and other sources get appropriate IRI representations.
    pub fn as_iri(&self) -> Result<IriS, InputSpecError> {
        match self {
            #[cfg(not(target_family = "wasm"))]
            InputSpec::Path(path) => {
                let path_absolute = path.canonicalize().map_err(|err| InputSpecError::AbsolutePathError {
                    path: path.to_string_lossy().to_string(),
                    error: err,
                })?;
                let url = Url::from_file_path(path_absolute)
                    .map_err(|_| InputSpecError::FromFilePath { path: path.clone() })?;
                Ok(IriS::new_unchecked(url.as_str()))
            },
            #[cfg(target_family = "wasm")]
            InputSpec::Path(_) => Err(InputSpecError::WasmNotSupported {
                operation: "File path operations".to_string(),
            }),
            InputSpec::Stdin => Ok(IriS::new_unchecked("file://stdin")),
            InputSpec::Str(_s) => Ok(IriS::new_unchecked("file://str")),
            InputSpec::Url(url) => Ok(IriS::new_unchecked(url.to_string().as_str())),
        }
    }

    /// Opens the input source for reading.
    ///
    /// # Parameters
    /// - `accept`: Optional HTTP Accept header value for URL requests
    /// - `context_error`: Context string for error messages
    ///
    /// # Returns
    /// A reader that implements `BufRead` for the input source.
    ///
    /// # Note
    /// The initial version of this code was inspired by [patharg](https://github.com/jwodder/patharg/blob/edd912e865143646fd7bb4c7796aa919fa5622b3/src/lib.rs#L264)
    pub fn open_read(&self, accept: Option<&str>, context_error: &str) -> Result<InputSpecReader, InputSpecError> {
        println!("Opening input source: {:?}", self);
        match self {
            InputSpec::Stdin => Ok(Either::Left(io::stdin().lock())),
            InputSpec::Path(p) => match fs::File::open(p) {
                Ok(reader) => Ok(Either::Right(Either::Left(BufReader::new(reader)))),
                Err(e) => Err(InputSpecError::OpenPathError {
                    msg: context_error.to_string(),
                    path: p.to_path_buf(),
                    err: e,
                }),
            },
            InputSpec::Url(url_spec) => {
                let url = url_spec.url.clone();
                let resp = match accept {
                    None => url_spec.client.get(url_spec.url.as_str()),
                    Some(accept_str) => {
                        let mut headers = reqwest::header::HeaderMap::new();
                        let accept_value =
                            HeaderValue::from_str(accept_str).map_err(|e| InputSpecError::AcceptValue {
                                context: context_error.to_string(),
                                str: accept_str.to_string(),
                                error: e.to_string(),
                            })?;
                        headers.insert(ACCEPT, accept_value);
                        let user_agent_rudof = HeaderValue::from_str("rudof")
                            .map_err(|e| InputSpecError::UserAgentValue { error: e.to_string() })?;
                        headers.insert(USER_AGENT, user_agent_rudof);
                        let client = Client::builder()
                            .default_headers(headers)
                            .build()
                            .map_err(|e| InputSpecError::ClientBuilderError { error: format!("{e}") })?;
                        client.get(url_spec.url.as_str())
                    },
                }
                .send()
                .map_err(|e| InputSpecError::UrlDerefError {
                    url,
                    error: format!("{e}"),
                })?;
                let reader = BufReader::new(resp);
                Ok(Either::Right(Either::Right(Either::Left(reader))))
            },
            InputSpec::Str(s) => {
                println!(
                    "Warning: Using raw string input. This is intended for testing and may not be suitable for large inputs."
                );
                // The following code had the problem that it didn't detect properly if the file didn't exist
                let cursor = Cursor::new(s.clone().into_bytes());
                let reader = BufReader::new(cursor);
                /*let path = Path::new(s);
                let file = File::open(&path).map_err(|e| InputSpecError::OpenPathError {
                    msg: context_error.to_string(),
                    path: path.to_path_buf(),
                    err: e,
                })?; */
                // let reader = BufReader::new(file);
                // Ok(Either::Right(Either::Left(reader)))
                Ok(Either::Right(Either::Right(Either::Right(reader))))
            },
        }
    }

    /// Attempts to guess a base IRI for the input source.
    ///
    /// Used when no explicit base IRI is provided for relative IRI resolution.
    pub fn guess_base(&self) -> Result<String, InputSpecError> {
        match self {
            #[cfg(not(target_family = "wasm"))]
            InputSpec::Path(path) => {
                let absolute_path = fs::canonicalize(path).map_err(|err| InputSpecError::AbsolutePathError {
                    path: path.to_string_lossy().to_string(),
                    error: err,
                })?;
                let url: Url = Url::from_file_path(absolute_path).map_err(|_| InputSpecError::GuessBaseFromPath {
                    path: path.to_path_buf(),
                })?;
                Ok(url.to_string())
            },
            #[cfg(target_family = "wasm")]
            InputSpec::Path(_) => Err(InputSpecError::WasmNotSupported {
                operation: "File path operations".to_string(),
            }),
            InputSpec::Stdin => Ok("stdin://".to_string()),
            InputSpec::Url(url_spec) => Ok(url_spec.url.to_string()),
            InputSpec::Str(_) => Ok("string://".to_string()),
        }
    }

    /*     pub fn parse_from_str(s: &str) -> Result<Self, InputSpecError> {
        match s {
            _ if s == "-" => Ok(InputSpec::Stdin),
            _ if s.starts_with("http://") => {
                let url_spec = UrlSpec::parse(s)?;
                Ok(InputSpec::Url(url_spec))
            },
            _ if s.starts_with("https://") => {
                let url_spec = UrlSpec::parse(s)?;
                Ok(InputSpec::Url(url_spec))
            },
            _ if Path::new(s).exists() => {
                let pb: PathBuf = PathBuf::from_str(s).map_err(|e| InputSpecError::ParsingPathError {
                    str: s.to_string(),
                    error: format!("{e}"),
                })?;
                Ok(InputSpec::Path(pb))
            },
            _ => Err(InputSpecError::FileDoesntExist { str: s.to_string() }),
        }
    }*/
}

impl Display for InputSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InputSpec::Path(path_buf) => write!(f, "Path: {}", path_buf.display()),
            InputSpec::Stdin => write!(f, "Stdin"),
            InputSpec::Url(url_spec) => write!(f, "Url: {url_spec}"),
            InputSpec::Str(s) => write!(f, "String: {s}"),
        }
    }
}

/*
impl FromStr for InputSpec {
    type Err = InputSpecError;

    /// Parses a string into an InputSpec.
    ///
    /// # Logic
    /// - "-" becomes Stdin
    /// - Strings starting with "http://" or "https://" become URLs
    /// - Existing file paths become Path
    /// - Everything else becomes a raw string (Str)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_from_str(s)
    }
}
*/
// ============================================================================
// UrlSpec
// ============================================================================

impl UrlSpec {
    /// Parses a string into a UrlSpec with an HTTP client.
    pub fn parse(str: &str) -> Result<UrlSpec, InputSpecError> {
        let url = Url::parse(str).map_err(|e| InputSpecError::UrlParseError {
            str: str.to_string(),
            error: format!("{e}"),
        })?;
        let client = ClientBuilder::new()
            .build()
            .map_err(|e| InputSpecError::ClientBuilderError { error: format!("{e}") })?;
        Ok(UrlSpec { url, client })
    }

    /// Returns the URL as a string slice.
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl Display for UrlSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
