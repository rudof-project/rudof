use either::Either;
use iri_s::IriS;
use reqwest::Url;
use std::{
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
    Url(String),
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
            InputSpec::Url(url) => Ok(IriS::new_unchecked(url.as_str())),
        }
    }

    // The initial version of this code was inspired by [patharg](https://github.com/jwodder/patharg/blob/edd912e865143646fd7bb4c7796aa919fa5622b3/src/lib.rs#L264)
    pub fn open_read(&self) -> Result<InputSpecReader, InputSpecError> {
        match self {
            InputSpec::Stdin => Ok(Either::Left(io::stdin().lock())),
            InputSpec::Path(p) => Ok(Either::Right(Either::Left(BufReader::new(fs::File::open(
                p,
            )?)))),
            InputSpec::Url(str) => {
                let resp = reqwest::blocking::get(str)?;
                let reader = BufReader::new(resp);
                Ok(Either::Right(Either::Right(reader)))
            }
        }
    }
}

impl FromStr for InputSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.starts_with("http://") => Ok(InputSpec::Url(s.to_string())),
            _ if s.starts_with("https://") => Ok(InputSpec::Url(s.to_string())),
            _ if s == "-" => Ok(InputSpec::Stdin),
            _ => {
                let pb: PathBuf = PathBuf::from_str(s)
                    .map_err(|e| format!("Error parsing {s} as a path: {e}"))?;
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

    #[error("Absolute path error: {path}, error: {error}")]
    AbsolutePathError { path: String, error: io::Error },
}
