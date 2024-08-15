use either::Either;
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
}
