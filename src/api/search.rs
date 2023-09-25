use crate::{
  api::api_wrapper,
  errors::AppError,
  utils::{SearchQuery, Torrent},
};

pub async fn search(form: &SearchQuery) -> Result<Vec<Torrent>, AppError> {
  api_wrapper::<Vec<Torrent>, SearchQuery>(form).await
}
