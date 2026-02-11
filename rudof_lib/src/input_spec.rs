use either::Either;
use iri_s::IriS;
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{ACCEPT, HeaderValue, USER_AGENT},
    // Url as ReqwestUrl,
};
use std::io::Cursor;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader, StdinLock},
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;
use url::Url;

// Consider using clio
#[derive(Debug, Clone)]
pub enum InputSpec {
    Path(PathBuf),
    Stdin,
    Str(String),
    Url(UrlSpec),
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

impl InputSpec {
    pub fn path<P: AsRef<Path>>(path: P) -> InputSpec {
        InputSpec::Path(PathBuf::from(path.as_ref()))
    }

    pub fn source_name(&self) -> String {
        match self {
            InputSpec::Path(path_buf) => path_buf.display().to_string(),
            InputSpec::Stdin => "stdin".to_string(),
            InputSpec::Url(url_spec) => url_spec.to_string(),
            InputSpec::Str(_) => "string".to_string(),
        }
    }

    pub fn as_iri(&self) -> Result<IriS, InputSpecError> {
        match self {
            InputSpec::Path(path) => {
                let path_absolute = path.canonicalize().map_err(|err| InputSpecError::AbsolutePathError {
                    path: path.to_string_lossy().to_string(),
                    error: err,
                })?;
                let url = Url::from_file_path(path_absolute)
                    .map_err(|_| InputSpecError::FromFilePath { path: path.clone() })?;
                Ok(IriS::new_unchecked(url.as_str()))
            },
            InputSpec::Stdin => Ok(IriS::new_unchecked("file://stdin")),
            InputSpec::Str(_s) => Ok(IriS::new_unchecked("file://str")),
            InputSpec::Url(url) => Ok(IriS::new_unchecked(url.to_string().as_str())),
        }
    }

    // The initial version of this code was inspired by [patharg](https://github.com/jwodder/patharg/blob/edd912e865143646fd7bb4c7796aa919fa5622b3/src/lib.rs#L264)
    pub fn open_read(&self, accept: Option<&str>, context_error: &str) -> Result<InputSpecReader, InputSpecError> {
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
                let cursor = Cursor::new(s.clone().into_bytes());
                let reader = BufReader::new(cursor);
                Ok(Either::Right(Either::Right(Either::Right(reader))))
            },
        }
    }

    pub fn guess_base(&self) -> Result<String, InputSpecError> {
        match self {
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
            InputSpec::Stdin => Ok("stdin://".to_string()),
            InputSpec::Url(url_spec) => Ok(url_spec.url.to_string()),
            InputSpec::Str(_) => Ok("string://".to_string()),
        }
    }
}

impl FromStr for InputSpec {
    type Err = InputSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            _ => Ok(InputSpec::Str(s.to_string())),
        }
    }
}

/// This type implements [`std::io::BufRead`].
pub type InputSpecReader = Either<
    StdinLock<'static>,
    Either<BufReader<fs::File>, Either<BufReader<reqwest::blocking::Response>, BufReader<std::io::Cursor<Vec<u8>>>>>,
>;

#[derive(Error, Debug)]
pub enum InputSpecError {
    #[error("IO Error reading {msg} from {path}: {err}")]
    OpenPathError { msg: String, path: PathBuf, err: io::Error },

    #[error("IO Error: {err}")]
    IOError {
        #[from]
        err: io::Error,
    },

    #[error("Url access error: {err}")]
    UrlError {
        #[from]
        err: reqwest::Error,
    },

    #[error("From file path: {path}")]
    FromFilePath { path: PathBuf },

    #[error("Guessing base from path: {path}")]
    GuessBaseFromPath { path: PathBuf },

    #[error("Parsing path error for {str}, error: {error}")]
    ParsingPathError { str: String, error: String },

    #[error("Absolute path error: {path}, error: {error}")]
    AbsolutePathError { path: String, error: io::Error },

    #[error("Url parsing error for :{str} {error}")]
    UrlParseError { str: String, error: String },

    #[error("Client builder error {error}")]
    ClientBuilderError { error: String },

    #[error("Dereferencing url {url} error: {error}")]
    UrlDerefError { url: Url, error: String },

    #[error("Error at {context} creating accept value {str} error: {error}")]
    AcceptValue {
        context: String,
        str: String,
        error: String,
    },

    #[error("Error setting USER_AGENT: {error}")]
    UserAgentValue { error: String },
}

#[derive(Debug, Clone)]
pub struct UrlSpec {
    url: Url,
    client: Client,
}

impl Display for UrlSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl UrlSpec {
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

    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}
