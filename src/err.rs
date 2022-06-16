use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // http error
    #[error("request url:{url} error, statusCode:{status_code}, message:{message}")]
    HttpError {
        url: String,
        status_code: StatusCode,
        message: String,
    },
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error(transparent)]
    ParselIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

// new http rest error
pub fn http_error(url: String, status_code: StatusCode, message: String) -> Error {
    Error::HttpError {
        url,
        status_code,
        message,
    }
}

impl Error {
    // 判断是否是401错误
    pub fn is_authentication_error(&self) -> bool {
        if let Error::HttpError {
            url: _,
            status_code,
            message: _,
        } = self
        {
            return status_code == &StatusCode::UNAUTHORIZED;
        }
        false
    }
}
