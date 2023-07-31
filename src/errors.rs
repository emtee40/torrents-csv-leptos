use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum AppError {
  #[error("Not Found")]
  NotFound,
  #[error("Internal Server Error")]
  InternalServerError,
  #[error("Couldn't parse params")]
  ParamsError,
  #[error("{error:?}")]
  APIError { error: String },
}

impl AppError {
  // pub fn status_code(&self) -> StatusCode {
  //   match self {
  //     LemmyAppError::NotFound => StatusCode::NOT_FOUND,
  //     LemmyAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
  //     Lemmy
  //   }
  // }
}

impl From<ParseIntError> for AppError {
  fn from(_value: ParseIntError) -> Self {
    Self::ParamsError
  }
}

#[cfg(not(feature = "ssr"))]
impl From<gloo_net::Error> for AppError {
  fn from(_value: gloo_net::Error) -> Self {
    Self::InternalServerError
  }
}

#[cfg(feature = "ssr")]
impl From<reqwest::Error> for AppError {
  fn from(_value: reqwest::Error) -> Self {
    Self::InternalServerError
  }
}
