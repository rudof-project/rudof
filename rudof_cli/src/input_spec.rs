use either::Either;
use std::{
    fs,
    io::{self, BufReader, StdinLock},
    path::{Path, PathBuf},
    str::FromStr,
};

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
    pub fn open_read(&self) -> io::Result<InputSpecReader> {
        Ok(match self {
            InputSpec::Stdin => Either::Left(io::stdin().lock()),
            InputSpec::Path(p) => Either::Right(BufReader::new(fs::File::open(p)?)),
            InputSpec::Url(_) => todo!(),
        })
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
pub type InputSpecReader = Either<StdinLock<'static>, BufReader<fs::File>>;
