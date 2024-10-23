use either::Either;
use iri_s::IriS;
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderValue, ACCEPT},
    Url,
};
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader, StdinLock},
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;

// Consider using clio
#[derive(Debug, Clone)]
pub enum InputSpec {
    Path(PathBuf),
    Stdin,
    Url(UrlSpec),
}

impl Display for InputSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InputSpec::Path(path_buf) => write!(f, "Path: {}", path_buf.display()),
            InputSpec::Stdin => write!(f, "Stdin"),
            InputSpec::Url(url_spec) => write!(f, "Url: {url_spec}"),
        }
    }
}

impl InputSpec {
    pub fn path<P: AsRef<Path>>(path: P) -> InputSpec {
        InputSpec::Path(PathBuf::from(path.as_ref()))
    }

    pub fn as_iri(&self) -> Result<IriS, InputSpecError> {
        match self {
            InputSpec::Path(path) => {
                let path_absolute =
                    path.canonicalize()
                        .map_err(|err| InputSpecError::AbsolutePathError {
                            path: path.to_string_lossy().to_string(),
                            error: err,
                        })?;
                let url = Url::from_file_path(path_absolute)
                    .map_err(|_| InputSpecError::FromFilePath { path: path.clone() })?;
                Ok(IriS::new_unchecked(url.as_str()))
            }
            InputSpec::Stdin => Ok(IriS::new_unchecked("file://stdin")),
            InputSpec::Url(url) => Ok(IriS::new_unchecked(url.to_string().as_str())),
        }
    }

    // The initial version of this code was inspired by [patharg](https://github.com/jwodder/patharg/blob/edd912e865143646fd7bb4c7796aa919fa5622b3/src/lib.rs#L264)
    pub fn open_read(&self, accept: Option<&str>) -> Result<InputSpecReader, InputSpecError> {
        match self {
            InputSpec::Stdin => Ok(Either::Left(io::stdin().lock())),
            InputSpec::Path(p) => Ok(Either::Right(Either::Left(BufReader::new(fs::File::open(
                p,
            )?)))),
            InputSpec::Url(url_spec) => {
                let url = url_spec.url.clone();
                let resp = match accept {
                    None => url_spec.client.get(url_spec.url.as_str()),
                    Some(accept_str) => {
                        let mut headers = reqwest::header::HeaderMap::new();
                        let accept_value = HeaderValue::from_str(accept_str).map_err(|e| {
                            InputSpecError::AcceptValue {
                                str: accept_str.to_string(),
                                error: format!("{e}"),
                            }
                        })?;
                        headers.insert(ACCEPT, accept_value);
                        let client =
                            Client::builder()
                                .default_headers(headers)
                                .build()
                                .map_err(|e| InputSpecError::ClientBuilderError {
                                    error: format!("{e}"),
                                })?;
                        client.get(url_spec.url.as_str())
                    }
                }
                .send()
                .map_err(|e| InputSpecError::UrlDerefError {
                    url,
                    error: format!("{e}"),
                })?;
                let reader = BufReader::new(resp);
                Ok(Either::Right(Either::Right(reader)))
            }
        }
    }
}

impl FromStr for InputSpec {
    type Err = InputSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.starts_with("http://") => {
                let url_spec = UrlSpec::parse(s)?;
                Ok(InputSpec::Url(url_spec))
            }
            _ if s.starts_with("https://") => {
                let url_spec = UrlSpec::parse(s)?;
                Ok(InputSpec::Url(url_spec))
            }
            _ if s == "-" => Ok(InputSpec::Stdin),
            _ => {
                let pb: PathBuf =
                    PathBuf::from_str(s).map_err(|e| InputSpecError::ParsingPathError {
                        str: s.to_string(),
                        error: format!("{e}"),
                    })?;
                Ok(InputSpec::Path(pb))
            }
        }
    }
}

/// This type implements [`std::io::BufRead`].
pub type InputSpecReader =
    Either<StdinLock<'static>, Either<BufReader<fs::File>, BufReader<reqwest::blocking::Response>>>;

#[derive(Error, Debug)]
pub enum InputSpecError {
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

    #[error("Creating accept value {str} error: {error}")]
    AcceptValue { str: String, error: String },
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
        let client =
            ClientBuilder::new()
                .build()
                .map_err(|e| InputSpecError::ClientBuilderError {
                    error: format!("{e}"),
                })?;
        Ok(UrlSpec { url, client })
    }
}
