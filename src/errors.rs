use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("parsing url: {0}")]
    UrlParse(#[from] url::ParseError),
}
