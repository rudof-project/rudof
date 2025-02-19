use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum GenericIriError {
    #[error("Error parsing {str} as IRI. Error: {err}")]
    IriParse { str: String, err: String },

    #[error("Error parsing {str} using base: {base} as IRI. Error: {err}")]
    IriWithBaseParse {
        str: String,
        base: String,
        err: String,
    },

    #[error("Error resolving IRI `{other}` with base IRI `{base}`. Error: {err}")]
    IriResolve {
        base: String,
        other: String,
        err: String,
    },

    #[error("Error joining IRI `{current}` with `{str}`. Error: {err}")]
    Join {
        current: String,
        str: String,
        err: String,
    },

    #[error("Error creating reqwest HTTP client. Error: {error}")]
    ReqwestClientCreation { error: String },

    #[error("Error parsing Iri {str} as Url. Error: {error}")]
    UrlParse { str: String, error: String },

    #[error("HTTP request error: {error}")]
    Reqwest { error: String },

    #[error("HTTP request error as String: {error}")]
    ReqwestText { error: String },

    #[error("Error trying to obtain a path from file scheme Url: {url}")]
    ConvertingFileUrlToPath { url: String },

    #[error("Error reading from file {path} obtained from url {url}. Error: {error}")]
    IO {
        path: String,
        url: String,
        error: String,
    },
}
